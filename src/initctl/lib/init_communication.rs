use crate::lib::{error, Success};

pub fn initctl(command: String, argv: Vec<String>) -> Result<Success, error::Error> {
    let mut args = String::new();
    for i in argv {
        args.push_str(i.as_str());
        args.push(' ');
    }

    if !std::path::Path::new("/run/init/initctl").exists() {
        return Err(error::Error::InitctlNotFound("No such file or directory".to_string()));
    }

    match std::fs::write(std::path::Path::new("/run/init/initctl"), format!("{command} {args}")) {
        Ok(_) => (),
        Err(e) => return Err(error::Error::InitctlNotFound(format!("{e}"))),
    }

    Ok(Success::CommandSended)
}