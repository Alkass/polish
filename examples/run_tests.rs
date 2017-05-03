extern crate polish;

use polish::test_case::{TestRunner, TestCaseStatus, TestCase};
use polish::logger::Logger;

fn main() {
    TestRunner::get_instance(0).run_tests(vec![TestCase::new("1st Test Case Title",
                                 "1st Test Case Criteria",
                                 Box::new(|logger: &mut Logger| -> TestCaseStatus {
                                              logger.pass(format!("Good to go"));
                                              TestCaseStatus::PASSED
                                          })),
                   TestCase::new("2nd Test Case Title",
                                 "2nd Test Case Criteria",
                                 Box::new(|logger: &mut Logger| -> TestCaseStatus {
                                              logger.warn(format!("This is a warning"));
                                              TestCaseStatus::UNKNOWN
                                          })),
                   TestCase::new("3rd Test Case Title",
                                 "3rd Test Case Criteria",
                                 Box::new(|logger: &mut Logger| -> TestCaseStatus {
                                              logger.fail(format!("this is a failure"));
                                              TestCaseStatus::FAILED
                                          }))]);
}
