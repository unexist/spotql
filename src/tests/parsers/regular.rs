///
/// @package Spotql
///
/// @file Spotql regular parser tests
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parsers::regular::{ parse_regular };

///
/// Startup packet
///

#[test]
fn test_parse_regular() {
    match parse_regular("N\u{0}\u{3}\u{0}\u{0}user\u{0}unexist\u{0}database\u{0}foo\u{0}application_name\u{0}psql\u{0}client_encoding\u{0}UTF8\u{0}\u{0}".as_bytes()) {
        Ok(regular) => {
            println!("### {:?} ###", regular);

            assert_eq!(regular.tag, 'N');
            assert_eq!(regular.len, 768);
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}