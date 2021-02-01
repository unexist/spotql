///
/// @package Spotql
///
/// @file Spotql message parser
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parsers::parser_error::ParserError;
use crate::parsers::{
    startup::{ Startup, startup_parser },
    auth::{ Auth, auth_parser },
    query::{ Query, query_parser },
    terminate::{ Terminate, terminate_parser },
};

#[derive(Debug)]
pub enum Message<'a> {
    Startup(Startup<'a>),
    Auth(Auth<'a>),
    Query(Query<'a>),
    Terminate(Terminate),
}

named!(message_parser<&[u8], Message>,
    switch!(peek!(take!(1)),
        b"\0" => map!(startup_parser, |m: Startup| Message::Startup(m))
        | b"p" => map!(auth_parser, |m: Auth| Message::Auth(m))
        | b"X" => map!(terminate_parser, |m: Terminate| Message::Terminate(m))
        | _ => map!(query_parser, |m: Query| Message::Query(m))
    )
);

pub fn parse_message(input: &[u8]) -> Result<Message, ParserError> {
    match message_parser(input) {
        Ok((_, msg)) => Ok(msg),
        Err(e) => Err(ParserError {
            message: e.to_string()
        })
    }
}