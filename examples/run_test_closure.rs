extern crate polish;

use polish::test_case::{TestRunner, TestCaseStatus, TestCase};
use polish::logger::Logger;

fn main() {
    let test_case = TestCase::new("Test Case Title",
                                  "Test Case Criteria",
                                  Box::new(|logger: &mut Logger| -> TestCaseStatus {
                                               // TODO: Your test case code goes here
                                               TestCaseStatus::PASSED
                                           }));
    TestRunner::new(0).run_test(test_case);
}
