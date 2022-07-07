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

    unsafe {
        let buf = "/run/init/initctl".as_ptr();
        if fifo::mkfifo(buf, 420) == -1 {
            log::error(&format!("Failed to create fifo file. Starting infinite loop"));
            infinite_loop();
        }
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