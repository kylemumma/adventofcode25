use std::fs::File;
use std::io::{self, BufRead};

fn parse_rotation(rotation: &str) -> Result<(char, i32), String> {
    let mut chars = rotation.chars();
    let prefix = chars.next().ok_or("bad str")?;
    let number = chars.as_str().parse::<i32>().map_err(|_| "bad str")?;
    Ok((prefix, number))
}

fn rotate(start: i32, prefix: char, mut rotation: i32) -> (i32, i32) {
    if prefix == 'L' {
        rotation *= -1
    }
    let rotated = start + rotation;
    let mut passes = 0;
    if rotated == 0 {
        passes = 1;
    } else if rotated > 99 {
        passes = rotated/100;
    } else if rotated < 0 {
        passes = 1+(-1*(rotated/100));
        if start == 0 {
            passes -= 1;
        }
    }
    (rotated.rem_euclid(100), passes)
}

pub fn run() {
    let mut ptr = 50;
    let mut ctr = 0;

    let file = File::open("src/day1/input.txt").expect("couldnt open file");
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("couldnt read line");
        let (dir, delta) = parse_rotation(&line).expect("failed to parse line");
        let (newptr, ct) = rotate(ptr, dir, delta);
        ptr = newptr;
        ctr += ct;
    }
    println!("{:?}", ctr);
}
