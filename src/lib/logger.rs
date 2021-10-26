use log::LevelFilter;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log4rs::append::console::ConsoleAppender;
use log4rs::filter::threshold::ThresholdFilter;
use std::path::Path;

use super::config::parser::ParsedConfig;
use crate::lib::utils::io;

pub fn log_init(config: ParsedConfig) {

  let log_path_str = String::from(&config.logging.logging_path);
  let log_path = Path::new(&log_path_str);
  io::mkdir_if_none_exist(log_path);
  // NOTE Create Logger Directory if None Exist

  let log_prefix = config.logging.logging_prefix.clone();
  let log_path_combine = log_path.join(&(log_prefix));
  let log_path_combine_rolling = &(format!("{}{{}}.log", String::from(log_path_combine.to_str().unwrap())));
  let log_path_combine = &(format!("{}.log", String::from(log_path_combine.to_str().unwrap())));

  let stdout = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new("<{d(%Y-%m-%d %H:%M:%S)}> {h({l})} {t}\n    {m}{n}")))
    .build();

  let window_size = 3; // log0, log1, log2
  let fixed_window_roller = FixedWindowRoller::builder().build(log_path_combine_rolling, window_size).unwrap();

  let size_limit = 5 * 1024;
  let size_trigger = SizeTrigger::new(size_limit);

  let compound_policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

  let config = Config::builder()
    .appender(
        Appender::builder()
            .filter(Box::new(ThresholdFilter::new(LevelFilter::Debug)))
            .build(
                "logfile",
                Box::new(
                    RollingFileAppender::builder()
                        .encoder(Box::new(PatternEncoder::new("{d} {l}::{m}{n}")))
                        .build(log_path_combine.replace('_', r#""#), Box::new(compound_policy)).unwrap(),
                ),
            ),
    )
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .build(
        Root::builder()
        .appender("logfile")
        .appender("stdout")
        .build(LevelFilter::Debug),
    ).unwrap();

  log4rs::init_config(config).unwrap();
}