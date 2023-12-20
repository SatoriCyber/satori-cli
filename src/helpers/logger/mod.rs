use env_logger::Builder;
use log::LevelFilter;

pub fn init(debug: bool) {
    let mut builder = Builder::new();

    if debug {
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
