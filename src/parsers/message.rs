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

use crate::parsers::parser_error::ParserError;
use crate::parsers::{
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

    /*alt(
        (
            recognize(peek(char('\0')), map(startup_parser, |m: Startup| Message::Startup(m))),
            recognize(peek(char('b')), map(auth_parser, |m: Auth| Message::Auth(m))),
            recognize(peek(char('x')), map(terminate_parser, |m: Terminate| Message::Terminate(m))),
            map(query_parser, |m: Query| Message::Query(m)),
        )
    ).parse(input)*/

pub fn parse_message(input: &[u8]) -> Result<Message<'_>, ParserError> {
    let result = match input[0] as char {
        '\0' => if let Ok(msg) = startup_parser(input) {
            Message::Startup(msg.1)
        } else {
            Message::Error("Cannot parse startup")
        },
        'b' => if let Ok(msg) = auth_parser(input) {
            Message::Auth(msg.1)
        } else {
            Message::Error("Cannot parse auth")
        },
        'x' => if let Ok(msg) = terminate_parser(input) {
            Message::Terminate(msg.1)
        } else {
            Message::Error("Cannot parse auth")
        },
        'Q' =>  if let Ok(msg) = query_parser(input) {
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
