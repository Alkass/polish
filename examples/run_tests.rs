extern crate polish;

use polish::test_case::{TestRunner, TestCaseStatus, TestCase};
use polish::logger::Logger;

fn main() {
    let my_tests = vec![TestCase::new("1st Test Case Title",
                                      "1st Test Case Criteria",
                                      Box::new(|logger: &mut Logger| -> TestCaseStatus {
                                                   // TODO: Your test case goes here
                                                   TestCaseStatus::PASSED
                                               })),
                        TestCase::new("2nd Test Case Title",
                                      "2nd Test Case Criteria",
                                      Box::new(|logger: &mut Logger| -> TestCaseStatus {
                                                   // TODO: Your test case goes here
                                                   TestCaseStatus::UNKNOWN
                                               })),
                        TestCase::new("3rd Test Case Title",
                                      "3rd Test Case Criteria",
                                      Box::new(|logger: &mut Logger| -> TestCaseStatus {
                                                   // TODO: Your test case goes here
                                                   TestCaseStatus::FAILED
                                               }))];
    TestRunner::new(0).run_tests(my_tests);
}
