extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str;
use regex::Regex;

#[derive(Debug)]
struct Rect {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn main() -> std::io::Result<()> {
    println!("day 3");
    let mut input_buf = vec![];
    BufReader::new(File::open("src/bin/day03.txt")?).read_to_end(&mut input_buf)?;
    let input = input_buf.split(|i| *i == '\n' as u8).collect::<Vec<&[u8]>>();

    let mut rects = vec![];

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    for ref line in input {
        for cap in re.captures_iter(str::from_utf8(*line).unwrap()) {
            let rect = Rect {
                id: cap[1].parse().unwrap(),
                x: cap[2].parse().unwrap(),
                y: cap[3].parse().unwrap(),
                width: cap[4].parse().unwrap(),
                height: cap[5].parse().unwrap()
            };
            rects.push(rect);
        }
    }

    /*
    // calculate max size up front, easy enough.
    let mut x = 0;
    let mut y = 0;
    for rect in &rects {
        if rect.x + rect.width > x {
            x = rect.x + rect.width;
        }
        if rect.y + rect.height > x {
            y = rect.y + rect.height;
        }
    }
    println!("{},{}", x, y); => 999 x 998
    */

    let mut fabric: [i32; 1000*1000] = [0; 1000*1000];
    for rect in &rects {
        let i = rect.y * 1000 + rect.x;
        for h in 0..rect.height {
            for w in 0..rect.width {
                fabric[(i+h*1000+w) as usize] += 1;
            }
        }
    }

    let mut num_overlapped = 0;
    for inch in &fabric[..] {
        if *inch > 1 {
            num_overlapped += 1;
        }
    }

    println!("num overlapped inches: {}", num_overlapped);

    'rect: for rect in &rects {
        let i = rect.y * 1000 + rect.x;
        for h in 0..rect.height {
            for w in 0..rect.width {
                if fabric[(i+h*1000+w) as usize] != 1 {
                    continue 'rect;
                }
            }
        }
        println!("only non overlapping claim: {}", rect.id);
        break;
    }

    Ok(())
}