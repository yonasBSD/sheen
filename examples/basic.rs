use sheen::Logger;

fn main() {
    let logger = Logger::with_level(sheen::Level::Trace);

    logger.trace("this is trace");
    logger.debug("this is debug");
    logger.info("this is info");
    logger.warn("this is warn");
    logger.error("this is error");
}
