extern crate polish;

use polish::test_case::{TestRunner, TestCaseStatus, TestCase};
use polish::logger::Logger;

fn my_test_case(logger: &mut Logger) -> TestCaseStatus {
    logger.info(format!("{} + {} = {}", 1, 2, 1 + 2));
    logger.pass(format!("{id}: {message}", id = "alkass", message = "this is a message"));
    logger.warn(format!("about to fail"));
    logger.fail(format!("failed with err_code: {code}", code = -1));
    TestCaseStatus::UNKNOWN
}

fn main() {
    let test_case = TestCase::new("Test Case Title", "Test Case Criteria", Box::new(my_test_case));
    TestRunner::new().run_test(test_case);
}
