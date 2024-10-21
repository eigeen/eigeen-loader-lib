use crate::include;

pub struct Logger {
    prefix: String,
    max_level: log::LevelFilter,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("[{}] {}", self.prefix, record.args());
            include::logging::log(&msg, record.level());
        }
    }

    fn flush(&self) {}
}

impl Logger {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            max_level: log::LevelFilter::Debug,
        }
    }

    pub fn set_max_level(&mut self, level: log::LevelFilter) {
        self.max_level = level;
    }
}
