#[no_mangle]
/// Logs a message to the console.
pub extern "C" fn Log(msg: *const u8, len: usize, level: u8) {
    let message = unsafe {
        let buf = std::slice::from_raw_parts(msg, len);
        std::str::from_utf8(buf).unwrap_or_default()
    };

    match level {
        1 => log::error!("{}", message),
        2 => log::warn!("{}", message),
        3 => log::info!("{}", message),
        4 => log::debug!("{}", message),
        5 => log::trace!("{}", message),
        _ => log::info!("{}", message),
    }
}
