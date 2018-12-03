use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use std::collections::HashSet;

fn main() -> Result<()> {
    // Part one
    let path = Path::new("day1-input");
    let display = path.display();
    let mut file = File::open(&path)?;

    let mut resulting_frequency = 0;
    let mut frequency_history : HashSet<i32>= HashSet::new();
    let mut frequency_dupes : Vec<i32> = Vec::new();
    let mut count = 0;

    // Read in the file, and convert into a Vec<i32> so we can
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    drop(file);
    let period : Vec<i32> = file_contents.split("\n")
        .filter_map(|s: &str| s.to_string().parse::<i32>().ok())
        .collect();

    // Looping once like this, so we can get the answer to part 1
    for f_delta in &period {
        count += 1;
        resulting_frequency += f_delta;

        if !frequency_history.insert(resulting_frequency.clone()) {
            frequency_dupes.push(resulting_frequency.clone());
            break;
        }
    }
    println!("Resulting frequency, for part1: {:?}", &resulting_frequency);

    // Hack
    while frequency_dupes.len() == 0 {
        count += 1;

        for f_delta in &period {
            resulting_frequency += f_delta;
    
            if !frequency_history.insert(resulting_frequency.clone()) {
                frequency_dupes.push(resulting_frequency.clone());
                break;
            }
        }

    }

    println!("First duplicate frequency: {:?}", frequency_dupes.pop().unwrap());
    println!("Length of frequency history: {:?}", frequency_history.len());
    println!("Looped #{:?} times before finding duplicate", count);
    Ok(())
}
