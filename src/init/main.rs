use std::fs::File;
use std::io::{BufReader, BufRead};
use std::{fs, path::Path};

pub mod lib;
use lib::*;

fn main() {  
    if std::process::id() != 1 {
        log::error(&"Init have to be executed as PID 1".to_string());
        std::process::exit(1);
    }

    match run_cmd(&"mount -t tmpfs tmpfs /run".to_string()) {
        Err(e) => {
            log::error(&format!("failed to mount /run: {e}"));
        }
        Ok(o) => {
            if !o {
                log::error(&format!("mount command returned non-zero exit status. Failed to mount /run"));
            } else {
                log::done(&"/run has been mounted".to_string());
            }
        }
    }

    match fs::create_dir_all(Path::new("/run/init")) {
        Ok(_) => log::done(&"created init directory /run/init".to_string()),
        Err(e) => {
            log::error(&format!("Failed to create directory '/run/init': {e}"));
            log::error(&"cannot continue".to_string());
            infinite_loop()
        }
    }

    let init_file: String = format!("{INIT_DIR}/../init.list");

    let mut running: Vec<String> = Vec::new();

    let mut svcs: Vec<String> = Vec::new();
    let filebuf = match File::open(Path::new(init_file.as_str())) {
        Ok(o) => o,
        Err(e) => {
            log::init_panic(&format!("Init file not found. Check if {init_file} exists. Error: {e}"), 1);
            return;
        }
    };
    let rd = BufReader::new(filebuf);
    for line in rd.lines() {
        match line {
            Err(e) => {
                log::error(&format!("Failed to read line from init file: {e}"));
                break;
            }
            Ok(o) => svcs.push(o),
        }
    }

    for svc in &svcs {
        let svc = &format!("{}/{}.json", INIT_DIR, &svc);
        let buf = match fs::read_to_string(Path::new(svc.as_str())) {
            Ok(o) => o,
            Err(e) => {
                log::error(&format!("Failed to read from file {svc}: {e}"));
                continue;
            }
        };

        let name = match run_service(buf.as_str(), &mut running) {
            Ok(o) => o,
            Err(e) => {
                log::error(&format!("Failed to run service from file {svc}: {e}"));
                continue;
            }
        };

        log::done(&format!("{name} has started"));
    }

    match run_cmd(&"mkfifo /run/init/initctl".to_string()) {
        Ok(o) => {
            if !o {
                log::error(&"failed to create initctl. Starting infinite loop".to_string());
                infinite_loop();
            }
        }
        Err(e) => {
            log::error(&format!("failed to create initctl: {e}. Starting infinite loop"));
            infinite_loop();
        }
    }

    match fs::read_to_string(Path::new("/etc/endscript.json")) {
        Ok(o) => {
            match run_service(o.as_str(), &mut running) {
                Ok(_) => (),
                Err(e) => log::error(&format!("failed to execute init endscript: {e}")),
            }
        }
        Err(e) => log::error(&format!("failed to run init endscript: {e}")),
    }

    loop {
        match fs::read_to_string(Path::new("/run/init/initctl")) {
            Err(e) => log::error(&format!("Failed to read line from initctl: {e}")),
            Ok(o) => {
                let split: Vec<&str> = o.split_ascii_whitespace().collect();
                if split.get(0).unwrap_or(&"").to_string().eq("init_start_service") {
                    if split.get(1) == None {
                        log::error(&"Invalid command".to_string());
                        continue;
                    }

                    let json = match fs::read_to_string(Path::new(format!("{INIT_DIR}/{}.json", split.get(1).unwrap()).as_str())) {
                        Ok(o) => o,
                        Err(e) => {
                            log::error(&format!("Failed to run service {}: {e}", split.get(1).unwrap()));
                            break;
                        }
                    };

                    match run_service(&json, &mut running) {
                        Ok(o) => log::done(&format!("{o} has started")),
                        Err(e) => log::error(&format!("failed to start service: {e}")),
                    }
                }
                else if split.get(0).unwrap_or(&"").to_string().eq("init_stop_service") {
                    if split.get(1) == None {
                        log::error(&"Invalid command".to_string());
                        continue;
                    }

                    match stop_svc(split.get(1).unwrap().to_string(), &mut running) {
                        Ok(_) => log::done(&format!("{} has been stopped", split[1].clone())),
                        Err(e) => log::error(&format!("{} couldn't have been stopped: {e}", split[1].clone()))
                    }
                }
                else if split.get(0).unwrap_or(&"").to_string().eq("init_operate_halting") {
                    if split.get(1) == None {
                        log::error(&"Invalid command".to_string());
                        continue;
                    }

                    let operation;
                    if split.get(1).unwrap().to_string().eq("reboot") {
                        operation = halt::Halt::Reboot;
                    }
                    else if split.get(1).unwrap().to_string().eq("poweroff") {
                        operation = halt::Halt::Poweroff;
                    }
                    else if split.get(1).unwrap().to_string().eq("halt") {
                        operation = halt::Halt::Halt;
                    }
                    else {
                        log::error(&"Invalid command".to_string());
                        continue;
                    }

                    match operation.operate(&mut running) {
                        Ok(_) => (),
                        Err(_) => log::error(&"Failed to finish shutdown operation".to_string()),
                    }
                }
                else if split.get(0).unwrap_or(&"").to_string().eq("init_get_list") {
                    for i in &running {
                        println!("{i}");
                    }
                }
            }
        }     
    }
}
