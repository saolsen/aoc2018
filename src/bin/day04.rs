use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str;
use regex::Regex;

use std::collections::HashMap;

#[derive(Debug)]
enum Action {
    GoesOnShift,
    FallsAsleep,
    WakesUp
}

#[derive(Debug)]
struct LogRecord {
    id: i32,
    minute: i32,
    action: Action
}

fn main() -> std::io::Result<()> {
    let records = {
        let mut input_buf = vec![];
        BufReader::new(File::open("src/bin/day04.txt")?).read_to_end(&mut input_buf)?;
        let mut input = input_buf.split(|i| *i == '\n' as u8).collect::<Vec<&[u8]>>();
        input.sort();

        let line_re = Regex::new(r"\[\d{4}-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)").unwrap();
        let id_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
        let mut guard_id: i32 = 0;

        let mut records = vec![];  
        
        for ref line in input {
            let cap = line_re.captures(str::from_utf8(*line).unwrap()).unwrap();
            let action = match &cap[5] {
                "falls asleep" => { Action::FallsAsleep },
                "wakes up" => { Action::WakesUp },
                _ => {
                    let id_cap = id_re.captures(&cap[5]).unwrap();
                    guard_id = id_cap[1].parse().unwrap();
                    Action::GoesOnShift
                }
            };
            let log_record = LogRecord {
                id: guard_id,
                minute: cap[4].parse().unwrap(),
                action
            };

            records.push(log_record);
        }  

        records  
    };
    
    let mut guard_total_mins_asleep: HashMap<i32, i32> = HashMap::new();
    let mut guard_specific_mins_asleep: HashMap<i32, [i32; 60]> = HashMap::new();

    let mut best_sleeper_id = 0;
    let mut best_sleeper_total_mins_asleep = 0;

    {
        let mut current_guard_id = 0;
        let mut fell_asleep_at = 0;

        for record in &records {
            match record.action {
                Action::FallsAsleep => {
                    fell_asleep_at = record.minute;
                },
                Action::WakesUp => {
                    let mins_asleep = record.minute - fell_asleep_at;
                    let current_guard_total_mins_asleep = guard_total_mins_asleep.entry(record.id).or_insert(0);
                    *current_guard_total_mins_asleep += mins_asleep;
                    let current_guard_specific_mins_asleep = guard_specific_mins_asleep.entry(record.id).or_insert([0; 60]);
                    for i in 0..mins_asleep {
                        let minute = fell_asleep_at + i;
                        current_guard_specific_mins_asleep[minute as usize] += 1;
                    }
                },
                Action::GoesOnShift => {
                    if let Some(current_guard_total_mins_asleep) = guard_total_mins_asleep.get(&current_guard_id) {
                        if *current_guard_total_mins_asleep > best_sleeper_total_mins_asleep {
                            best_sleeper_id = current_guard_id;
                            best_sleeper_total_mins_asleep = *current_guard_total_mins_asleep;
                        }
                    }
                    current_guard_id = record.id;
                }
            }
        }
    }

    let mut sleepiest_minute = -1;
    let mut time_asleep = 0;
    if let Some(best_sleeper_specific_mins_asleep) = guard_specific_mins_asleep.get(&best_sleeper_id) {
        for (i, v) in best_sleeper_specific_mins_asleep.iter().enumerate() {
            if *v > time_asleep {
                time_asleep = *v;
                sleepiest_minute = i as i32;
            }
        }
    }

    println!("[Strategy 1] Guard Id: {}, Time Asleep: {}, Sleepiest Minute: {}, Answer: {}", best_sleeper_id, best_sleeper_total_mins_asleep, sleepiest_minute, best_sleeper_id*sleepiest_minute);

    {
        let mut guard_id = 0;
        let mut longest_minute = 0;
        let mut longest_minute_num_nights = 0;
        for (id, specific_mins_asleep) in guard_specific_mins_asleep {
            for (minute, num_nights) in specific_mins_asleep.iter().enumerate() {
                if *num_nights > longest_minute_num_nights {
                    guard_id = id;
                    longest_minute = minute as i32;
                    longest_minute_num_nights = *num_nights;
                }
            }
        }

        println!("[Strategy 2] Guard Id: {}, Sleepiest Minute: {}, Num Nights Asleep On Minute: {}, Answer: {}", guard_id, longest_minute, longest_minute_num_nights, guard_id * longest_minute);
    }
    
    Ok(())
}