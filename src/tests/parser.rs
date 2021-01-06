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

use crate::parser::parse;

#[test]
fn test_parse_simple_statement() {
    match parse("select * from playlists") {
        Ok(stmt) => {
            assert_eq!(stmt.verb, "select");
        },
        Err(e) => panic!(format!("Incomplete: {:?}", e)),
    }
}