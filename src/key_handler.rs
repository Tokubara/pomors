use std::io::stdin;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

pub const ENTER: &str = "enter";
pub const TERMINATE: &str = "terminate";

// TODO: Enumで返すようにする
pub fn run() -> Receiver<&'static str> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    sender.send(ENTER).unwrap();
                }
                Key::Char('q') => {
                    sender.send(TERMINATE).unwrap();
                    break;
                }
                Key::Ctrl(c) => {
                    if c == 'c' {
                        sender.send(TERMINATE).unwrap();
                        break;
                    }
                }
                _ => (),
            }
        }
    });
    receiver
}