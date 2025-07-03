use polish::logger::Logger;

#[test]
fn logger_counts_increment() {
    let mut logger = Logger::new();
    logger.pass("ok".to_string());
    logger.fail("err".to_string());
    logger.warn("warn".to_string());
    logger.info("info".to_string());
    assert_eq!(logger.get_num_pass(), 1);
    assert_eq!(logger.get_num_fail(), 1);
    assert_eq!(logger.get_num_warn(), 1);
    assert_eq!(logger.get_num_info(), 1);
}
