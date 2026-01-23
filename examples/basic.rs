fn main() {
    let logger = sheen::Logger::new().level(sheen::Level::Trace);

    let req_log = logger.with(&[("request_id", &"abc123")]);

    req_log.info("started", &[]);
    req_log.info("db query", &[("table", &"users")]);
    req_log.error("failed", &[("code", &500)]);
}
