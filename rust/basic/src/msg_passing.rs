use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("recv: {}", data);
    });
    let _ = tx.send("hello world");
    let _ = handle.join();
}