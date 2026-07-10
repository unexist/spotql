//!
//! @package Spotql
//!
//! @file Spotql message parser
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use log::{Level, debug, log_enabled};
use stdext::function_name;

use crate::parsers::parser_error::ParserError;
use crate::parsers::incoming::{
    startup::{Startup, startup_parser},
    auth::{Auth, auth_parser},
    query::{Query, query_parser},
    terminate::{Terminate, terminate_parser},
};

#[derive(Debug)]
pub enum Message<'a> {
    Startup(Startup<'a>),
    Auth(Auth<'a>),
    Query(Query<'a>),
    Terminate(Terminate),
    Error(&'static str),
}

pub fn parse_message(input: &[u8]) -> Result<Message<'_>, ParserError> {
    if log_enabled!(Level::Debug) {
        let utf8_string = String::from_utf8_lossy(input);

        debug!("{}: input={:?}", function_name!(), utf8_string);
    }

    let result = match input[0] as char {
        '\0' => if let Ok(msg) = startup_parser(input) {
            Message::Startup(msg.1)
        } else {
            Message::Error("Cannot parse startup")
        },
        'p' => if let Ok(msg) = auth_parser(input) {
            Message::Auth(msg.1)
        } else {
            Message::Error("Cannot parse auth")
        },
        'x' => if let Ok(msg) = terminate_parser(input) {
            Message::Terminate(msg.1)
        } else {
            Message::Error("Cannot parse auth")
        },
        'Q' => if let Ok(msg) = query_parser(input) {
            Message::Query(msg.1)
        } else {
            Message::Error("Cannot parse query")
        },
        _ => Message::Error("Message type not implemented yet")
    };

    match result {
        Message::Error(e) => Err(ParserError {
            message: e.to_string()
        }),
        msg => Ok(msg),
    }
}
