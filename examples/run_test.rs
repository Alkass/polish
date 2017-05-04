extern crate polish;

use polish::test_case::{TestRunner, TestCaseStatus, TestCase};
use polish::logger::Logger;

fn my_first_test_case (logger: &mut Logger) -> TestCaseStatus {
    logger.info(format!("10 + 20 == 30 ?"));
    if 10 + 20 == 30 {
        logger.pass(format!("10 + 20 = 30"));
        return TestCaseStatus::PASSED;
    }
    else {
        logger.fail(format!("10 + 20 != 30"));
        return TestCaseStatus::FAILED;
    }
}

fn main() {
    let mut tester_instance = TestRunner::get_instance(0);
    tester_instance.run_test(TestCase::new("My first test case", "Compare two numbers", Box::new(my_first_test_case)));
}
