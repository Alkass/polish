use time;
use chrono::prelude::Local;
use ansi_term::Colour::{Green, Red, Yellow};
use logger::Logger;

pub enum TestCaseStatus {
    PASSED,
    FAILED,
    SKIPPED,
    UNKNOWN,
}

pub struct TestCase {
    pub title: &'static str,
    pub criteria: &'static str,
    pub exec: Box<Fn(&mut Logger) -> TestCaseStatus>,
}
impl TestCase {
    pub fn new(title: &'static str,
               criteria: &'static str,
               exec: Box<Fn(&mut Logger) -> TestCaseStatus>)
               -> TestCase {
        TestCase {
            title: title,
            criteria: criteria,
            exec: exec,
        }
    }
}

pub struct TestCaseResults {
    title: &'static str,
    criteria: &'static str,
    duration: i32,
    status: TestCaseStatus,
}

pub trait Testable {
    fn tests(self) -> Vec<TestCase>;
}

#[allow(dead_code)]
pub struct TestRunnerAttributes {
    pub drop_after_first_failure: i64,
    pub disable_final_stats: i64
    // more attributes can be added here
}

pub static TEST_RUNNER_ATTRIBUTES: TestRunnerAttributes = TestRunnerAttributes {
    drop_after_first_failure: 0x0000000000000001,
    disable_final_stats: 0x0000000000000002
        // attribute values need to be assigned here if new attributes are added to TestRunnerAttributes
};

#[allow(dead_code)]
pub struct TestRunner {
    attributes: i64,
    results: Vec<TestCaseResults>,
}
impl TestRunner {
    pub fn new() -> TestRunner {
        TestRunner {
            attributes: 0,
            results: vec![],
        }
    }
    pub fn set_attribute(&mut self, attribute: i64) {

        self.attributes |= attribute;
    }
    pub fn set_attributes(&mut self, attributes: i64) {
        self.attributes = attributes;
    }
    pub fn has_attribute(&self, attribute: i64) -> bool {
        self.attributes & attribute == attribute
    }
    pub fn run_test(&mut self, test: TestCase) -> bool {
        println!("Starting {} at {} on {}",
                 test.title,
                 Local::now().format("%H:%M:%S").to_string(),
                 Local::now().format("%Y-%m-%d").to_string());
        let mut logger: Logger = Logger::new();
        let starting_time: i32 = time::now().tm_nsec;
        let mut status: TestCaseStatus = (test.exec)(&mut logger);
        let ending_time: i32 = time::now().tm_nsec;
        println!("Ended {} at {} on {}",
                 test.title,
                 Local::now().format("%H:%M:%S").to_string(),
                 Local::now().format("%Y-%m-%d").to_string());
        match status {
            TestCaseStatus::PASSED => {}
            TestCaseStatus::FAILED => {}
            TestCaseStatus::SKIPPED => {}
            TestCaseStatus::UNKNOWN => {
                if logger.get_num_fail() > 0 {
                    status = TestCaseStatus::FAILED;
                } else {
                    status = TestCaseStatus::PASSED;
                }
            }
        }
        let mark = match status {
            TestCaseStatus::PASSED => "✅",
            TestCaseStatus::FAILED => "❌",
            TestCaseStatus::SKIPPED => "❗",
            TestCaseStatus::UNKNOWN => "",
        };
        let formatted_criteria = match status {
            TestCaseStatus::PASSED => Green.paint(test.criteria),
            TestCaseStatus::FAILED => Red.paint(test.criteria),
            TestCaseStatus::SKIPPED => Yellow.paint(test.criteria),
            TestCaseStatus::UNKNOWN => Yellow.paint(test.criteria),
        };
        println!("{} ... {}", formatted_criteria, mark);
        let test_info: TestCaseResults = TestCaseResults {
            title: test.title,
            criteria: test.criteria,
            duration: (ending_time - starting_time) / 1000,
            status: status,
        };
        self.results.push(test_info);
        return true; // TODO: implement
    }
    pub fn run_tests(&mut self, tests: Vec<TestCase>) -> bool {
        for test in tests {
            if !self.run_test(test) {
                return false;
            }
        }
        return true;
    }
    pub fn run_tests_from_class<T: Testable>(&mut self, test_class: T) -> bool {
        return self.run_tests(test_class.tests());
    }
    /*pub fn run_tests_from_classes<T: Testable>(&mut self) {
        // TODO: Implement
    }*/
}
impl Drop for TestRunner {
    fn drop(&mut self) {
        if !self.has_attribute(TEST_RUNNER_ATTRIBUTES.disable_final_stats) {
            let (mut total_count, mut total_duration): (i32, i32) = (0, 0);
            let (mut pass, mut fail, mut skip): (i32, i32, i32) = (0, 0, 0);
            print!("\n");
            for stat in self.results.iter() {
                match stat.status {
                    TestCaseStatus::PASSED => pass += 1,
                    TestCaseStatus::FAILED => fail += 1,
                    TestCaseStatus::SKIPPED => skip += 1,
                    _ => {}
                }
                let color = match stat.status {
                    TestCaseStatus::PASSED => Green,
                    TestCaseStatus::FAILED => Red,
                    TestCaseStatus::SKIPPED => Yellow,
                    _ => Yellow,
                };
                total_count += 1;
                total_duration += stat.duration;
                let formatted_text =
                    color.paint(format!("{} ({}) ... {}ns", stat.title, stat.criteria, stat.duration));
                println!("{}", formatted_text);
            }
            println!("\nRan {} test(s) in {}ns", total_count, total_duration);
            let formatted_pass = Green.paint(format!("{} Passed", pass));
            let formatted_failed = Red.paint(format!("{} Failed", fail));
            let formatted_skipped = Yellow.paint(format!("{} Skipped", skip));
            println!("{}  {}  {}",
                     formatted_pass,
                     formatted_failed,
                     formatted_skipped);
            }
        }
}
