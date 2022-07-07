use std::{io::Write, process::Command, path::Path, fs};

use serde::{Deserialize, Serialize};

pub const INIT_DIR: &str = "/etc/init";

pub const RED: &str = "\u{001B}[31m";
pub const GREEN: &str = "\u{001B}[33m";
pub const WHITE: &str = "\u{001B}[37m";
pub const RESET: &str = "\u{001B}[0m";

pub mod log;

#[derive(Serialize, Deserialize)]
struct InitScript {
    name: String,
    script_type: String,
    depends: Vec<String>,

    start: Vec<String>,
    stop: Vec<String>,
}

pub fn run_service(json: &str) -> Result<String, String> {
    let parsed: InitScript = match serde_json::from_str(&json) {
        Ok(o) => o,
        Err(e) => return Err(format!("Failed to parse JSON: {e}")),
    };

    if Path::new(format!("/run/init/{}", &parsed.name).as_str()).exists() {
        return Ok("init_svc_running_currently".to_string());
    }

    for dep in &parsed.depends {
        let content = match fs::read_to_string(Path::new(format!("{INIT_DIR}/{dep}.json").as_str())) {
            Ok(o) => o,
            Err(e) => return Err(format!("failed to get json of dependency '{dep}': {e}")),
        };

        match run_service(content.as_str()) {
            Ok(e) => {
                if !e.eq("init_svc_running_currently") {
                    log::done(&format!("{e} has started"));
                }
            }
            Err(e) => return Err(format!("failed to run dependency '{dep}': {e}")),
        }
    }

    if (parsed.script_type.eq("oneshot"))
        || (parsed.script_type.eq("daemon"))
        || (parsed.script_type.eq("script"))
    {
        for command in &parsed.start {
            let ret = match run_cmd(&command) {
                Ok(o) => o,
                Err(e) => return Err(format!("failed to run command '{command}': {e}")),
            };

            if !ret {
                return Err(format!("Command {command} returned non-zero exit status"));
            }
        }

        #[allow(unused_must_use)]
        match create_file(
            &format!("/run/init/{}", &parsed.name),
            &format!("{}", &parsed.script_type),
        ) {
            Ok(_) => (),
            Err(e) => {
                for command in &parsed.stop {
                    run_cmd(&command);
                }
                return Err(format!(
                    "service stopped. unable to create file '/run/init/{}': {e}",
                    &parsed.name
                ));
            }
        }
    } else {
        return Err(format!("invalid init script type: {}", parsed.script_type));
    };

    Ok(parsed.name)
}

pub fn run_cmd(command: &String) -> Result<bool, String> {
    let ret = match Command::new("sh").args(["-c", &command]).status() {
        Ok(o) => o,
        Err(e) => return Err(format!("failed to execute command '{}': {e}", &command)),
    };

    if !ret.success() {
        return Ok(false);
    }

    return Ok(true);
}

fn create_file(path: &String, content: &String) -> std::io::Result<()> {
    let mut file = match fs::File::create(path) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn infinite_loop() {
    loop {
        use std::{thread, time};
        thread::sleep(time::Duration::from_secs(5));
    }
}