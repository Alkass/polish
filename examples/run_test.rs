extern crate polish;

use polish::test_case::{TestCaseStatus, TestCase, run_test};
use polish::logger::{Logger};

fn main () {
    run_test(
        TestCase::new("Test Case Title", "Test Case Criteria", Box::new(|logger: &mut Logger| -> TestCaseStatus {
            logger.pass(format!("Good to go"));
            TestCaseStatus::PASSED
        }))
    );
}
