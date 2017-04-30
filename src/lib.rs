
extern crate chrono;
extern crate ansi_term;

pub mod logger {
    use chrono::prelude::Local;
    use ansi_term::Colour::{Green, Red, Cyan, Yellow, Purple};
    pub enum LogType {
        PASS,
        FAIL,
        INFO,
        WARN
    }
    pub struct Logger {
        _pass: i32,
        _fail: i32,
        _warn: i32,
        _info: i32
    }
    impl Logger {
        pub fn new () -> Logger {
            Logger {
                _pass: 0,
                _fail: 0,
                _warn: 0,
                _info: 0
            }
        }
        pub fn log (&mut self, log_type: LogType, message: String) {
            let mark: _ = match log_type {
                LogType::PASS => {
                    self._pass += 1;
                    Green.paint("PASS")
                },
                LogType::FAIL => {
                    self._fail += 1;
                    Red.paint("FAIL")
                },
                LogType::INFO => {
                    self._info += 1;
                    Cyan.paint("INFO")
                },
                LogType::WARN => {
                    self._warn += 1;
                    Yellow.paint("WARN")
                }
            };
            println!("{} - {}: {}", mark, Purple.paint(Local::now().date().to_string()), message);
        }
        pub fn pass (&mut self, message: String) {
            self.log(LogType::PASS, message);
        }
        pub fn fail (&mut self, message: String) {
            self.log(LogType::FAIL, message);
        }
        pub fn info (&mut self, message: String) {
            self.log(LogType::INFO, message);
        }
        pub fn warn (&mut self, message: String) {
            self.log(LogType::WARN, message);
        }
        pub fn get_num_pass (&self) -> i32 {
            self._pass
        }
        pub fn get_num_fail (&self) -> i32 {
            self._fail
        }
        pub fn get_num_warn (&self) -> i32 {
            self._warn
        }
        pub fn get_num_info (&self) -> i32 {
            self._info
        }
    }
}

pub mod test_case {
    use logger::Logger;
    pub enum TestCaseStatus {
        PASSED,
        FAILED,
        SKIPPED,
        UNKNOWN
    }
    pub struct TestCase {
        pub title: &'static str,
        pub criteria: &'static str,
        pub exec: Box<Fn (&mut Logger) -> TestCaseStatus>
    }
    impl TestCase {
        pub fn new (title: &'static str, criteria: &'static str, exec: Box<Fn (&mut Logger) -> TestCaseStatus>) -> TestCase {
            TestCase {
                title: title,
                criteria: criteria,
                exec: exec
            }
        }
    }
    pub struct TestCaseResults {
        title: &'static str,
        criteria: &'static str,
        duration: f32,
        status: TestCaseStatus
    }
    pub fn run_test (test: TestCase) -> Vec<TestCaseResults> {
        println!("Test: {} ({})", test.title, test.criteria);
        let mut logger: Logger = Logger::new();
        let starting_time: f32 = 0 as f32; // TODO: get starting time
        let status: TestCaseStatus = (test.exec)(&mut logger);
        // TODO: get logger stats
        let ending_time: f32 =  0 as f32; // TODO: get ending time
        println!("{} PASS  {} FAIL  {} WARN  {} INFO",
            logger.get_num_pass(),
            logger.get_num_fail(),
            logger.get_num_warn(),
            logger.get_num_info()
        );
        let mark: &str = match status {
            TestCaseStatus::PASSED  => "✅",
            TestCaseStatus::FAILED  => "❌",
            TestCaseStatus::SKIPPED => "❗",
            TestCaseStatus::UNKNOWN => "⁉️",
        };
        println!("{} ... {}", test.criteria, mark);
        vec![TestCaseResults {
            title: test.title,
            criteria: test.criteria,
            duration: ending_time - starting_time,
            status: status
        }]
    }
    pub fn run_tests (tests: Vec<TestCase>) -> Vec<TestCaseResults> {
        let mut results: Vec<TestCaseResults> = vec![];
        for test in tests {
            for item in run_test(test) {
                results.push(item);
            }
        }
        return results;
    }
    pub trait Testable {
        fn tests (self) -> Vec<TestCase>;
    }
    pub fn run_tests_from_class <T: Testable> (test_class: T) -> Vec<TestCaseResults> {
        return run_tests(test_class.tests());
    }
    pub fn run_tests_from_classes <T: Testable> () -> Vec<TestCaseResults> {
        // TODO: Implement
        return Vec::new();
    }
    pub fn statify (stats: &Vec<TestCaseResults>) -> bool {
        let (mut total_count, mut total_duration): (i32, f32) = (0, 0 as f32);
        let (mut pass, mut fail, mut skip, mut unknown): (i32, i32, i32, i32) = (0, 0, 0, 0);
        println!("\n---\n");
        for stat in stats.iter() {
            match stat.status {
                TestCaseStatus::PASSED  => pass += 1,
                TestCaseStatus::FAILED  => fail += 1,
                TestCaseStatus::SKIPPED => skip += 1,
                TestCaseStatus::UNKNOWN => unknown += 1
            }
            let formatted_message: String = match stat.status {
                TestCaseStatus::PASSED  => format!("{}", stat.criteria),
                TestCaseStatus::FAILED  => format!("{}", stat.criteria),
                TestCaseStatus::SKIPPED => format!("{}", stat.criteria),
                TestCaseStatus::UNKNOWN => format!("{}", stat.criteria)
            };
            total_count += 1;
            total_duration += stat.duration;
            println!("{} ({}s): {}", stat.title, stat.duration, formatted_message);
        }
        println!("\nRan {} Test Case(s) in {} Second(s)", total_count, total_duration);
        println!("{} Passed  {} Failed  {} Skipped  {} Unknown", pass, fail, skip, unknown);
        return (fail + unknown) > 0;
    }
}
