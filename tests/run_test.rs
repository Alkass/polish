use proj::test_case::{TestCaseStatus, TestCase, run_test,statify};
use proj::logger::{Logger, /*LogType*/};

fn main () {
	run_test(
		TestCase::new("Test Case Title", "Test Case Criteria", Box::new(|logger: &mut Logger| -> TestCaseStatus {
			logger.info(format!("some logs"));
			logger.pass(format!("some more logs"));
			logger.warn(format!("a little more"));
			logger.fail(format!("we're done"));
			TestCaseStatus::PASSED
		}))
	);
}
