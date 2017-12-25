extern crate polish;

use polish::test_case::{TestRunner, TestCaseStatus, TestCase};
use polish::logger::Logger;

fn my_test_case(_: &mut Logger) -> TestCaseStatus {
    // TODO: Your test case code goes here
    TestCaseStatus::PASSED
}

fn main() {
    let test_case = TestCase::new("Test Case Title", "Test Case Criteria", Box::new(my_test_case));
    TestRunner::new().run_test(test_case);
}
