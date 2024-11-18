// Team Members:
// - [Cora Rogers, Dylan Nicks, William Wang]

mod serv1;
mod serv2;
mod serv3;

use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

use serv1::serv1;
use serv2::serv2;
use serv3::serv3;

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();

    thread::spawn(move || serv1(rx1, tx2));
    thread::spawn(move || serv2(rx2, tx3));
    thread::spawn(move || serv3(rx3));

    loop {
        println!("Enter a message (or 'all_done' to exit):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "all_done" {
            tx1.send(Message::Halt).unwrap();
            println!("Main process exiting.");
            break;
        }

        match parse_input(input) {
            Ok(msg) => tx1.send(msg).unwrap(),
            Err(e) => println!("Error parsing input: {}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Halt,
    ArithmeticOp(String, f64, f64),
    SingleOp(String, f64),
    List(Vec<f64>),
    Error(String),
    Unknown(String),
}

fn parse_input(input: &str) -> Result<Message, &'static str> {
    if input.starts_with("[") && input.ends_with("]") {
        // Parse list
        let numbers: Vec<f64> = input[1..input.len() - 1]
            .split(',')
            .filter_map(|s| s.trim().parse::<f64>().ok())
            .collect();
        return Ok(Message::List(numbers));
    }

    if input.starts_with("{error,") && input.ends_with("}") {
        // Parse {error, "Some error message"}
        let error_msg = input[7..input.len() - 1]
            .trim()
            .trim_matches('"')
            .to_string();
        return Ok(Message::Error(error_msg));
    }

    if let Some((op, args)) = input.split_once(' ') {
        let args: Vec<f64> = args
            .split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();

        match (op, args.as_slice()) {
            ("add" | "sub" | "mult" | "div", [a, b]) => {
                return Ok(Message::ArithmeticOp(op.to_string(), *a, *b))
            }
            ("neg" | "sqrt", [a]) => return Ok(Message::SingleOp(op.to_string(), *a)),
            _ => return Err("Invalid operation or arguments"),
        }
    }

    if input == "halt" {
        return Ok(Message::Halt);
    }

    Ok(Message::Unknown(input.to_string()))
}