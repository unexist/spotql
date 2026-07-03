#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, unused_assignments))]

//!
//! @package Spotql
//!
//! @file Spotql main entry
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

#[cfg(test)]
mod tests;
mod parsers;
mod logger;
mod config;

use anyhow::{Result, bail};
use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
#[allow(unused_imports)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use parsers::message::{Message, parse_message};
use std::mem;
use std::str::from_utf8;
use crate::config::Config;

/// Print version info
fn print_version() {
    info!("{} {} - Copyright (c) 2025-present {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    info!("Released under the GNU GPLv3");
}

/// Main function
///
/// # Returns
///
/// A [`Result`] with either [`unit`] on success or otherwise [`anyhow::Error`]
#[tokio::main]
async fn main() -> Result<()> {
    // Load config
    let (config, maybe_path, _format) = Config::parse_info();

    logger::init(&config)?;

    print_version();

    if let Some(path) = maybe_path {
        info!("Reading file `{:?}'", path);
    }
    debug!("Config: {:?}", config);

    let listen_addr = format!("{}:{}", config.hostname, config.port);

    info!("Listening on `{}'", listen_addr);

    // Finally start listener loop
    let listener = TcpListener::bind(listen_addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(err) = process(socket).await {
                error!("Error reading data: `{:?}`", err);
            }
        });
    }
}

async fn send_param(socket: &mut TcpStream, param_name: &str, param_val: &str) -> Result<()> {
    let formatted = format!("{}\0{}\0", param_name, param_val);
    let message = formatted.as_bytes();

    socket.write_u8('S' as u8).await?;
    socket.write_i32(4 + mem::size_of_val(message) as i32).await?; // Message len
    socket.write(message).await?;

    Ok(())
}

/// Handle tcp communication
///
/// # Arguments
///
/// * `socket` - Tcp stream of the connected client
///
/// # Returns
///
/// A [`Result`] with either [`unit`] on success or otherwise [`anyhow::Error`]
async fn process(mut socket: TcpStream) -> Result<()> {
    let mut buf = vec![0; 1024];

    loop {
        let n = socket.read(&mut buf).await?;

        if 0 == n {
            continue;
        }

        info!("Read data: n={:?}, data={:?}", n, from_utf8(&buf[0..n])?);

        match parse_message(&buf[0..n]) {
            Ok(result) => {
                match result {
                    Message::Startup(startup) => {
                        info!("Parsed startup message: {:?}", startup);

                        /* Send NegotiateProtocolVersion - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-NEGOTIATEPROTOCOLVERSION> */
                        socket.write_all(&['v' as u8, 0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0]).await?;

                        /* Ask for password */
                        socket.write_all(&['R' as u8, 0, 0, 0, 8, 0, 0, 0, 3]).await?;
                    },
                    Message::Auth(auth) => {
                        info!("Parsed auth message: {:?}", auth);

                        /* Send AuthenticationOk - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONOK> */
                        socket.write_all(&['R' as u8, 0, 0, 0, 8, 0, 0, 0, 0]).await?;

                        /* Send ParameterStatus - <https://www.postgresql.org/docs/13/protocol-flow.html#PROTOCOL-ASYNC> */
                        send_param(&mut socket, "application_name", env!("CARGO_PKG_NAME")).await?;
                        send_param(&mut socket, "server_version", env!("CARGO_PKG_VERSION")).await?;

                        /* Send ReadyForQuery  - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-READYFORQUERY> */
                        socket.write_all(&['Z' as u8, 0, 0, 0, 5, 'I' as u8]).await?;
                    },
                    Message::Query(query) => {
                        info!("Parsed query message: {:?}", query);

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
                        info!("Parsed terminate message: {:?}", terminate);

                        break;
                    },
                    #[allow(unreachable_patterns)]
                    _ => unreachable!()
                };
            },
            Err(e) => bail!(e)
        }
    }

    Ok(())
}
