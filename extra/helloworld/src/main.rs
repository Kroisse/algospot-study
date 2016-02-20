/*
    https://algospot.com/judge/problem/read/HELLOWORLD
 */
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::with_capacity(80);
    // drop the first line
    handle.read_line(&mut buf).unwrap();
    buf.clear();
    while handle.read_line(&mut buf).unwrap() > 0 {
        println!("Hello, {}!", buf.trim_right());
        buf.clear();
    }
}
