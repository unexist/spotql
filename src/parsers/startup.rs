///
/// @package Spotql
///
/// @file Spotql startup parser
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use std::str::from_utf8;
use std::collections::HashMap;
use nom::number::Endianness;

#[derive(Debug)]
pub struct Startup<'a> {
    pub protocol_version: i32,
    pub len: i32,
    pub parameters: Option<HashMap<&'a str, &'a str>>,
}

/* Startup message: int32 len | int32 protocol version | key \0 | value \0 | \0 */
named!(pub startup_parser<&[u8], Startup>,
    dbg_dmp!(
        do_parse!(
            len: i32!(Endianness::Big) >>
            version: i32!(Endianness::Big) >>
            payload: opt!(
                separated_list0!(
                    tag!([0]),
                    is_a!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_01234567890")
                )
            ) >>
            (Startup {
                len: len,
                protocol_version: version,
                parameters: if let Some(list) = payload {
                    let mut params: HashMap<&str, &str> = HashMap::new();
                    let mut iter = list.iter();

                    while let Some(key) = iter.next() {
                        if let Some(value) = iter.next() {
                            params.insert(from_utf8(key).unwrap(), from_utf8(value).unwrap());
                        }
                    }

                    Some(params)
                } else {
                    None
                },
            })
        )
    )
);