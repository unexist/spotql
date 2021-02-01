///
/// @package Spotql
///
/// @file Spotql main entry
/// @copyright (c) 2021 Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

#[macro_use]
extern crate nom;
extern crate tokio;

#[cfg(test)]
mod tests;
mod parsers;

use tokio::net::TcpListener;
#[allow(unused_imports)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use parsers::message::{ Message, parse_message };

use std::mem;
use std::str::from_utf8;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:5432").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket.read(&mut buf).await.expect("Failed to read data from socket");

                if 0 == n {
                    return;
                }

                println!("Read: n={:?}, data={:?}", n, from_utf8(&buf[0..n]).unwrap());

                match parse_message(&buf[0..n]) {
                    Ok(result) => {
                        match result {
                            Message::Startup(startup) => {
                                println!("Parsed startup message: {:?}", startup);

                                /* Ask for password */
                                socket.write_u8('R' as u8).await.ok();
                                socket.write_i32(8).await.ok();
                                socket.write_i32(3).await.ok();
                            },
                            Message::Auth(auth) => {
                                println!("Parsed auth message: {:?}", auth);

                                /* Tell password is ok */
                                socket.write_u8('R' as u8).await.ok();
                                socket.write_i32(8).await.ok();
                                socket.write_i32(0).await.ok();

                                /* Tell ready for query */
                                socket.write_u8('Z' as u8).await.ok();
                                socket.write_i32(5).await.ok();
                                socket.write_u8('I' as u8).await.ok();
                            },
                            Message::Query(query) => {
                                println!("Parsed query message: {:?}", query);

                                /* Tell row description */
                                let message = b"name\0";

                                socket.write_u8('T' as u8).await.ok();
                                socket.write_i32(29).await.ok(); // Message len
                                socket.write_i16(1).await.ok(); // Number of columns
                                socket.write(message).await.ok();
                                socket.write_i32(0).await.ok(); // Table OID
                                socket.write_i16(0).await.ok(); // Attribute number of column
                                socket.write_i32(25).await.ok(); // Object OID of type: text = 25
                                socket.write_i16(-1).await.ok(); // Data type len
                                socket.write_i32(0).await.ok(); // Data type modifier
                                socket.write_i16(0).await.ok(); // Format code: text = 0, binary = 1

                                /* Tell data rows */
                                let message = b"test\0";

                                socket.write_u8('D' as u8).await.ok();
                                socket.write_i32(4 + 2 + 4 + mem::size_of_val(message) as i32).await.ok(); // Message len
                                socket.write_i16(1).await.ok(); // Number of columns
                                socket.write_i32(mem::size_of_val(message) as i32).await.ok(); // Length of column value without self
                                socket.write(message).await.ok();

                                /* Tell command complete */
                                let message = b"SELECT 0\0";

                                socket.write_u8('C' as u8).await.ok();
                                socket.write_i32(4 + mem::size_of_val(message) as i32).await.ok();
                                socket.write(message).await.ok();

                                /* Tell ready for query */
                                socket.write_u8('Z' as u8).await.ok();
                                socket.write_i32(5).await.ok();
                                socket.write_u8('I' as u8).await.ok();
                            },
                            Message::Terminate(terminate) => {
                                println!("Parsed terminate message: {:?}", terminate);
                            },
                            #[allow(unreachable_patterns)]
                            _ => unreachable!()
                        };
                    },
                    Err(e) => eprintln!("Error {:?}", e)
                }
            }
        });
    }
}