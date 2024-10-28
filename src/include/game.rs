extern "C" {
    fn ShowSystemMessage(message: *const u8, len: usize, color_flag: i8);
}

pub fn show_system_message(message: &str) {
    let message_bytes = message.as_bytes();
    unsafe {
        ShowSystemMessage(message_bytes.as_ptr(), message_bytes.len(), 0);
    }
}

pub fn show_system_message_primary(message: &str) {
    let message_bytes = message.as_bytes();
    unsafe {
        ShowSystemMessage(message_bytes.as_ptr(), message_bytes.len(), 1);
    }
}
