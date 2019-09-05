use std::convert::TryFrom;
use std::io::{self, Read, Write};
use std::process;

use m_o::value::Value;

fn main() {
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin
        .read_to_string(&mut buf)
        .expect("stdin read will succeed");
    let value = Value::try_from(buf.trim()).unwrap_or_else(|e| {
        eprintln!("Error: Could not parse input as Python data expression!");
        eprintln!("\t{:?}", e);
        process::exit(1);
    });

    // Get terminal width (if it's available), use 80 as default.
    let (width, _height) = termion::terminal_size().unwrap_or_else(|_| (80, 0));

    let doc = value.to_doc();
    let fmt = doc.pretty(width as usize);
    writeln!(io::stdout(), "{}", fmt).expect("stdout can be written to");
}
