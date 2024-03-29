use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn main() {
    for i in 0..=10 {
        print!("\rProgress: {}%", i * 10);
        io::stdout().flush().unwrap();
        // 等待一段时间，模拟耗时操作
        thread::sleep(Duration::from_millis(1));
    }
    println!("\nDone!");
}
