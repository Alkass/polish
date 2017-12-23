extern crate polish;

use polish::test_case::{TestRunner, TestCase, TestCaseStatus, TEST_RUNNER_ATTRIBUTES as attributes};
use polish::logger::Logger;

fn main() {
    TestRunner::new()
        .set_attribute(attributes.minimize_output)
        .set_attribute(attributes.disable_final_stats)
        .set_attribute(attributes.bail_out_after_first_failure)
        .run_test(TestCase::new("title", "criteria", Box::new(|_: &mut Logger| -> TestCaseStatus {TestCaseStatus::UNKNOWN})));
}
