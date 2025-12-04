use std::io;
use std::fs;

mod day1;
mod day2;

fn get_possible_days() {
    let paths = fs::read_dir("src/").expect("");
    for path in paths {
        let entry = path.expect("");
        if entry.file_type().expect("").is_dir() {
            println!("found folder: {}", entry.path().display());
        }
    }
}

fn main() {
    get_possible_days();
    println!("Which day would you like to run?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let cleaned = input.trim();
    if cleaned == "1" {
        day1::day1::run();
    } else if cleaned == "2" {
        day2::day2::run();
    } else {
        println!("nope:{:?}",cleaned);
    }
    
}
