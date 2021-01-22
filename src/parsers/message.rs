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
    regular::{ Regular, regular_parser },
};

#[derive(Debug)]
pub enum Message<'a> {
    Startup(Startup<'a>),
    Auth(Auth<'a>),
    Regular(Regular<'a>),
}

named!(message_parser<&[u8], Message>,
    switch!(peek!(take!(1)),
        b"\0" => map!(startup_parser, |m: Startup| Message::Startup(m))
        | b"p" => map!(auth_parser, |m: Auth| Message::Auth(m))
        | _ => map!(regular_parser, |m: Regular| Message::Regular(m))
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
