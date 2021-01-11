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

#[allow(unused_imports)]
use parsers::startup::{Startup, parse_startup};

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

                println!("Read {} bytes", n);

                let client = parse_startup(&buf[0..n]);

                println!("Client {:?}", client)

                /*println!("{:?}", String::from_utf8_lossy(&buf[0..n]));

                let reply = b"AuthenticationCleartextPassword";

                socket.write_all(&reply[..]).await.expect("Failed to write data to socket");*/
            }
        });
    }
}