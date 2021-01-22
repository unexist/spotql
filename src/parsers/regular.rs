///
/// @package Spotql
///
/// @file Spotql regular parser
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use nom::number::Endianness;
use nom::character::complete::anychar;

#[derive(Debug)]
pub struct Regular<'a> {
    pub tag: char,
    pub len: i32,
    pub payload: Option<&'a str>,
}

/* Regular message: char tag | int32 len | payload | \0 */
named!(pub regular_parser<&[u8], Regular>,
    dbg_dmp!(
        do_parse!(
            tag: anychar >>
            len: i32!(Endianness::Big) >>
            (Regular {
                tag: tag,
                len: len,
                payload: None,
            })
        )
    )
);