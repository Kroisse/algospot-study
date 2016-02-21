/*
    https://algospot.com/judge/problem/read/HELLOWORLD

    - O(n)으로는 속도 제한을 감당할 수 없다.
    - 문제의 의도는 결국 어떤 구간 안에 모두 같은 숫자만 있는지를 알아보는려는 것.
    - 수열 전체를 탐색하는 대신, 수열에서 숫자가 바뀌는 지점의 인덱스만 저장해 두고 이걸 쓰자.

    - 속도 때문에 시간초과가 뜨는 게 아닌 것 같다.
    - 통계 페이지에서 푼 사람들 10페이지까지 가도 답안 내는 속도가 300ms를 안 넘음.
    - 가능한 최악의 테스트 케이스를 만들어 돌려 봐도 로컬에서 45ms +- 13ms 수준.
    - 입출력 오류일 가능성이 높음.
    - 일부러 이상한 답을 내는 코드를 작성. 586ms만에 오답.
    - 아니면 입출력이 오래 걸리는 걸까?
      - 테스트 추가해 봤는데 그렇진 않은 것 같음
    - 작정하고 무조건 오답만 내는 프로그램을 만들어 보자
      - 493ms만에 오답.
    - 데이타를 읽기만 하는데 500ms가 걸린다. 뭔가 문제가 있음.
    - 뭔가 웃긴 얘기지만 이 문제는 I/O bounded인 것 같다.
      - process()에서 임시 버퍼를 빼는 것으로 100ms 정도 성능 향상이 있었음
*/
#![feature(test)]
mod tests;
#[cfg(test)] extern crate rand;
#[cfg(test)] extern crate test;

use std::cmp;
use std::io::{self, BufRead};
use std::ops::Range;

fn determine(seq: &[u32], range: Range<u32>) -> bool {
    let start = seq.binary_search(&(range.start + 1)).unwrap_or_else(|idx| idx);
    let end = seq.binary_search(&range.end).unwrap_or_else(|idx| idx);
    start == end
}

fn parse_sequence(input: &mut BufRead) -> io::Result<Vec<u32>> {
    let mut seq = Vec::with_capacity(1_000_000);
    let mut current = b'0'; // 첫 바이트를 뭘로 생각하던 대세에 영향이 없음
    let mut index = 0;
    let mut read_count;
    'exit: loop {
        read_count = 0;
        {
            let buf = try!(input.fill_buf());
            if buf.len() == 0 { break 'exit; }
            for &b in buf {
                match b {
                    b'0' | b'1' => {
                        if current != b {
                            current = b;
                            seq.push(index);
                        }
                    }
                    _ => { break 'exit; }
                }
                index += 1;
                read_count += 1;
            }
        }
        input.consume(read_count);
    }
    input.consume(read_count);
    Ok(seq)
}

fn process<R: io::BufRead, W: io::Write>(input: &mut R, output: &mut W) {
    let sequence = parse_sequence(input).unwrap();
    consume(input, |b| b == b'\n').unwrap();

    let cases = read_u32(input).unwrap();
    consume(input, |b| b == b'\n').unwrap();

    for _ in 0..cases {
        let a = read_u32(input).unwrap();
        consume(input, |b| b == b' ').unwrap();
        let b = read_u32(input).unwrap();
        consume(input, |b| b == b'\n').unwrap();
        let (begin, last) = (cmp::min(a, b), cmp::max(a, b));
        let clean = determine(&sequence, begin..(last+1));
        writeln!(output, "{}", if clean { "Yes" } else { "No" }).unwrap();
    }
}

fn read_u32<R: io::BufRead>(read: &mut R) -> io::Result<u32> {
    let mut result: Option<u32> = None;
    let mut count;
    'a: loop {
        count = 0;
        {
            let buf = try!(read.fill_buf());
            if buf.len() == 0 {
                break 'a;
            }
            for &b in buf {
                result = match (b, result) {
                    (b'0' ... b'9', Some(r)) => Some(r * 10 + (b - b'0') as u32),
                    (b'0' ... b'9', None)    => Some((b - b'0') as u32),
                    _ => { break 'a; }
                };
                count += 1;
            }
        }
        read.consume(count);
    }
    read.consume(count);
    if let Some(r) = result {
        Ok(r)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "failed to read"))
    }
}

fn consume<'a, R: io::BufRead, F>(read: &mut R, mut pred: F) -> io::Result<usize> where F: FnMut(u8) -> bool + 'a {
    let mut count = 0;
    let mut c;
    'c: loop {
        c = 0;
        {
            let buf = try!(read.fill_buf());
            if buf.len() == 0 {
                break 'c;
            }
            for &b in buf {
                if pred(b) {
                    /* just pass it */
                } else {
                    break 'c;
                }
                c += 1;
            }
        }
        read.consume(c);
        count += c;
    }
    read.consume(c);
    Ok(count + c)
}

fn main() {
    let stdin = io::stdin();
    process(&mut stdin.lock(), &mut io::stdout());
}
