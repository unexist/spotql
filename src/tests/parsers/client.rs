///
/// @package Spotql
///
/// @file Spotql client parser tests
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use crate::parsers::client::{ parse_client };

///
/// Startup packet
///

#[test]
fn test_parse_client() {
    match parse_client("\u{0}\u{0}\u{0}N\u{0}\u{3}\u{0}\u{0}user\u{0}unexist\u{0}database\u{0}foo\u{0}application_name\u{0}psql\u{0}client_encoding\u{0}UTF8\u{0}\u{0}".as_bytes()) {
        Ok(client) => {
            assert_eq!(client.protocol_version, 3);
            assert_eq!(client.username, Some("unexist"));
            assert_eq!(client.database, Some("foo"));
        }   ,
        Err(e) => panic!(format!("Error: {}", e)),
    }
}