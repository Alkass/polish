extern crate polish;

use polish::test_case::{TestRunner, TestCaseStatus, TestCase};
use polish::logger::Logger;

fn my_test_case(logger: &mut Logger) -> TestCaseStatus {
    TestCaseStatus::PASSED
}

fn main() {
    let test_case = TestCase::new("Test Case Title",
                                  "Test Case Criteria",
                                  Box::new(my_test_case));
    TestRunner::new(0).run_test(test_case);
}
