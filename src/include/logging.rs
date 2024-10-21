extern "C" {
    fn Log(msg: *const u8, len: usize, level: u8);
}

pub fn log(msg: &str, level: log::Level) {
    unsafe {
        Log(msg.as_ptr(), msg.len(), level as u8);
    }
}
