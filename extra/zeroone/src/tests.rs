#![cfg(test)]

use std::cmp;
use std::io::BufRead;

use rand::{self, Rng};
use rand::distributions::{IndependentSample, Range};
use test::{Bencher, black_box};

use super::{determine, parse_sequence, process};

const SAMPLE_DATA: &'static [(&'static str, &'static [&'static str])] = &[
    (
        "0000011111
3
0 5
4 2
5 9",
        &["No", "Yes", "Yes"]
    ),
];

#[test]
fn full_test() {
    for &(input, expected) in SAMPLE_DATA {
        let mut output = Vec::new();
        process(&mut input.as_bytes(), &mut output);
        let result: Vec<String> = (&output[..]).lines().map(|line| line.unwrap().trim_right().to_owned()).collect();
        assert_eq!(result, expected);
    }
}

#[test]
fn test_simple() {
    let buf = b"0000011111";
    let seq = parse_sequence(&mut &buf[..]).unwrap();
    assert_eq!(determine(&seq, 0..6), false);
    assert_eq!(determine(&seq, 2..5), true);
    assert_eq!(determine(&seq, 5..10), true);
}

#[bench]
fn bench_parse_sequence(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let seq: String = (0..1_000_000).map(|_| if rng.gen() { '1' } else { '0' }).collect();
    b.iter(|| {
        black_box(parse_sequence(&mut seq.as_ref()).unwrap());
    });
}

#[bench]
fn bench_near_worst_case(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let seq: Vec<_> = (1 .. 1_000_000 - 1).collect();
    let seq_range = Range::new(0, 100_000);
    let questions: Vec<_> = (0..1_000_100).map(|_| {
        let a = seq_range.ind_sample(&mut rng);
        let b = seq_range.ind_sample(&mut rng);
        (cmp::min(a, b), cmp::max(a, b))
    }).collect();

    b.iter(|| {
        for &(a, b) in &questions {
            black_box(determine(&seq, a..b));
        }
    });
}
