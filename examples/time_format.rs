use sheen::{Level, Logger};

fn main() {
    sheen::init_with(
        Logger::new()
            .level(Level::Trace)
            .time_format("%Y-%m-%d %H:%M:%S"),
    );

    sheen::trace!("starting up");
    sheen::debug!("loading configuration");
    sheen::info!("server listening on port 3000");
    sheen::warn!("cache is nearly full");
    sheen::error!("failed to connect to database");
}
