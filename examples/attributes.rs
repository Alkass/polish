extern crate polish;

use polish::test_case::{TestRunner, TestCase, TestCaseStatus, TEST_RUNNER_ATTRIBUTES as attributes};
use polish::logger::Logger;

fn main() {
    let mut t = TestRunner::new();
    t.set_attributes(0 as i64);
    t.set_attribute(attributes.drop_after_first_failure);
    t.set_attribute(attributes.disable_final_stats);
    t.run_test(TestCase::new("title", "criteria", Box::new(|_: &mut Logger| -> TestCaseStatus {TestCaseStatus::UNKNOWN})));
}
