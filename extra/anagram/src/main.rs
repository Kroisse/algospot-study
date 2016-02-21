/*
    https://algospot.com/judge/problem/read/ANAGRAM
*/
mod tests;

use std::io;

fn check(serial: &str, password: &str) -> bool {
    if serial.len() != password.len() {
        false
    } else if serial == password {
        false
    } else {
        let mut letters = [0i8; 128];
        for b in serial.bytes() {
            letters[b as usize] += 1;
        }
        for b in password.bytes() {
            letters[b as usize] -= 1;
            if letters[b as usize] < 0 {
                return false;
            }
        }
        letters.iter().all(|&x| x == 0)
    }
}

fn process(input: &mut io::BufRead, output: &mut io::Write) {
    let mut buf = String::with_capacity(80);
    // drop the first line
    input.read_line(&mut buf).unwrap();
    while { buf.clear(); input.read_line(&mut buf).unwrap() > 0 } {
        let mut splits = buf.trim().split(' ');
        let (serial, password) = (splits.next().unwrap(), splits.next().unwrap());
        let result = check(serial, password);
        writeln!(output, "{}", if result { "Yes" } else { "No." }).unwrap();
    }
}

fn main() {
    let stdin = io::stdin();
    process(&mut stdin.lock(), &mut io::stdout());
}
