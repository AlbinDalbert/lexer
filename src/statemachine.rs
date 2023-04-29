use crate::token::Token;
use crate::tokenize;
use std::fs::File;
use std::io::{BufReader, Read};

mod token;
mod tokenize;

enum State {
    TopLevel,
    InCluster,
    InEnum,
    InStruct,
    InFuction,
}

struct StateMachine {
    state: State,
    token: Token,
}

impl StateMachine {
    fn transition(&mut self) {
        match self.state {
            State::TopLevel => {}
            State::InCluster => {}
            State::InEnum => {}
            State::InStruct => {}
            State::InFunction => {}
            _ => {}
        }
    }

    fn run(&mut self, path: &str) {
        let mut reader = open_file(path);
        loop {
            match reader.read_char() {}
        }
    }
}

fn top_level(machine: StateMachine) {}

fn open_file(path: &str) -> BufReader {
    let file = File::open(path).unwrap();
    BufReader::new_with_encoding(file, encoding_rs::UTF_8)
}
