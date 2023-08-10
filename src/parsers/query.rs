///
/// @package Spotql
///
/// @file Spotql query parser
/// @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use nom::number::Endianness;
use nom::character::complete::anychar;

use crate::parsers::statement::{ Statement, statement_parser };

#[derive(Debug)]
pub struct Query<'a> {
    pub tag: char,
    pub len: i32,
    pub statement: Option<Statement<'a>>,
}

/* Auth message: char tag | int32 len | payload | \0 */
named!(pub query_parser<&[u8], Query>,
    dbg_dmp!(
        do_parse!(
            tag: anychar >>
            len: i32!(Endianness::Big) >>
            stmt: opt!(statement_parser) >>
            (Query {
                tag: tag,
                len: len,
                statement: stmt
            })
        )
    )
);