///
/// @package Spotql
///
/// @file Spotql parser
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use std::str;
use nom::character::complete::{
    multispace0,
    alphanumeric1,
};

#[derive(Debug)]
pub struct Column<'a> {
    pub name: &'a str,
    pub alias: Option<&'a str>,
}

//
// Column parser
//

named!(pub column_name_parser<&[u8], &str>,
    map_res!(
        delimited!(
            multispace0,
            alt!(
                tag!("*")
                | alphanumeric1
            ),
            multispace0
        ), str::from_utf8
    )
);

named!(pub column_parser<&[u8], Column>,
    do_parse!(
        name: column_name_parser >>
        alias: opt!(
            complete!(
                preceded!(
                    delimited!(
                        multispace0,
                        tag!("as"),
                        multispace0
                    ),
                    column_name_parser
                )
            )
        ) >>
        (Column {
            name: name,
            alias: alias,
        })
    )
);

named!(pub column_list_parser<&[u8], Vec<Column>>,
    complete!(
        dbg_dmp!(
            separated_list0!(
                complete!(
                    delimited!(
                        multispace0,
                        tag!(","),
                        multispace0
                    )
                ),
                column_parser
            )
        )
    )
);
