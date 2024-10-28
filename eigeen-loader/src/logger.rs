use std::sync::Mutex;

use colored::Colorize;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Console::{
        AllocConsole, GetConsoleWindow, GetStdHandle, SetConsoleMode, WriteConsoleW,
        ENABLE_PROCESSED_OUTPUT, ENABLE_VIRTUAL_TERMINAL_PROCESSING, ENABLE_WRAP_AT_EOL_OUTPUT,
        STD_OUTPUT_HANDLE,
    },
};

use crate::{error::Result, utility};

pub fn initialize_logging() -> Result<()> {
    unsafe {
        // alloc console
        if AllocConsole().is_err() {
            // try to get current console window
            let hwnd = GetConsoleWindow();
            if hwnd.0.is_null() {
                panic!("Failed to allocate console window");
            }
        };

        // // set console info
        // SetConsoleCP(65001)?; // utf-8
    }

    let stdout_handle: HANDLE = unsafe { GetStdHandle(STD_OUTPUT_HANDLE)? };
    unsafe {
        // enable virtual terminal processing
        SetConsoleMode(
            stdout_handle,
            ENABLE_VIRTUAL_TERMINAL_PROCESSING
                | ENABLE_PROCESSED_OUTPUT
                | ENABLE_WRAP_AT_EOL_OUTPUT,
        )?;
    };

    let logger = Logger::new(stdout_handle);

    log::set_boxed_logger(Box::new(logger)).unwrap(); // we cannot handle this error
    #[cfg(feature = "log_trace")]
    log::set_max_level(log::LevelFilter::Trace);
    #[cfg(not(feature = "log_trace"))]
    log::set_max_level(log::LevelFilter::Debug);

    Ok(())
}

pub struct Logger {
    stdout: Mutex<HANDLE>,
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

impl Drop for Logger {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(*self.stdout.lock().unwrap());
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let msg_str = format!("{}", record.args());
            // colored
            let msg_str_colored = match record.level() {
                log::Level::Error => msg_str.red().bold(),
                log::Level::Warn => msg_str.yellow(),
                log::Level::Info => msg_str.white(),
                log::Level::Debug => msg_str.dimmed(), // 浅色
                log::Level::Trace => msg_str.dimmed(), // 浅色
            };

            let now = chrono::Local::now();
            let time_str = format!("[ {} ]", now.format("%Y-%m-%d %H:%M:%S"));
            let msg = format!("{} {}\n", time_str.green(), msg_str_colored);

            let stdout = self.stdout.lock().unwrap();
            unsafe {
                let _ = WriteConsoleW(
                    *stdout,
                    &utility::string::to_wstring_bytes(&msg),
                    None,
                    None,
                );
            }
        }
    }

    fn flush(&self) {}
}

impl Logger {
    pub fn new(handle: HANDLE) -> Self {
        Logger {
            stdout: Mutex::new(handle),
        }
    }
}
