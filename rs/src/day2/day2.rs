use std::fs::File;
use std::io::{BufRead,BufReader};

pub fn read_file(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.split(b',').map(|e| String::from_utf8(e.unwrap()).unwrap().trim().to_string())
}

pub fn run() {
    let mut ctr: u64 = 0;
    for item in read_file("src/day2/input.txt") {
        let parts: Vec<u64> = item.split('-').map(|e| e.parse().unwrap()).collect();
//        println!("will check {} to {}", parts[0], parts[1]);
        for i in parts[0]..(parts[1]+1) {
            let chars: Vec<char> = i.to_string().chars().collect();
            if chars.len() % 2 != 0 {
                continue;
            }
            let mid = chars.len() / 2;
            if &chars[..mid] == &chars[mid..] {
                ctr += i;
//                println!("{:?}", chars);
            }

        }
    }
    println!("{}", ctr);
}
