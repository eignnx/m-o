use std::convert::TryFrom;
use std::io::{self, Read};
use std::process;

use m_o::Value;

fn main() {
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).expect("stdin read will succeed");
    let value = Value::try_from(buf.trim()).unwrap_or_else(|e| {
        eprintln!("Error: Could not parse input as Python data expression!");
        eprintln!("\t{:?}", e);
        process::exit(1);
    });
    println!("{}", value);
}
