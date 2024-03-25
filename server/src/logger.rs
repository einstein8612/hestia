use log::LevelFilter;
use log::SetLoggerError;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::console::Target;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;


pub fn setup_logger() -> Result<(), SetLoggerError> {
    let level = log::LevelFilter::Info;
    let file_path = format!("./logs/hestia-{}.log", chrono::offset::Local::now().to_rfc3339());

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h({d} [Thread {T}] {l} - {m})}\n")))
        .target(Target::Stdout).build();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h({d} [Thread {T}] {l} - {m})}\n")))
        .build(file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                // Everything above INFO to stdout
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stdout", Box::new(stdout)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    let _handle = log4rs::init_config(config)?;
    Ok(())
}