use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("day 2 part 2");
    let f = File::open("src/bin/day02.txt")?;
    let mut reader = BufReader::new(f);
    let mut input_buf = vec![];
    reader.read_to_end(&mut input_buf)?;
    let input = input_buf.split(|i| *i == '\n' as u8).collect::<Vec<&[u8]>>();

    for (li, ref line1) in (&input).iter().enumerate() {
        for line2 in (&input).iter().skip(li+1) {
            let mut hd = 0;
            let mut differing_index = -1;
            assert!(line1.len() == line2.len());
            for i in 0..line1.len() {
                if line1[i] != line2[i] {
                    differing_index = i as i32;
                    hd += 1;
                }
            }
            if hd == 1 {
                for i in 0..line1.len() {
                    if i as i32 != differing_index {
                        print!("{}", line1[i] as char);
                    }
                }
                print!("\n");
                return Ok(());
            }
        }
    }
    Ok(())
}