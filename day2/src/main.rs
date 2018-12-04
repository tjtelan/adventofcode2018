use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

#[derive(Debug)]
struct CandidateBox {
    id : String,
    two_letter_case : bool,
    three_letter_case : bool,
}

impl CandidateBox {
    fn new(id : &str) -> CandidateBox {
        let (two_letter_case, three_letter_case) = CandidateBox::_check_letter_cases(id);

        CandidateBox {
            id : id.to_string(),
            two_letter_case : two_letter_case,
            three_letter_case : three_letter_case,
        }
    }

    fn _check_letter_cases(id : &str) -> (bool, bool) {
        let s_slice : &str = &id[..];
        let mut chars : Vec<char> = s_slice.chars().collect();
        chars.sort();

        let mut two_flag = false;
        let mut three_flag = false;

        for n in chars {

            if &id.matches(n).count() == &2 {
                two_flag = true;
            }

            if &id.matches(n).count() == &3 {
                three_flag = true;
            }

            //println!("{} found {} times", n, &id.matches(n).count());
        }

        (two_flag, three_flag)
    }

    fn has_two_letter_case(&self) -> bool {
        self.two_letter_case
    }

    fn has_three_letter_case(&self) -> bool {
        self.three_letter_case
    }
}


fn main() -> Result<()> {
    // Part one
    let path = Path::new("day2-input");
    //let path = Path::new("test-input");
    //let path = Path::new("test2-input");
    let mut file = File::open(&path)?;

    // Read in the file, and convert into a Vec<i32> so we can
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    drop(file);
    let box_ids : Vec<String> = file_contents.split_whitespace()
        .map(|s: &str| s.to_string())
        .collect();

    println!("{:?}", box_ids);

    let mut two_letter_case_count = 0;
    let mut three_letter_case_count = 0;

    for id in &box_ids {
        // This will characterize the ids
        let new_box = CandidateBox::new(id);

        // Now count the number of two cases, and multiply by number of three cases
        if new_box.has_two_letter_case() {
            two_letter_case_count += 1;
        }

        if new_box.has_three_letter_case() {
            three_letter_case_count += 1;
        }

    }

    let checksum = two_letter_case_count * three_letter_case_count; // Part 1 answer
    println!("Checksum: {:?}", checksum);


    let mut box1 = "";
    let mut box2 = "";
    let mut most_common_count = 0;

    // Ugly O(n^2) solution
    for outer in &box_ids {

        for inner in &box_ids {
            if outer == inner {
                continue;
            }

            let mut iter = outer.chars().zip(inner.chars());

            let mut char_count = 0;
            for check in iter {
                //println!("{:?}", check);

                if check.0 == check.1 {
                    char_count += 1;
                }

            }

            if char_count > most_common_count {
                most_common_count = char_count;
                box1 = outer;
                box2 = inner;
            }

        }
    }

    println!("box1: {:?}\nbox2: {:?}", box1, box2);

    Ok(())
}

