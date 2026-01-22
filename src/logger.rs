use crate::Level;

pub struct Logger {
    level: Level,
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(&self, level: Level) -> bool {
        level >= self.level
    }

    pub fn info(&self, message: &str) {
        self.log(Level::Info, message);
    }

    pub fn debug(&self, message: &str) {
        self.log(Level::Debug, message);
    }
    pub fn trace(&self, message: &str) {
        self.log(Level::Trace, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(Level::Warn, message);
    }

    pub fn error(&self, message: &str) {
        self.log(Level::Error, message);
    }

    pub fn log(&self, level: Level, message: &str) {
        if !self.enabled(level) {
            return;
        }

        eprintln!("{} {}", level.as_str(), message)
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self { level: Level::Info }
    }
}
