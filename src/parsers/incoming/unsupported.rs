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

use nom::branch::alt;
use nom::combinator::complete;
use nom::combinator::map;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom::sequence::pair;

use crate::parsers::incoming::expression::expression_parser;
use crate::parsers::incoming::common::{btag, ws};

/* Case statement: case expr when expr then expr end */
pub(crate) fn unsupported_case_parser(input: &[u8]) -> IResult<&[u8], bool> {
    map(
        (
            ws(btag("case")),
            expression_parser,
            complete(
                many0(
                    pair(
                        ws(
                            alt((
                                btag("when"),
                                btag("then"),
                            ))
                        ),
                        expression_parser
                    )
                )
            ),
            ws(btag("end")),
        ),
        |(case_start, expression, when_then, case_end)| true
    ).parse(input)
}

pub(crate) fn unsupported_parser(input: &[u8]) -> IResult<&[u8], bool> {
    map(
        alt(
            (
                unsupported_case_parser,
            )
        ),
        |_| true
    ).parse(input)
}
