//!
//! @package Spotql
//!
//! @file Spotql terminate parser
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use nom::IResult;
use nom::combinator::map;
use nom::character::complete::anychar;
use nom::Parser;

#[derive(Debug)]
pub struct Terminate {
    pub tag: char,
    pub len: i32,
}

/* Terminate message: char tag | int32 len | \0 */
pub(crate) fn terminate_parser(input: &[u8]) -> IResult<&[u8], Terminate> {
    map(
        (
            anychar,
            nom::character::complete::i32,
        ),
        |(tag, len)| Terminate {
            tag,
            len
        }
    ).parse(input)
}
