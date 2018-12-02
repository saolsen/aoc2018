use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("day 2");
    let f = File::open("src/bin/day02.txt")?;
    let mut reader = BufReader::new(f);
    let mut input_buf = vec![];
    reader.read_to_end(&mut input_buf)?;
    let input = input_buf.split(|i| *i == '\n' as u8).collect::<Vec<&[u8]>>();

    let mut num_2s = 0;
    let mut num_3s = 0;
    for ref line in input {
        let mut letter_counts: [i32; 26] = [0; 26];
        for letter in line.iter() {
            let i = (letter - 'a' as u8) as usize;
            letter_counts[i] += 1;
        }
        let mut hit_2 = false;
        let mut hit_3 = false;
        for i in 0..26 {
            let num_of_letter = letter_counts[i];
            if num_of_letter == 2 && !hit_2 {
                hit_2 = true;
                num_2s += 1;
                if hit_3 {
                    break;
                }
            } else if num_of_letter == 3 && !hit_3 {
                hit_3 = true;
                num_3s += 1;
                if hit_2 {
                    break;
                }
            }
        }   
    }
    println!("checksum: {}", num_2s * num_3s);
    Ok(())
}