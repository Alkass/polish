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
    fn log(&mut self, log_type: LogType, message: String) {
        let mark = match log_type {
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
        println!("{} {}: {}",
                 Purple.paint(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                 mark,
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
impl Drop for Logger {
    fn drop(&mut self) {
        let formatted_pass = format!("{} {}", self.get_num_pass(), Green.paint("PASS"));
        let formatted_fail = format!("{} {}", self.get_num_fail(), Red.paint("FAIL"));
        let formatted_warn = format!("{} {}", self.get_num_warn(), Yellow.paint("WARN"));
        let formatted_info = format!("{} {}", self.get_num_info(), Cyan.paint("INFO"));
        println!("{}  {}  {}  {}",
                 formatted_pass,
                 formatted_fail,
                 formatted_warn,
                 formatted_info);
    }
}
