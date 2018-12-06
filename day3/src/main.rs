use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

use std::io::Error;
use std::str::FromStr;


#[derive(Debug,Clone,Copy)]
struct Claim {
    claim_id : usize,
    l_offset : usize,
    t_offset : usize,
    width : usize,
    height : usize,
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {

        let c : Vec<&str> = s.split(|c| c == '#' ||
                                    c == ' ' ||
                                    c == '@' ||
                                    c == ',' ||
                                    c == ':' ||
                                    c == 'x').filter(|x| x.len() > 0).collect();

        //println!("{:?}", c[0].parse::<usize>().ok().unwrap());

        Ok(
            Claim {
                claim_id : c[0].parse::<usize>().ok().unwrap(),
                l_offset : c[1].parse::<usize>().ok().unwrap(),
                t_offset : c[2].parse::<usize>().ok().unwrap(),
                width : c[3].parse::<usize>().ok().unwrap(),
                height : c[4].parse::<usize>().ok().unwrap(),
            }
        )
    }
}

// return the number of squares that conflict
fn mark_claim(fabric : &mut Vec<Vec<char>>, claim : &Claim) -> usize {
    let x_start = claim.l_offset;
    let y_start = claim.t_offset;
    let width = claim.width;
    let height = claim.height;

    let mut conflict = 0;

    for x in (x_start)..(x_start+width) as usize {
        for y in (y_start)..(y_start+height) as usize {

            if fabric[y][x] != '.' {
                if fabric[y][x] != 'x' {
                    fabric[y][x] = 'x';
                    conflict += 1;
                }
            }
            else {
                fabric[y][x] = '#';
            }
        }
    }

    //println!("{:?}", claim);
    //for line in fabric {
    //    println!("{:?}", line);
    //}

    conflict
}

// Return the claims that are completely intact
fn verify_claims(fabric : &Vec<Vec<char>>, claims : &Vec<Claim>) -> Vec<Claim> {
    let mut perfect_claims : Vec<Claim> = Vec::new();

    'claim: for claim in claims {
        let x_start = claim.l_offset;
        let y_start = claim.t_offset;
        let width = claim.width;
        let height = claim.height;

        for x in (x_start)..(x_start+width) as usize {
            for y in (y_start)..(y_start+height) as usize {
                if fabric[y][x] != '#' {
                    continue 'claim;
                }
            }
        }
        perfect_claims.push(*claim);
    }

    perfect_claims
}

fn main() -> Result<()> {
    let path = Path::new("day3-input");
    //let path = Path::new("test1-input");
    let mut file = File::open(&path)?;

    // Read in the file, and convert into a Vec<i32> so we can
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    drop(file);
    let claim_list : Vec<String> = file_contents.lines()
        .map(|s: &str| s.to_string())
        .collect();

    const FABRIC_SIZE_SQ : usize = 1000;
    //const FABRIC_SIZE_SQ : usize = 8;
    let mut fabric_sq = vec![vec!['.'; FABRIC_SIZE_SQ]; FABRIC_SIZE_SQ];

    //println!("{:?}", fabric_sq);

    let mut conflicts = 0;
    let mut claim_vec : Vec<Claim> = Vec::new();
    for c in claim_list {
        //println!("{:?}", &c.parse::<Claim>());
        let claim = &c.parse::<Claim>().unwrap();
        claim_vec.push(*claim);
        conflicts += mark_claim(&mut fabric_sq, &claim);
    }

    println!("There were {} conflicts", conflicts);

    //for line in fabric_sq {
    //    println!("{:?}", line);
    //}
    //
    let perfect_claims = verify_claims(&fabric_sq, &claim_vec);

    println!("{} Perfect claims: {:?}", perfect_claims.len(), perfect_claims);

    Ok(())
}
