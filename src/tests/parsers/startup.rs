///
/// @package Spotql
///
/// @file Spotql startup parser tests
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parsers::startup::{ parse_startup };

///
/// Startup packet
///

#[test]
fn test_parse_startup() {
    match parse_startup("\u{0}\u{0}\u{0}".as_bytes()) {
        Ok(startup) => {
            assert_eq!(startup.protocol_version, 0);
            assert_eq!(startup.len, 0);
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}