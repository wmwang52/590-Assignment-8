// Team Members:
// - [Cora Rogers, Dylan Nicks, William Wang]

use std::sync::mpsc::Receiver;
use crate::Message;

pub fn serv3(rx: Receiver<Message>) {
    let mut unprocessed_count = 0;

    loop {
        if let Ok(msg) = rx.recv() {
            match msg {
                Message::Halt => {
                    println!(
                        "(serv3) Halting. Unprocessed messages count: {}",
                        unprocessed_count
                    );
                    break;
                }
                Message::Error(error_msg) => {
                    println!("(serv3) Error: {}", error_msg);
                }
                other => {
                    println!("(serv3) Not handled: {:?}", other);
                    unprocessed_count += 1;
                }
            }
        }
    }
}