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
        let mut index: isize = table.len() as isize - 1;
        while index != -1 {
            match stop_svc(table.get(index as usize).unwrap_or(&"".to_string()).to_string(), &mut table) {
                Ok(_) => log::done(&format!("Stopped {}", table.get(index as usize).unwrap_or(&"".to_string()))),
                Err(e) => log::error(&format!("Failed to stop {}: {e}", table.get(index as usize).unwrap_or(&"".to_string()))),
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