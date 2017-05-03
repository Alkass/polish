extern crate polish;

use polish::test_case::{TestRunner, TestCase, TestCaseStatus};
use polish::logger::Logger;

fn main() {
    TestRunner::new(0).run_test(TestCase::new("test case",
                                              "test case 2",
                                              Box::new(|_: &mut Logger| -> TestCaseStatus {
                                                           TestCaseStatus::PASSED
                                                       })));
}
