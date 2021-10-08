pub use log::{debug, error, info, trace, warn, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::console::Target;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;

/// Log the output of the server based on the log level set by the user specified
pub fn log_output(log_level: LevelFilter) -> Handle {
    let config = log_config(log_level);

    // println!("Initializing logging");
    match log4rs::init_config(config) {
        Ok(handle) => {
            // test_logging();
            handle
        }
        Err(e) => {
            error!("Failed to configure console logging to stdout: {}", e);
            panic!("aborting...");
        }
    }
}

/// generate a log config for a given log level
fn log_config(log_level: LevelFilter) -> Config {
    let stdout = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y%m%d_%H%M%S)} {l} {M} {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(log_level));
    match config {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to start console logging to stdout: {}", e);
            panic!("aborting...");
        }
    }
}

#[allow(dead_code)]
pub fn set_log_level(log_level: LevelFilter, handle: &Option<Handle>) {
    match handle {
        Some(handle) => {
            let config = log_config(log_level);
            handle.set_config(config);
        }
        None => {}
    }
}

#[allow(dead_code)]
//pub fn test_logging(_log_level: LevelFilter) {
/// print out log message to see what they look like
pub fn test_logging() {
    // log_output(log_level);
    info!("info1");
    warn!("warn1");
    error!("error1");
    debug!("debug1");
    trace!("trace1");
}
