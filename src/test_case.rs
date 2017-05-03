use time;
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
pub struct TestRunner {
    attributes: i32,
    results: Vec<TestCaseResults>
}
impl TestRunner {
    pub fn get_instance(attributes: i32) -> TestRunner {
        TestRunner {
            attributes: attributes,
            results: vec![]
        }
    }
    pub fn run_test(&mut self, test: TestCase) {
        println!("Test: {} ({})", test.title, test.criteria);
        let mut logger: Logger = Logger::new();
        let starting_time: i32 = time::now().tm_nsec;
        let status: TestCaseStatus = (test.exec)(&mut logger);
        let ending_time: i32 = time::now().tm_nsec;
        let mark: &str = match status {
            TestCaseStatus::PASSED => "✅",
            TestCaseStatus::FAILED => "❌",
            TestCaseStatus::SKIPPED => "❗",
            TestCaseStatus::UNKNOWN => "⁉️",
        };
        println!("{} ... {}", test.criteria, mark);
        self.results.push(TestCaseResults {
                 title: test.title,
                 criteria: test.criteria,
                 duration: (ending_time - starting_time) / 1000,
                 status: status,
             })
    }
    pub fn run_tests(&mut self, tests: Vec<TestCase>) {
        for test in tests {
            self.run_test(test);
        }
    }
    pub fn run_tests_from_class<T: Testable>(&mut self, test_class: T) {
        self.run_tests(test_class.tests());
    }
    pub fn run_tests_from_classes<T: Testable>(&mut self) {
        // TODO: Implement
    }
}
impl Drop for TestRunner {
    fn drop(&mut self) {
        let (mut total_count, mut total_duration): (i32, i32) = (0, 0);
        let (mut pass, mut fail, mut skip, mut unknown): (i32, i32, i32, i32) = (0, 0, 0, 0);
        println!("\n--\n");
        for stat in self.results.iter() {
            match stat.status {
                TestCaseStatus::PASSED => pass += 1,
                TestCaseStatus::FAILED => fail += 1,
                TestCaseStatus::SKIPPED => skip += 1,
                TestCaseStatus::UNKNOWN => unknown += 1 
            }
            let formatted_message: String = match stat.status {
                TestCaseStatus::PASSED => format!("{}", Green.paint(stat.criteria)),
                TestCaseStatus::FAILED => format!("{}", Red.paint(stat.criteria)),
                TestCaseStatus::SKIPPED => format!("{}", Yellow.paint(stat.criteria)),
                TestCaseStatus::UNKNOWN => format!("{}", Yellow.paint(stat.criteria)),
            };
            total_count += 1;
            total_duration += stat.duration;
            println!("{} ({} nanosecond(s)): {}",
                stat.title,
                stat.duration,
                formatted_message);
            println!("\nRan {} Test Case(s) in {} nanosecond(s)",
                total_count,
                total_duration);
            println!("{} Passed  {} Failed  {} Skipped  {} Unknown",
                pass,
                fail,
                skip,
                unknown); 
        }
    }
}