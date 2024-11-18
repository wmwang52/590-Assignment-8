// Team Members:
// - [Cora Rogers, Dylan Nicks, William Wang]

use std::sync::mpsc::{Sender, Receiver};
use crate::Message;

pub fn serv2(rx: Receiver<Message>, tx: Sender<Message>) {
    loop {
        if let Ok(msg) = rx.recv() {
            match msg {
                Message::Halt => {
                    tx.send(Message::Halt).unwrap();
                    println!("(serv2) Halting.");
                    break;
                }
                Message::List(numbers) => {
                    if numbers.is_empty() {
                        tx.send(Message::List(numbers)).unwrap();
                        continue;
                    }
                    let head = numbers[0];
                    if head.fract() == 0.0 {
                        let sum: f64 = numbers.iter().filter(|x| x.is_finite()).sum();
                        println!("(serv2) Sum of numbers: {}", sum);
                    } else {
                        let product: f64 = numbers.iter().filter(|x| x.is_finite()).product();
                        println!("(serv2) Product of numbers: {}", product);
                    }
                }
                _ => tx.send(msg).unwrap(),
            }
        }
    }
}