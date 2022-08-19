use dotenv::dotenv;
use std::env;

/// Setup the logger based on the environment in which it's been deploy.
/// If the environment is development then the level of logging is set to Trace.
/// In any other case it's set in Warning.
///
/// # Arguments
/// * `environment` - The environment in which the application is been deploy.
pub fn setup_logger() {
  use log::LevelFilter;
  let level_filter;
  let log_level;
  let file_path;

  if cfg!(test) {
    log_level = "test".to_string();
    file_path = "/dev/null".to_string();
  } else {
    dotenv().ok();
    log_level = env::var("log_level").expect("log_level must be set");
    file_path = "logs/application.log".to_string();
  }

  match log_level.as_str() {
    "debug" => level_filter = LevelFilter::Trace,
    _ => level_filter = LevelFilter::Warn,
  }

  let (level, logger) = fern::Dispatch::new()
    .format(move |out, message, record| {
      out.finish(format_args!(
        "[{date}] [{level}][where: {target}, line: {line}] [{message}]",
        date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.3f]"),
        target = record.target(),
        level = record.level(),
        line = record.line().unwrap_or(0),
        message = message
      ))
    })
    .level(level_filter)
    .chain(std::io::stdout())
    .chain(
      fern::log_file(file_path.as_str())
        .unwrap_or_else(|_| panic!("Cannot open {}", file_path.as_str())),
    )
    .into_log();
  async_log::Logger::wrap(logger, || 0).start(level).unwrap();
}
