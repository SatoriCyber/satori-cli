use std::sync::OnceLock;

use env_logger::Builder;
use log::LevelFilter;

pub static DEBUG: OnceLock<bool> = OnceLock::new();

pub fn init() {
    let mut builder = Builder::new();

    if *DEBUG.get_or_init(|| false) {
        builder.filter(None, LevelFilter::Debug);
    } else {
        builder.filter(None, LevelFilter::Info);
    }
    // Set the format for log messages (optional)
    builder.format_timestamp(None);
    builder.format_level(false);
    builder.format_target(false);

    builder.init();
}
