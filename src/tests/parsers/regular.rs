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

//
// Startup packet
//

static MESSAGE: &'static str = "N\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}";

#[test]
fn test_parse_regular() {
    match parse_regular(MESSAGE.as_bytes()) {
        Ok(regular) => {
            assert_eq!(regular.tag, 'N');
        },
        Err(e) => panic!(format!("Error: {}", e)),
    }
}