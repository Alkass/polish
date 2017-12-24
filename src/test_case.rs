use time;
use chrono::prelude::Local;
use ansi_term::Colour;
use ansi_term::Colour::{Green, Red, Yellow};
use logger::Logger;

#[derive(PartialEq, Clone)]
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
    pub bail_out_after_first_failure: i64,
    pub disable_final_stats:          i64,
    pub minimize_output:              i64,
    // more attributes can be added here
}

// attribute values need to be assigned here if new attributes are added to TestRunnerAttributes
pub static TEST_RUNNER_ATTRIBUTES: TestRunnerAttributes = TestRunnerAttributes {
    bail_out_after_first_failure: 0x1,
    disable_final_stats:          0x2,
    minimize_output:              0x4,
};

#[allow(dead_code)]
pub struct TestRunner {
    attributes: i64,
    results: Vec<TestCaseResults>,
    module_path: &'static str,
}
impl TestRunner {
    pub fn new() -> TestRunner {
        TestRunner {
            attributes: 0,
            results: vec![],
            module_path: "",
        }
    }
    pub fn set_module_path(&mut self, path: &'static str) -> &mut Self {
        self.module_path = path;
        self
    }
    pub fn set_attribute(&mut self, attribute: i64) -> &mut Self {
        self.attributes |= attribute;
        self
    }
    pub fn set_attributes(&mut self, attributes: i64) -> &mut Self {
        self.attributes = attributes;
        self
    }
    pub fn has_attribute(&self, attribute: i64) -> bool {
        self.attributes & attribute == attribute
    }
    pub fn run_test(&mut self, test: TestCase) -> bool {
        if !self.has_attribute(TEST_RUNNER_ATTRIBUTES.minimize_output) {
            println!("Starting {} at {} on {}",
                     test.title,
                     Local::now().format("%H:%M:%S").to_string(),
                     Local::now().format("%Y-%m-%d").to_string());
        }
        let mut logger: Logger = Logger::new();
        let starting_time: i32 = time::now().tm_nsec;
        let mut status: TestCaseStatus = (test.exec)(&mut logger);
        let ending_time: i32 = time::now().tm_nsec;
        if !self.has_attribute(TEST_RUNNER_ATTRIBUTES.minimize_output) {
            println!("Ended {} at {} on {}",
                     test.title,
                     Local::now().format("%H:%M:%S").to_string(),
                     Local::now().format("%Y-%m-%d").to_string());
            logger.drop();
        }
        if status == TestCaseStatus::UNKNOWN {
            if logger.get_num_fail() > 0 {
                status = TestCaseStatus::FAILED;
            } else {
                status = TestCaseStatus::PASSED;
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
        let test_info = TestCaseResults {
            title: test.title,
            criteria: test.criteria,
            duration: ending_time - starting_time,
            status: status.clone(),
        };
        if self.module_path.len() > 0 {
            println!("{} {}::{}: {} ({}ns)", mark, self.module_path, test.title, formatted_criteria, test_info.duration);
        }
        else {
            println!("{} {}: {} ({}ns)", mark, test.title, formatted_criteria, test_info.duration);
        }
        self.results.push(test_info);
        return status == TestCaseStatus::PASSED;
    }
    pub fn run_tests(&mut self, tests: Vec<TestCase>) -> bool {
        for test in tests {
            if !self.run_test(test) && self.has_attribute(TEST_RUNNER_ATTRIBUTES.bail_out_after_first_failure) {
                return false;
            }
        }
        return true;
    }
    pub fn run_tests_from_class<T: Testable>(&mut self, test_class: T) -> bool {
        return self.run_tests(test_class.tests());
    }
}
impl Drop for TestRunner {
    fn drop(&mut self) {
        if !self.has_attribute(TEST_RUNNER_ATTRIBUTES.disable_final_stats) {
            let (mut total_count, mut total_duration): (i32, i32) = (0, 0);
            let (mut pass, mut fail, mut skip): (i32, i32, i32) = (0, 0, 0);
            print!("\n");
            for stat in self.results.iter() {
                let color: Colour;
                match stat.status {
                    TestCaseStatus::PASSED => {
                        pass += 1;
                        color = Green;
                    },
                    TestCaseStatus::FAILED => {
                        fail += 1;
                        color = Red;
                    },
                    TestCaseStatus::SKIPPED => {
                        skip += 1;
                        color = Yellow;
                    },
                    _ => {
                        color = Yellow;
                    }
                };
                total_count += 1;
                total_duration += stat.duration;
                let formatted_text = color.paint(format!("{} ({}) ... {}ns",
                                                         stat.title,
                                                         stat.criteria,
                                                         stat.duration));
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
