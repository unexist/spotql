//!
//! @package Spotql
//!
//! @file Spotql query parser
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use nom::bytes::tag;
use nom::combinator::{complete, map};
use nom::multi::many0;
use nom::IResult;
use nom::Parser;

use crate::parsers::incoming::expression::expression_parser;
use crate::parsers::incoming::ws::{btag, ws};

/* Case statement: case expr when expr then expr end */
pub(crate) fn unsupported_case_parser(input: &[u8]) -> IResult<&[u8], bool> {
    map(
        (
            tag("case"),
            expression_parser,
            complete(
                many0(
                    complete(
                        (
                            ws(btag("when")),
                            expression_parser,
                            ws(btag("then")),
                            expression_parser
                        )
                    )
                )
            ),
            tag("end"),
        ),
        |(case_start, expression, when_then, case_end)| {
            println!("{:?}", when_then);
            true
        }
    ).parse(input)
}

pub(crate) fn unsupported_parser(input: &[u8]) -> IResult<&[u8], bool> {
    map(
        unsupported_case_parser,
        |_| true
    ).parse(input)
}
