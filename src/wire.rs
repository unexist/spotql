//!
//! @package Spotql
//!
//! @file Postgres wire functions
//! @copyright (c) 2021-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv2.
//! See the file LICENSE for details.
//!

use anyhow::Result;
#[allow(unused_imports)]
use log::{debug, error, info};
use stdext::function_name;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::mem;

pub(crate) async fn send_auth_request(socket: &mut TcpStream) -> Result<()> {
    debug!("{}", function_name!());

    /* Ask for password */
    socket.write_all(&['R' as u8, 0, 0, 0, 8, 0, 0, 0, 3]).await?;

    Ok(())
}

pub(crate) async fn send_auth_ok(socket: &mut TcpStream) -> Result<()> {
    debug!("{}", function_name!());

    /* Send AuthenticationOk - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONOK> */
    socket.write_all(&['R' as u8, 0, 0, 0, 8, 0, 0, 0, 0]).await?;

    Ok(())
}

pub(crate) async fn send_param(socket: &mut TcpStream, param_name: &str, param_val: &str) -> Result<()> {
    let formatted = format!("{}\0{}\0", param_name, param_val);
    let message = formatted.as_bytes();

    debug!("{}: name={}, value={}", function_name!(), param_name, param_val);

    /* Send ParameterStatus - <https://www.postgresql.org/docs/13/protocol-flow.html#PROTOCOL-ASYNC> */
    socket.write_u8('S' as u8).await?;
    socket.write_i32(4 + mem::size_of_val(message) as i32).await?; // Message len
    socket.write(message).await?;

    Ok(())
}

pub(crate) async fn send_proto_negotiation(socket: &mut TcpStream) -> Result<()> {
    debug!("{}", function_name!());

    /* Send NegotiateProtocolVersion - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-NEGOTIATEPROTOCOLVERSION> */
    socket.write_all(&['v' as u8, 0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0]).await?;

    Ok(())
}

pub(crate) async fn send_ready_for_query(socket: &mut TcpStream) -> Result<()> {
    debug!("{}", function_name!());

    /* Send ReadyForQuery  - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-READYFORQUERY> */
    socket.write_all(&['Z' as u8, 0, 0, 0, 5, 'I' as u8]).await?;

    Ok(())
}
