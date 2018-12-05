use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut input = vec![];
    BufReader::new(File::open("src/bin/day05.txt")?).read_to_end(&mut input)?;
    assert_eq!(input.len() % 2, 0);

    let mut min_len = std::usize::MAX;
    for polymer in 65..91 {
        let mut units = input.clone();
        let mut new_units = vec![];
        for i in units {
            if i == polymer || i == polymer + 32u8 {
                continue;
            }
            new_units.push(i);
        }
        units = new_units;
        assert_eq!(units.len() % 2, 0);

        let mut stack = vec![];
        for c in units {
            if let Some(top) = stack.last() {
                if (*top as i32 - c as i32).abs() == 32 {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c);
        }

        println!("Reaction Without {}{}, length: {}", polymer as char, (polymer + 32u8) as char, stack.len());
        if stack.len() < min_len {
            min_len = stack.len();
        }
    }
    assert_eq!(min_len, 6946);
    Ok(())
}