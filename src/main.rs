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
mod wire;

use anyhow::{Result, bail};
use log::{debug, error, info};
use tokio::{net::{TcpListener, TcpStream}, signal};
#[allow(unused_imports)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::sync::CancellationToken;
use parsers::message::{Message, parse_message};
use std::str::from_utf8;
use crate::{config::Config, wire::send_row_data};
#[allow(unused_imports)]
use crate::wire::{send_auth_ok, send_auth_request, send_command_complete, send_param, send_proto_negotiation, send_ready_for_query, send_row_descriptions};

/// Print version info
fn print_version() {
    info!("{} {} - Copyright (c) 2025-present {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    info!("Released under the GNU GPLv3");
}

/// Run server thread
///
/// # Arguments
///
/// * `listen_addr` - Adress and port to listen on
/// * `token` -  Cancellation token to allow graceful shutdown
///
/// # Returns
///
/// A [`Result`] with either [`unit`] on success or otherwise [`anyhow::Error`]
async fn run(listen_addr: &str, token: CancellationToken) -> Result<()> {
    info!("Listening on `{}'", listen_addr);

    let listener = TcpListener::bind(listen_addr).await?;

    loop {
        tokio::select! {
            result = listener.accept() => {
                if let Ok((socket, _)) = result {
                    let cloned_token = token.clone();

                    tokio::spawn(async move {
                        if let Err(err) = handle_connection(socket, cloned_token).await {
                            error!("Error reading data: `{:?}`", err);
                        }
                    });
                }
            },
            _ = token.cancelled() => {
                info!("Exit listener");

                break;
            }
        }
    }

    Ok(())
}

/// Handle tcp communication
///
/// # Arguments
///
/// * `socket` - Tcp stream of the connected client
/// * `token` -  Cancellation token to allow graceful shutdown
///
/// # Returns
///
/// A [`Result`] with either [`unit`] on success or otherwise [`anyhow::Error`]
async fn handle_connection(mut socket: TcpStream, token: CancellationToken) -> Result<()> {
    let mut buf = vec![0; 1024];

    loop {
        tokio::select! {
            result = socket.read(&mut buf) => {
                let n = result.unwrap_or(0);

                if 0 == n {
                    continue;
                }

                debug!("Read data: n={:?}, data={:?}", n, from_utf8(&buf[0..n])?);

                match parse_message(&buf[0..n]) {
                    Ok(result) => {
                        match result {
                            Message::Startup(startup) => {
                                info!("Received startup message: {:?}", startup);

                                //send_proto_negotiation(&mut socket).await?;
                                send_auth_request(&mut socket).await?;
                            },
                            Message::Auth(auth) => {
                                info!("Received auth message: {:?}", auth);

                                send_auth_ok(&mut socket).await?;
                                send_param(&mut socket, "application_name", env!("CARGO_PKG_NAME")).await?;
                                send_param(&mut socket, "server_version", env!("CARGO_PKG_VERSION")).await?;
                                send_ready_for_query(&mut socket).await?;
                            },
                            Message::Query(query) => {
                                info!("Received query message: {:?}", query);

                                send_row_descriptions(&mut socket, &vec!["name", "watt"]).await?;
                                send_row_data(&mut socket, &vec!["foo", "bar"]).await?;
                                send_command_complete(&mut socket, &format!("SELECT {}", 1)).await?;
                                send_ready_for_query(&mut socket).await?;
                            },
                            Message::Terminate(terminate) => {
                                info!("Received terminate message: {:?}", terminate);

                                break;
                            },
                            #[allow(unreachable_patterns)]
                            _ => unreachable!()
                        };
                    },
                    Err(e) => bail!(e)
                }
            },
            _ = token.cancelled() => {
                info!("Exit connection handler");

                break;
            }
        };
    }

    Ok(())
}

/// Main function
///
/// # Returns
///
/// A [`Result`] with either [`unit`] on success or otherwise [`anyhow::Error`]
#[tokio::main]
async fn main() -> Result<()> {
    let token = CancellationToken::new();

    // Load config
    let (config, maybe_path, _format) = Config::parse_info();

    logger::init(&config)?;

    print_version();

    if let Some(path) = maybe_path {
        info!("Reading file `{:?}'", path);
    }
    debug!("Config: {:?}", config);

    // Listen on address
    let listen_addr = format!("{}:{}", config.hostname, config.port);
    let cloned_token = token.clone();

    let handle = tokio::spawn(async move {
        run(&listen_addr, cloned_token).await
    });

    signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
    info!("Shutting down...");

    token.cancel();

    info!("Exit");

    Ok(())
}
