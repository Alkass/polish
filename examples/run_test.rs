extern crate polish;

use polish::test_case::{TestRunner, TestCaseStatus, TestCase};
use polish::logger::Logger;

fn main() {
    TestRunner::get_instance(0)
        .run_test(TestCase::new("Test Case Title",
                                "Test Case Criteria",
                                Box::new(|logger: &mut Logger| -> TestCaseStatus {
                                             logger.pass(format!("Good to go"));
                                             TestCaseStatus::PASSED
                                         })));
}
