use std::io::prelude::*;
use std::fs::File;

fn file_to_string() -> String {
    let mut file = match File::open("test.b") {
        Ok(file) => file,
        Err(..) => panic!("looking for a file called test.b"),
    };
    let mut s = String::new();
    let content = file.read_to_string(&mut s);
    s
}

fn main() {
    let s = file_to_string();
    let mut tape = [0; 30000];
    let mut ptr = 0;

    let mut index = 0;
    let s: Vec<char> = s.chars().collect();
    let len = s.len();

    while index < len {
        let c = s[index];
        match c {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => tape[ptr] += 1,
            '-' => tape[ptr] -= 1,
            '.' => {
                let unsigned = tape[ptr] as u8;
                print!("{}", unsigned as char);
            },
            '[' => {
                if tape[ptr] == 0 {
                    let mut level = 1;
                    while level > 0 {
                        index += 1;
                        match s[index] {
                            '[' => level += 1,
                            ']' => level -= 1,
                            _ => {},
                        }
                    }
                }
            },
            ']' => {
                if tape[ptr] != 0 {
                    let mut level = 1;
                    while level > 0 {
                        index -= 1;
                        match s[index] {
                            '[' => level -= 1,
                            ']' => level += 1,
                            _ => {},
                        }
                    }
                }
            },

            _ => {},
        }
        index += 1;
    };
}
