//! Basic TCP server

mod requests_handler;

use std::net::TcpListener;
use std::thread::spawn
use std::sync::{
    Mutex,
    Arc,
};

use std::sync::mpsc::{
    Sender,
    Receiver,
    channel,
};

use requests_handler::{
    receive_message,
    handle_sent_messages,
    send_to_client,
};

fn main() {
    
    /* create a TCP socket server, listening for connections */
    let listener = TcpListener::bind("0.0.0.0:9090").unwrap();
};