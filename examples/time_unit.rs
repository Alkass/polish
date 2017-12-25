extern crate polish;

use polish::test_case::{TestRunner, TestCase, TestCaseStatus, TEST_RUNNER_TIME_UNITS as timeunits};
use polish::logger::Logger;

fn main() {
    TestRunner::new() // nanoseconds by default
        .run_test(TestCase::new("title", "criteria", Box::new(|_: &mut Logger| -> TestCaseStatus {TestCaseStatus::UNKNOWN})));

    TestRunner::new()
        .set_time_unit(timeunits.minutes)
        .run_test(TestCase::new("title", "criteria", Box::new(|_: &mut Logger| -> TestCaseStatus {TestCaseStatus::UNKNOWN})));

    TestRunner::new()
        .set_time_unit(timeunits.seconds)
        .run_test(TestCase::new("title", "criteria", Box::new(|_: &mut Logger| -> TestCaseStatus {TestCaseStatus::UNKNOWN})));

    TestRunner::new()
        .set_time_unit(timeunits.milliseconds)
        .run_test(TestCase::new("title", "criteria", Box::new(|_: &mut Logger| -> TestCaseStatus {TestCaseStatus::UNKNOWN})));

    TestRunner::new()
        .set_time_unit(timeunits.microseconds)
        .run_test(TestCase::new("title", "criteria", Box::new(|_: &mut Logger| -> TestCaseStatus {TestCaseStatus::UNKNOWN})));

    TestRunner::new()
        .set_time_unit(timeunits.nanoseconds)
        .run_test(TestCase::new("title", "criteria", Box::new(|_: &mut Logger| -> TestCaseStatus {TestCaseStatus::UNKNOWN})));
}
