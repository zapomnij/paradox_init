use std::path::Path;
pub mod log;

pub const INIT_DIR: &str = "/etc/init";

pub const RED: &str = "\u{001B}[31m";
pub const GREEN: &str = "\u{001B}[33m";
pub const WHITE: &str = "\u{001B}[37m";
pub const RESET: &str = "\u{001B}[0m";

pub enum Command {
    Initctl(String, Vec<String>),

    StartService(String),
    StopService(String),
    RestartService(String),

    EnableService(String),
    DisableService(String)
}

pub enum Success {
    CommandSended,
    CommandsSended,

    EnabledService,
    DisabledService,
}

pub mod error;
mod init_communication;
impl Command {
    pub fn operate(self) -> Result<Success, error::Error> {
        match self {
            Command::StartService(name) => return init_communication::initctl("init_start_service".to_string(), vec![name]),
            Command::StopService(name) => return init_communication::initctl("init_stop_service".to_string(), vec![name]),
            Command::RestartService(name) => {
                match init_communication::initctl("init_stop_service".to_string(), vec![name.clone()]) {
                    Ok(_) => (),
                    Err(e) => return Err(error::Error::OperationFailed(format!("Failed to stop {name}: {e}"))),
                }

                match init_communication::initctl("init_start_service".to_string(), vec![name.clone()]) {
                    Ok(_) => Ok(Success::CommandsSended),
                    Err(e) => return Err(error::Error::OperationFailed(format!("Failed to start {name}: {e}"))),
                }
            }

            Command::EnableService(name) => {
                if !Path::new(format!("{}/{name}.json", INIT_DIR).as_str()).exists() {
                    return Err(error::Error::ServiceNotFound((name, "No such file or directory".to_string())));
                } else if !Path::new(format!("{}/../init.list", INIT_DIR).as_str()).exists() {
                    return Err(error::Error::InitListNotFound("No such file or directory".to_string()));
                }

                match std::fs::write(Path::new(format!("{}/{name}.json", INIT_DIR).as_str()), format!("\n{name}")) {
                    Err(e) => return Err(error::Error::Other(format!("Cannot write to init list: {e}"))),
                    Ok(_) => Ok(Success::EnabledService),
                }
            }
            Command::DisableService(name) => {
                if !Path::new(format!("{}/{name}.json", INIT_DIR).as_str()).exists() {
                    return Err(error::Error::ServiceNotFound((name, "No such file or directory".to_string())));
                } else if !Path::new(format!("{}/../init.list", INIT_DIR).as_str()).exists() {
                    return Err(error::Error::InitListNotFound("No such file or directory".to_string()));
                }

                let mut buf: Vec<String> = Vec::new();
                let filebuf = match std::fs::File::open(Path::new(format!("{INIT_DIR}/../init.list").as_str())) {
                    Ok(o) => o,
                    Err(e) => return Err(error::Error::Other(format!("Cannot read from init list: {e}"))),
                };
                let rd = std::io::BufReader::new(filebuf);
                for line in std::io::BufRead::lines(rd) {
                    match line {
                        Err(e) => return Err(error::Error::Other(format!("Cannot read from init list: {e}"))),
                        Ok(o) => buf.push(o),
                    }
                }

                for (ind, it) in buf.iter().enumerate() {
                    if it.eq(&name) {
                        buf.remove(ind);
                        break;
                    }
                }

                for (ind, item) in buf.iter().enumerate() {
                    let content = if ind == buf.len() {
                        item.clone()
                    } else {
                        format!("{item}\n")
                    };
                    match std::fs::write(Path::new(format!("{}/../init.list", INIT_DIR).as_str()), content) {
                        Ok(_) => (),
                        Err(e) => return Err(error::Error::Other(format!("Failed to write to init list: {e}"))),
                    }
                }

                Ok(Success::DisabledService)
            }
            Command::Initctl(cmd, argv) => return init_communication::initctl(cmd, argv),
        }
    }
}