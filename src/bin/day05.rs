use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str;

use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut input = vec![];
    BufReader::new(File::open("src/bin/day05.txt")?).read_to_end(&mut input)?;
    assert_eq!(input.len() % 2, 0);

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

        let mut reactions = true;
        while (reactions) {
            reactions = false;
            if units.len() < 2 {
                break;
            }

            let mut next_units = vec![];
            let mut i = 0;

            while i < units.len() - 1 {
                let a = units[i];
                let b = units[i+1];
                
                if (a as i32 - b as i32).abs() == 32 {
                    reactions = true;
                    i += 2;
                } else {
                    next_units.push(a);
                    i += 1;
                }
            }
            if i == units.len() - 1 {
                next_units.push(units[i]);
            }
            units = next_units;
        }
        println!("Reaction Without {}{}, length: {}", polymer as char, (polymer + 32u8) as char, units.len());
    }
    Ok(())
}