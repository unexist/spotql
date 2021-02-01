///
/// @package Spotql
///
/// @file Spotql terminate parser
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use nom::number::Endianness;
use nom::character::complete::anychar;

#[derive(Debug)]
pub struct Terminate {
    pub tag: char,
    pub len: i32,
}

/* Terminate message: char tag | int32 len | \0 */
named!(pub terminate_parser<&[u8], Terminate>,
    dbg_dmp!(
        do_parse!(
            tag: anychar >>
            len: i32!(Endianness::Big) >>
            (Terminate {
                tag: tag,
                len: len,
            })
        )
    )
);