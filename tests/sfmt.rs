use rand_core::{RngCore, SeedableRng};
use sfmt::*;
use std::{fs, io, io::BufRead};

fn read_answer(filename: &str) -> Result<Vec<u64>, io::Error> {
    let f = io::BufReader::new(fs::File::open(filename)?);
    Ok(f.lines()
        .map(|line| {
            line.unwrap()
                .parse::<u64>()
                .expect("Failed to parse as u64")
        })
        .collect())
}

#[test]
fn compare_to_original_19937() {
    // This crate only works on x86/x86_64, which must be Little Endition
    let mut rng = SFMT::from_seed(1234_u32.to_le_bytes());
    let answer = read_answer("check/u64_19937.txt").unwrap();
    for ans in answer {
        let r = rng.next_u64();
        assert_eq!(r, ans);
    }
}
