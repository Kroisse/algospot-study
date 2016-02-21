use std::io::BufRead;

use super::{process};

const SAMPLE_DATA: &'static [(&'static str, &'static [&'static str])] = &[
    (
        "3
weird wired
apple angle
apple elppa
",
        &["Yes", "No.", "Yes"]
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
