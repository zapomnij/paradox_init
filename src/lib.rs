use std::process::Command;

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