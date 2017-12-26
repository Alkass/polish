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

type DurationType = f64;

pub struct TestCaseResults {
    title: &'static str,
    criteria: &'static str,
    duration: DurationType,
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

type TimeUnit = (i16, &'static str);

#[allow(dead_code)]
pub struct TestRunnerTimeUnits {
    pub minutes:      TimeUnit,
    pub seconds:      TimeUnit,
    pub milliseconds: TimeUnit,
    pub microseconds: TimeUnit,
    pub nanoseconds:  TimeUnit,
}

pub static TEST_RUNNER_TIME_UNITS: TestRunnerTimeUnits = TestRunnerTimeUnits {
    minutes:      (0x1, "min"),
    seconds:      (0x2, "sec"),
    milliseconds: (0x4, "ms"),
    microseconds: (0x8, "μs"),
    nanoseconds:  (0x10, "ns"),
};

#[allow(dead_code)]
pub struct TestRunner {
    attributes: i64,
    time_unit: TimeUnit,
    results: Vec<TestCaseResults>,
    module_path: &'static str,
}
impl TestRunner {
    pub fn new() -> TestRunner {
        TestRunner {
            attributes: 0,
            time_unit: TEST_RUNNER_TIME_UNITS.nanoseconds,
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
    pub fn set_time_unit(&mut self, time_unit: TimeUnit) -> &mut Self {
        self.time_unit = time_unit;
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
        let mut duration: DurationType = (ending_time - starting_time) as DurationType;
        if self.time_unit == TEST_RUNNER_TIME_UNITS.minutes {
            duration /= 1000000000 as DurationType;
            duration /= 60 as DurationType;
        }
        else if self.time_unit == TEST_RUNNER_TIME_UNITS.seconds {
            duration /= 1000000000 as DurationType;
        }
        else if self.time_unit == TEST_RUNNER_TIME_UNITS.milliseconds {
            duration /= 1000000 as DurationType;
        }
        else if self.time_unit == TEST_RUNNER_TIME_UNITS.microseconds {
            duration /= 1000 as DurationType;
        }
        let precision = match duration {
            _ if duration < 1.0 => 2_usize,
            _ if duration < 10.0 => 1_usize,
            _ => 0_usize,
        };
        let test_info = TestCaseResults {
            title: test.title,
            criteria: test.criteria,
            duration: duration,
            status: status.clone(),
        };
        if self.module_path.len() > 0 {
            println!("{} {}::{}: {} ({:.*}{})",mark, self.module_path, test.title, formatted_criteria, precision, test_info.duration, self.time_unit.1);
        }
        else {
            println!("{} {}: {} ({:.*}{})",mark, test.title, formatted_criteria, precision, test_info.duration, self.time_unit.1);
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
            let (mut total_count, mut total_duration): (i32, DurationType) = (0, 0 as DurationType);
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
                if self.module_path.len() > 0 {
                    println!("{}", color.paint(format!("{}::{}: {} ({}{})", self.module_path, stat.title, stat.criteria, stat.duration, self.time_unit.1)));
                }
                else {
                    println!("{}", color.paint(format!("{}: {} ({}{})", stat.title, stat.criteria, stat.duration, self.time_unit.1)));
                }

            }
            if total_count == 1 {
                println!("\nRan 1 test in {}{}", total_duration, self.time_unit.1);
            }
            else {
                println!("\nRan {} tests in {}{}", total_count, total_duration, self.time_unit.1);
            }
            let formatted_pass = Green.paint(format!("{} Passed", pass));
            let formatted_failed = Red.paint(format!("{} Failed", fail));
            let formatted_skipped = Yellow.paint(format!("{} Skipped", skip));
            println!("{}  {}  {}", formatted_pass, formatted_failed, formatted_skipped);
        }
    }
}
