use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// Doing it a low level way that's closer to how  I'll do it
// in stee.
fn main() -> std::io::Result<()> {
    let f = File::open("src/bin/day01.txt")?;
    let mut reader = BufReader::new(f);
    
    let mut frequency : i32 = 0;
    for maybe_line in reader.split(b'\n') {
        if let Ok(line) = maybe_line {
            let mut n : i32 = 0;
            for digit in &line[1..] {
                n *= 10;
                n += (*digit - '0' as u8) as i32;
            }
            let sign = line[0] as char;
            match sign {
                '+' => frequency += n,
                '-' => frequency -= n,
                _ => {}
            }
        }
    }
    println!("{}", frequency);

    Ok(())
}


