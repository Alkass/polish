extern crate polish;

use polish::test_case::{TestRunner, TestCaseStatus, TestCase, Testable};
use polish::logger::Logger;

fn main() {
    struct MyTestCase;
    impl Testable for MyTestCase {
        fn tests(self) -> Vec<TestCase> {
            vec![TestCase::new("Some Title #1",
                               "Testing Criteria",
                               Box::new(|_: &mut Logger| -> TestCaseStatus {
                                            TestCaseStatus::PASSED
                                        })),
                 TestCase::new("Some Title #2",
                               "Testing Criteria",
                               Box::new(|_: &mut Logger| -> TestCaseStatus {
                                            TestCaseStatus::SKIPPED
                                        }))]
        }
    }
    TestRunner::new(0).run_tests_from_class(MyTestCase {});
}
