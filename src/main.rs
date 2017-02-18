use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

fn foo(a: &mut i32) -> i32 {
    *a = *a + 1;
    *a + 1
}
fn main() {
    let threads = 100;
    let x = &mut 42;
    let y = foo(x);
    println!("Hello, world! {} {}", x, y);
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    for i in 0..threads {
        let tx_t = tx.clone();
        thread::spawn(move || {
            tx_t.send(i).unwrap();
            println!("this is thread number {}", i)
        });
    }
    for _ in 0..threads {
        let x = match rx.recv() {
            Ok(num) => num,
            Err(_) => -1,
        };
        println!("{}", x);
    }
    println!("done")
}
