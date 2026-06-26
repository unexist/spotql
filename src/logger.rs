//!
//! @package Spotql
//!
//! @file Log handling
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use log::{LevelFilter, debug};
use anyhow::Result;
use stdext::function_name;

use crate::config::Config;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    /// No log messages at all
    None,
    /// All other informational messages
    Info,
    /// Subtle related warnings
    Warnings,
    /// Subtle related errors
    Error,
    /// Debugging messages
    Debug
}

impl From<&String> for LogLevel {
    fn from(level: &String) -> Self {
        match level.to_lowercase().as_str() {
            "none" => LogLevel::None,
            "info" => LogLevel::Info,
            "warnings" => LogLevel::Warnings,
            "errors" => LogLevel::Error,
            "debug" => LogLevel::Debug,
            _ => LogLevel::Info,
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::None => LevelFilter::Off,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warnings => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Debug => LevelFilter::Debug,
        }
    }
}

/// Check config and init all log related options
///
/// # Arguments
///
/// * `config` - Config values read either from args or config file
///
/// # Returns
///
/// A `Result` with either `Unit` on success or otherwise `Error
pub(crate) fn init(config: &Config) -> Result<()> {
    let mut level = LogLevel::from(&config.loglevel);

    if config.debug {
        level = LogLevel::Debug;
    }

    let filter = LevelFilter::from(level);

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .filter_level(filter)
        .try_init()?;

    debug!("{}", function_name!());

    Ok(())
}
