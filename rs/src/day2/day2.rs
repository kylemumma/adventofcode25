use std::fs::File;
use std::io::{BufRead,BufReader};

pub fn read_file(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.split(b',').map(|e| String::from_utf8(e.unwrap()).unwrap().trim().to_string())
}

fn get_divisors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    for i in 1..=n.isqrt() {
        if n % i == 0 {
            divisors.push(i);
            if i != (n/i) {
                divisors.push(n/i);
            }
        }        
    }
    divisors
}

fn is_made_of(s: &str, subs: &str) -> bool {
    // assuming utf-8 strings that are size divisible
    let mut start = 0;
    let mut end = subs.len();
    while start < s.len() {
        if &s[start..end] != subs {
            return false;
        }
        start = end;
        end = start + subs.len();
    }
    true
}

pub fn run() {
    let mut ctr: u64 = 0;
    let mut newctr: u64 = 0;
    for item in read_file("src/day2/sample.txt") {
        let parts: Vec<u64> = item.split('-').map(|e| e.parse().unwrap()).collect();
//        println!("will check {} to {}", parts[0], parts[1]);
        for i in parts[0]..(parts[1]+1) {
            let istr = i.to_string();
            let divisors = get_divisors(istr.len() as u64);
            let mut found = false;
            for d in divisors {
                if is_made_of(&istr, &istr[0..(d as usize)]) {
                    found = true;
                    break;
                }
            }
            if found {
                newctr += i;
            }
            // old code:
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
    println!("{}", newctr);
}
