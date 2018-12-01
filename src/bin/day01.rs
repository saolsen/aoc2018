use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashSet;

// Doing it a low level way that's closer to how  I'll do it
// in stee.
fn main() -> std::io::Result<()> {
    let f = File::open("src/bin/day01.txt")?;
    let mut reader = BufReader::new(f);
    let mut input_buf = vec![];
    reader.read_to_end(&mut input_buf)?;
    let input = input_buf.split(|i| *i == '\n' as u8).collect::<Vec<&[u8]>>();

    let mut seen_frequencies = HashSet::new();
    let mut frequency : i32 = 0;
    'outer: loop {
        for ref line in &input {
            if line.len() > 0 {
                let mut n : i32 = 0;
                for digit in &line[1..] {
                    n *= 10;
                    n += (digit - '0' as u8) as i32;
                }
                let sign = line[0] as char;
                if sign == '+' {
                    frequency += n;
                } else {
                    frequency -= n;
                }
                if !seen_frequencies.insert(frequency) {
                    break 'outer;
                }
            }
        }
    }
    println!("First Repeat: {}", frequency);
    Ok(())
}