use std::fs::File;
use std::io::{BufReader, BufRead};
use std::{fs, path::Path};
use paradox_init::*;

fn main() {  
    let init_file: String = format!("{INIT_DIR}/../init.list");

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

        let name = match run_service(buf.as_str()) {
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
            match run_service(o.as_str()) {
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
                }
            }
        }     
    }
}
