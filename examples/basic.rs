use sheen::Logger;

fn main() {
    let logger = Logger::with_level(sheen::Level::Trace);

    logger.info("Server started", &[("port", &3000), ("host", &"localhost")]);
}
