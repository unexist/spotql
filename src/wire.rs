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
    socket.write_all(&['R' as u8, // Tag name
        0, 0, 0, 8, // Message len
        0, 0, 0, 3
    ]).await?;

    Ok(())
}

pub(crate) async fn send_auth_ok(socket: &mut TcpStream) -> Result<()> {
    debug!("{}", function_name!());

    /* Send AuthenticationOk - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONOK> */
    socket.write_all(&['R' as u8, // Tag name
        0, 0, 0, 8, // Message len
        0, 0, 0, 0
    ]).await?;

    Ok(())
}

pub(crate) async fn send_param(socket: &mut TcpStream, param_name: &str, param_val: &str) -> Result<()> {
    debug!("{}: name={}, value={}", function_name!(), param_name, param_val);

    let formatted = format!("{}\0{}\0", param_name, param_val);
    let message = formatted.as_bytes();

    /* Send ParameterStatus - <https://www.postgresql.org/docs/13/protocol-flow.html#PROTOCOL-ASYNC> */
    socket.write_u8('S' as u8).await?;
    socket.write_i32(4 + mem::size_of_val(message) as i32).await?; // Message len
    socket.write(message).await?;

    Ok(())
}

pub(crate) async fn send_proto_negotiation(socket: &mut TcpStream) -> Result<()> {
    debug!("{}", function_name!());

    /* Send NegotiateProtocolVersion - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-NEGOTIATEPROTOCOLVERSION> */
    socket.write_all(&['v' as u8, // Tag name
        0, 0, 0, 12, // Message len
        0, 3, 0, 0,
        0, 0, 0, 0
    ]).await?;

    Ok(())
}

pub(crate) async fn send_ready_for_query(socket: &mut TcpStream) -> Result<()> {
    debug!("{}", function_name!());

    /* Send ReadyForQuery  - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-READYFORQUERY> */
    socket.write_all(&['Z' as u8, // Tag name
        0, 0, 0, 5, // Message len
        'I' as u8 // Transaction status (I = idle, T = transaction, E = transaction error)
    ]).await?;

    Ok(())
}

pub(crate) async fn send_command_complete(socket: &mut TcpStream, command_tag: &str) -> Result<()> {
    debug!("{}: command_tag={}", function_name!(), command_tag);

    let formatted = format!("{}\0", command_tag);
    let message = formatted.as_bytes();

    /* Send CommandComplete - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-COMMANDCOMPLETE> */
    socket.write_u8('C' as u8).await?; // Tag name
    socket.write_i32(4 + mem::size_of_val(message) as i32).await?; // Message len
    socket.write(message).await?;

    Ok(())
}

pub(crate) async fn send_row_descriptions(socket: &mut TcpStream, row_names: &Vec<&str>) -> Result<()> {
    debug!("{}: rows={:?}", function_name!(), row_names);

    let len_in_bytes: i32 = row_names.len() as i32 + row_names.iter().map(|&s| mem::size_of_val(s) as i32).sum::<i32>();

    /* Send RowDescription - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-ROWDESCRIPTION> */
    socket.write_u8('T' as u8).await?;
    socket.write_i32(6 + 18 * row_names.len() as i32 + len_in_bytes).await.ok(); // Message len
    socket.write_i16(row_names.len() as i16).await?; // Number of columns

    /* Repeat for each row */
    for (oid, row) in row_names.iter().enumerate() {
        let formatted = format!("{}\0", row);
        let message = formatted.as_bytes();

        socket.write(message).await?;
        socket.write_i32(oid as i32).await?; // Table OID
        socket.write_i16(oid as i16).await?; // Attribute number of column
        socket.write_i32(25).await?; // Object OID of type: text = 25
        socket.write_i16(-1).await?; // Data type len
        socket.write_i32(0).await?; // Data type modifier
        socket.write_i16(0).await?; // Format code: text = 0, binary = 1
    }

    Ok(())
}

pub(crate) async fn send_row_data(socket: &mut TcpStream, row_data: &Vec<&str>) -> Result<()> {
    debug!("{}: row_data={:?}", function_name!(), row_data);

    let len_in_bytes: i32 = row_data.len() as i32 + row_data.iter().map(|&s| mem::size_of_val(s) as i32).sum::<i32>();

    /* Send DataRow - <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-DATAROW> */
    socket.write_u8('D' as u8).await.ok();
    socket.write_i32(6 + len_in_bytes + row_data.len() as i32 * 4).await.ok(); // Message len
    socket.write_i16(row_data.len() as i16).await.ok(); // Number of columns

    for data in row_data.iter() {
        let formatted = format!("{}\0", data);
        let message = formatted.as_bytes();

        socket.write_i32(mem::size_of_val(message) as i32).await.ok(); // Length of column value without self
        socket.write(message).await.ok();
    }

    Ok(())
}
