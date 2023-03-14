// Methods to handle clients requests

use std::net::TcpStream;

use std::io::{
    Write,
    BufReader,
    BufRead,
};

use std::sync::{
    mpsc,
    Mutex,
    Arc,
};

/// Run by threads created at each client connection. Handles the sent messages by one client. 
/// There is one thread per client.
///
/// # Arguments:
///
/// * `stream` - TCP stream between the server and the new connected client
/// * `sender` - the sender to use to forward the received message

pub fn handle_sent_messages(
    stream: TcpStream,
    sender: mpsc::Sender<String>,
) {
    /* create a buffer in order to read data sent through the stream;
       in other words, the data sent by the client attached to this stream */

    let mut buffer = BufReader::new(stream);
    let mut message = String::new();

    loop {
        /* blocking step to read data from the client stream */
        let request = buffer.read_line(&mut message);

        if request.is_err() {
            continue;
        }
    

        /* get message as bytes slice in order to check
            what character exactly has been sent */
        let message_copy = message.clone();
        let message_bytes = message_copy.as_bytes();

        /* ignore the message if the first character
            is a carriage return */
        const END_OF_LINE: u8 = 10;
        if message_bytes.get(0) == Some(&END_OF_LINE) {
            break;
        }

        /* attempt to send the message through the current client sender;
            as the sender is part of the senders dynamic array
            created from one unique receiver, the messages sent by every client
            all go to this unique receiver for forward */
        let send_message = sender.send(message.to_string());
        if send_message.is_err() {
            break;
        }
        message.clear();
    }
}