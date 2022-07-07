use crate::{stop_svc, log};

extern "C" {
    pub fn reboot(cmd: u32) -> i32;
}

pub enum Halt {
    Reboot,
    Poweroff,
    Halt,
}

impl Halt {
    pub fn operate(self, mut table: &mut Vec<String>) -> Result<(), ()> {
        let mut index: usize = table.len() - 1;
        loop {
            match stop_svc(table.get(index).unwrap_or(&"".to_string()).to_string(), &mut table) {
                Ok(_) => log::done(&format!("Stopped {}", table.get(index).unwrap_or(&"".to_string()).to_string())),
                Err(e) => log::error(&format!("Failed to stop {}: {e}", table.get(index).unwrap_or(&"".to_string()).to_string())),
            }

            if index == 0 {
                break;
            }
            index -= 1;
        }
        
        unsafe {
            let res = match self {
                Halt::Halt => reboot(0xcdef0123),
                Halt::Poweroff => reboot(0x4321fedc),
                Halt::Reboot => reboot(0x01234567),
            };

            if res == -1 {
                return Err(());
            }
        }

        Ok(())
    }
}