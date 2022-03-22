use crate::{
  app::{DEFAULT_LOGFILE_LEVEL, DEFAULT_LOG_LEVEL, LOG_FILE_PATH},
  util::file_exists,
};
use log::{debug, LevelFilter, SetLoggerError};
use log4rs::{
  append::{
    console::{ConsoleAppender, Target},
    file::FileAppender,
  },
  config::{Appender, Config, Root},
  encode::pattern::PatternEncoder,
  filter::threshold::ThresholdFilter,
};
use std::fs;

// default stdout logger
// pub fn init_log() {
//   // trace show all, can enable log levels per module ex `RUST_LOG="warn,test::foo=info,test::foo::bar=debug"`
//   let rust_log = env::var("RUST_LOG").unwrap_or(DEFAULT_RUST_LOG.to_string());
//   std::env::set_var("RUST_LOG", rust_log);
//   // // init env logger before anything else and in main
//   env_logger::init();
// }

// log4rs logger :shared with c3-updater and actixweb4-starter
pub fn init_log4rs() -> Result<(), SetLoggerError> {
  let default_log_level = std::env::var("LOG_LEVEL").unwrap_or(DEFAULT_LOG_LEVEL.to_string());
  let default_logfile_level = std::env::var("LOGFILE_LEVEL").unwrap_or(DEFAULT_LOGFILE_LEVEL.to_string());
  // debug!("log env DEFAULT_LOG_LEVEL: '{}', DEFAULT_LOGFILE_LEVEL: '{}'", log_default_level, logfile_default_level);
  // closure to get LevelFilter from env string
  let get_log_level = |env_level: String| -> LevelFilter {
    match env_level.to_uppercase().as_str() {
      "OFF" => LevelFilter::Off,
      "ERROR" => LevelFilter::Error,
      "WARN" => LevelFilter::Warn,
      "INFO" => LevelFilter::Info,
      "DEBUG" => LevelFilter::Debug,
      "TRACE" => LevelFilter::Trace,
      _ => LevelFilter::Error,
    }
  };
  let log_level = get_log_level(default_log_level);
  let logfile_level = get_log_level(default_logfile_level);
  // always delete old log
  if file_exists(LOG_FILE_PATH) {
    debug!("removing old log file:{}", LOG_FILE_PATH);
    fs::remove_file(LOG_FILE_PATH).unwrap();
  };
  // Build a stderr logger.
  let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
  // Logging to log file.
  let logfile = FileAppender::builder()
    // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
    .encoder(Box::new(PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S)}] {l} - {m}\n")))
    .build(LOG_FILE_PATH)
    .unwrap();
  // Log Trace level output to file where trace is the default level
  // and the programmatically specified level to stderr.
  let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .appender(Appender::builder().filter(Box::new(ThresholdFilter::new(log_level))).build("stderr", Box::new(stderr)))
    .build(Root::builder().appender("logfile").appender("stderr").build(logfile_level))
    .unwrap();

  // Use this to change log levels at runtime.
  // This means you can change the default log level to trace
  // if you are trying to debug an issue and need more logs on then turn it off
  // once you are done.
  let _handle = log4rs::init_config(config)?;

  // error!("Goes to stderr and file");
  // warn!("Goes to stderr and file");
  // info!("Goes to stderr and file");
  // debug!("Goes to file only");
  // trace!("Goes to file only");
  // debug!("current log level: '{:?}'", log_level);

  Ok(())
}
