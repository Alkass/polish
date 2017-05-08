extern crate polish;

use polish::test_case::{TestRunner, TEST_RUNNER_ATTRIBUTES as attributes};

fn main() {
    let mut t = TestRunner::new();
    t.set_attribute(attributes.drop_after_first_failure);
    t.set_attributes(attributes.drop_after_first_failure);
}
