use chrono::prelude::Local;
use ansi_term::Colour::{Green, Red, Cyan, Yellow, Purple};
pub enum LogType {
    PASS,
    FAIL,
    INFO,
    WARN,
}
pub struct Logger {
    _pass: i32,
    _fail: i32,
    _warn: i32,
    _info: i32,
}
impl Logger {
    pub fn new() -> Logger {
        Logger {
            _pass: 0,
            _fail: 0,
            _warn: 0,
            _info: 0,
        }
    }
    pub fn log(&mut self, log_type: LogType, message: String) {
        let mark: _ = match log_type {
            LogType::PASS => {
                self._pass += 1;
                Green.paint("PASS")
            }
            LogType::FAIL => {
                self._fail += 1;
                Red.paint("FAIL")
            }
            LogType::INFO => {
                self._info += 1;
                Cyan.paint("INFO")
            }
            LogType::WARN => {
                self._warn += 1;
                Yellow.paint("WARN")
            }
        };
        println!("{} - {}: {}",
                 mark,
                 Purple.paint(Local::now().date().to_string()),
                 message);
    }
    pub fn pass(&mut self, message: String) {
        self.log(LogType::PASS, message);
    }
    pub fn fail(&mut self, message: String) {
        self.log(LogType::FAIL, message);
    }
    pub fn info(&mut self, message: String) {
        self.log(LogType::INFO, message);
    }
    pub fn warn(&mut self, message: String) {
        self.log(LogType::WARN, message);
    }
    pub fn get_num_pass(&self) -> i32 {
        self._pass
    }
    pub fn get_num_fail(&self) -> i32 {
        self._fail
    }
    pub fn get_num_warn(&self) -> i32 {
        self._warn
    }
    pub fn get_num_info(&self) -> i32 {
        self._info
    }
}
