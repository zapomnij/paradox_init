extern "C" {
    pub fn mkfifo(path: *const u8, mode: u32) -> i32;
}