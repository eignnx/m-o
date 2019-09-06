use std::convert::TryFrom;
use std::io::{self, Read};
use std::process;

use structopt::StructOpt;

use m_o::value::Value;

mod opt;

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

fn pretty_print(value: Value, options: &opt::Opt) {
    let doc = value.to_doc(&options.into());
    println!("{}", doc.pretty(options.columns));
}

fn main() {
    let options = opt::Opt::from_args();
    let input = read_stdin();
    let value = parse_input(&input);
    pretty_print(value, &options);
}
