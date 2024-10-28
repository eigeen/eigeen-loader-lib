/// Show system message in game.
#[no_mangle]
pub extern "C" fn ShowSystemMessage(message: *const u8, len: usize, color_flag: i8) {
    unsafe {
        let buf = std::slice::from_raw_parts(message, len);
        let Ok(msg) = std::str::from_utf8(buf) else {
            return;
        };

        crate::utility::game::show_system_message(msg, color_flag);
    }
}
