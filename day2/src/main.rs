use std::io::{self, Write};


fn main() {
    print!("Hello, world!");
    io::stdout().flush().unwrap(); // flushes the output buffer
}