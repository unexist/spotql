//!
//! @package Spotql
//!
//! @file Config functions
//! @copyright (c) 2025-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv3.
//! See the file LICENSE for details.


use clap_config_file::ClapConfigFile;

#[derive(ClapConfigFile)]
#[config_file_name = "spotql"]
#[config_file_formats = "yaml,toml,json"]
pub(crate) struct Config {
    /// Set logging level LEVEL
    #[config_arg(short = 'l', name = "level", default_value = "")]
    pub(crate) loglevel: String,

    /// Print debugging messages
    #[config_arg(short = 'd', default_value = false)]
    pub(crate) debug: bool,

    /// Hostname to listen on
    #[config_arg(short = 'H', default_value = "localhost")]
    pub(crate) hostname: String,

    /// Port to listen on
    #[config_arg(short = 'p', default_value = "5432")]
    pub(crate) port: u16,
}
