extern crate polish;

use polish::test_case::{TestRunner, TestCaseStatus, TestCase, Testable, statify};
use polish::logger::Logger;

fn main() {
    struct MyTestCase;
    impl Testable for MyTestCase {
        fn tests(self) -> Vec<TestCase> {
            vec![TestCase::new("Some Title #1",
                               "Testing Criteria",
                               Box::new(|logger: &mut Logger| -> TestCaseStatus {
                                            logger.pass(format!("Good to go"));
                                            TestCaseStatus::PASSED
                                        })),
                 TestCase::new("Some Title #2",
                               "Testing Criteria",
                               Box::new(|logger: &mut Logger| -> TestCaseStatus {
                                            logger.info(format!("Skipping this one"));
                                            TestCaseStatus::SKIPPED
                                        }))]
        }
    }
    let res = TestRunner::get_instance(0).run_tests_from_class(MyTestCase {});
    statify(&res);
}
