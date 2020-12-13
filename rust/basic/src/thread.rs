use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("hello world");
    });
    dbg!(handle.join());
}