pub mod lib;

use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        lib::log::error(&"Not enough arguments".to_string());
        exit(1);
    }

    if args[1].eq("start") {
        if args.len() < 3 {
            lib::log::error(&"Not enough arguments".to_string());
            exit(1);
        }

        match lib::Command::StartService(args[2].clone()).operate() {
            Err(e) => {
                lib::log::error(&format!("Failed to start service {}: {e}", args[2]));
                exit(1);
            }
            Ok(_) => lib::log::done(&format!("{} has started", args[2])),
        }
    } else if args[1].eq("stop") {
        if args.len() < 3 {
            lib::log::error(&"Not enough arguments".to_string());
            exit(1);
        }

        match lib::Command::StopService(args[2].clone()).operate() {
            Err(e) => {
                lib::log::error(&format!("Failed to stop service {}: {e}", args[2]));
                exit(1);
            }
            Ok(_) => lib::log::done(&format!("{} has stopped", args[2])),
        }
    } else if args[1].eq("restart") {
        if args.len() < 3 {
            lib::log::error(&"Not enough arguments".to_string());
            exit(1);
        }
        
        match lib::Command::StopService(args[2].clone()).operate() {
            Err(e) => {
                lib::log::error(&format!("Failed to stop service {}: {e}", args[2]));
                exit(1);
            }
            Ok(_) => lib::log::done(&format!("{} has stopped", args[2])),
        }

        match lib::Command::StartService(args[2].clone()).operate() {
            Err(e) => {
                lib::log::error(&format!("Failed to start service {}: {e}", args[2]));
                exit(1);
            }
            Ok(_) => lib::log::done(&format!("{} has started", args[2])),
        }
    } else if args[1].eq("enable") {
        if args.len() < 3 {
            lib::log::error(&"Not enough arguments".to_string());
            exit(1);
        }

        match lib::Command::EnableService(args[2].clone()).operate() {
            Err(e) => {
                lib::log::error(&format!("Failed to enable service {}: {e}", args[2]));
                exit(1);
            }
            Ok(_) => lib::log::done(&format!("{} has been enabled since now", args[2])),
        }
    } else if args[1].eq("disable") {
        if args.len() < 3 {
            lib::log::error(&"Not enough arguments".to_string());
            exit(1);
        }

        match lib::Command::DisableService(args[2].clone()).operate() {
            Err(e) => {
                lib::log::error(&format!("Failed to disable service {}: {e}", args[2]));
                exit(1);
            }
            Ok(_) => lib::log::done(&format!("{} has been disabled since now", args[2])),
        }
    } else if args[1].eq("send") {
        if args.len() < 3 {
            lib::log::error(&"Not enough arguments".to_string());
            exit(1);
        }

        let mut argv: Vec<String> = Vec::new();

        for item in &args[3..] {
            argv.push(item.clone());
        }

        match lib::Command::Initctl(args[2].clone(), argv).operate() {
            Ok(_) => (),
            Err(e) => {
                lib::log::error(&format!("Failed to send commands to init: {e}"));
                exit(1);
            }
        }
    }
}