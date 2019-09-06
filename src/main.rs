use std::convert::TryFrom;
use std::io::{self, Read};
use std::process;

use m_o::value::Value;

fn read_stdin() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("stdin read will succeed");
    buf
}

fn parse_input(input: &str) -> Value {
    Value::try_from(input.trim()).unwrap_or_else(|e| {
        eprintln!("Error: Could not parse input as Python data expression!");
        eprintln!("\t{:?}", e);
        process::exit(1);
    })
}

fn terminal_width() -> usize {
    termion::terminal_size()
        .map(|(w, _h)| w as usize)
        .unwrap_or(80)
}

fn pretty_print(value: Value) {
    let width = terminal_width();
    let doc = value.to_doc();
    println!("{}", doc.pretty(width));
}

fn main() {
    let input = read_stdin();
    let value = parse_input(&input);
    pretty_print(value);
}
