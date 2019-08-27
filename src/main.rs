use std::io::{self, Read};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for byte in stdin.lock().bytes() {
        let byte = byte?;
        println!(">>> {}", byte);
    }
    Ok(())
}
