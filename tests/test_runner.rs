use polish::test_case::{TestCase, TestCaseStatus, TestRunner, Testable, TEST_RUNNER_ATTRIBUTES, TEST_RUNNER_TIME_UNITS};
use polish::logger::Logger;

#[test]
fn run_test_returns_true_on_pass() {
    let tc = TestCase::new("title", "criteria", Box::new(|_: &mut Logger| TestCaseStatus::PASSED));
    let mut runner = TestRunner::new();
    assert!(runner.run_test(tc));
}

#[test]
fn run_test_returns_false_on_fail() {
    let tc = TestCase::new("title", "criteria", Box::new(|_: &mut Logger| TestCaseStatus::FAILED));
    let mut runner = TestRunner::new();
    assert!(!runner.run_test(tc));
}

#[test]
fn run_tests_bails_after_failure() {
    let tests = vec![
        TestCase::new("one", "crit", Box::new(|_: &mut Logger| TestCaseStatus::FAILED)),
        TestCase::new("two", "crit", Box::new(|_: &mut Logger| TestCaseStatus::PASSED)),
    ];
    let mut runner = TestRunner::new();
    runner.set_attribute(TEST_RUNNER_ATTRIBUTES.bail_out_after_first_failure);
    assert!(!runner.run_tests(tests));
}

#[test]
fn run_tests_from_class_example() {
    struct MyClass;
    impl Testable for MyClass {
        fn tests(self) -> Vec<TestCase> {
            vec![
                TestCase::new("t1", "crit", Box::new(|_: &mut Logger| TestCaseStatus::PASSED)),
                TestCase::new("t2", "crit", Box::new(|_: &mut Logger| TestCaseStatus::SKIPPED)),
            ]
        }
    }
    let mut runner = TestRunner::new();
    assert!(runner.run_tests_from_class(MyClass));
}

#[test]
fn test_time_unit() {
    let tc = TestCase::new("title", "criteria", Box::new(|_: &mut Logger| TestCaseStatus::PASSED));
    let mut runner = TestRunner::new();
    runner.set_time_unit(TEST_RUNNER_TIME_UNITS.seconds);
    assert!(runner.run_test(tc));
}
