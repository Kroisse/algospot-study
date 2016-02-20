#![cfg(test)]

use std::io::BufRead;

use super::{calculate, process};

const SAMPLE_DATA: &'static [(&'static str, &'static [f64])] = &[
    (
        "2
6 3
1 2 3 1 2 3
6 2
1 2 3 1 2 3
",
        &[1.75000000000, 1.50000000000]
    ),
];

#[test]
fn full_test() {
    for &(input, expected) in SAMPLE_DATA {
        let mut output = Vec::new();
        process(&mut input.as_bytes(), &mut output);
        let output: Vec<f64> = (&output[..]).lines().map(|line| line.unwrap().trim().parse().unwrap()).collect();
        assert_eq!(output, expected);
    }
}

#[test]
fn test_edges() {
    assert_eq!(calculate(&[1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3], 3), 1.75);
    assert_eq!(calculate(&[1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3], 2), 1.5);
    assert_eq!(calculate(&[1, 2, 3, 1, 2, 3], 1), 1.);
    assert_eq!(calculate(&[1], 1), 1.);
    assert_eq!(calculate(&[1, 1, 1, 1, 1], 1), 1.);
    assert_eq!(calculate(&[1, 1, 1, 1, 1], 3), 1.);
    assert_eq!(calculate(&[1, 1, 1, 1, 1], 5), 1.);
}
