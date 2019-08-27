use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).expect("stdin read will succeed");
    let (_, value) = m_o::parse_value(&buf).expect("input is parsable Python expression");
    println!("{}", value);
    Ok(())
}
