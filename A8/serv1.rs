// Team Members:
// - [Cora Rogers, Dylan Nicks, William Wang]

use std::sync::mpsc::{Sender, Receiver};
use crate::Message;

pub fn serv1(rx: Receiver<Message>, tx: Sender<Message>) {
    loop {
        if let Ok(msg) = rx.recv() {
            match msg {
                Message::Halt => {
                    tx.send(Message::Halt).unwrap();
                    println!("(serv1) Halting.");
                    break;
                }
                Message::ArithmeticOp(op, a, b) => {
                    let result = match op.as_str() {
                        "add" => a + b,
                        "sub" => a - b,
                        "mult" => a * b,
                        "div" if b != 0.0 => a / b,
                        "div" => {
                            println!("(serv1) Error: Division by zero");
                            continue;
                        }
                        _ => {
                            tx.send(Message::ArithmeticOp(op, a, b)).unwrap();
                            continue;
                        }
                    };
                    println!("(serv1) {} {} {} = {}", a, op, b, result);
                }
                Message::SingleOp(op, a) => {
                    let result = match op.as_str() {
                        "neg" => -a,
                        "sqrt" if a >= 0.0 => a.sqrt(),
                        "sqrt" => {
                            println!("(serv1) Error: Sqrt of negative number");
                            continue;
                        }
                        _ => {
                            tx.send(Message::SingleOp(op, a)).unwrap();
                            continue;
                        }
                    };
                    println!("(serv1) {} {} = {}", op, a, result);
                }
                _ => tx.send(msg).unwrap(),
            }
        }
    }
}