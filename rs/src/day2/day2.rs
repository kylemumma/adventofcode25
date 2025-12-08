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
            if i <= n/2 {
                divisors.push(i);
            }
            if i != (n/i) && (n/i) <= n/2 {
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
    //println!("{}", is_made_of("565656", "56"));
    //return;
    let mut ctr: u64 = 0;
    let mut newctr: u64 = 0;
    for item in read_file("src/day2/input.txt") {
        let parts: Vec<u64> = item.split('-').map(|e| e.parse().unwrap()).collect();
        println!("{}-{}", parts[0], parts[1]);
        for i in parts[0]..(parts[1]+1) {
        //for i in 565656..565657 {
            let istr = i.to_string();
            let divisors = get_divisors(istr.len() as u64);
            let div2 = divisors.clone();
            
            //println!("{}: {} divisors {:?}", istr, istr.len(), div2);
            let mut found = false;
            for d in divisors {
                let arg1 = &istr;
                let arg2 = &istr[0..(d as usize)];
                //println!("{}, {}", arg1, arg2);
                let res = is_made_of(arg1, arg2);
                if res {
                    
                    println!("{:?} is made of {:?}",arg1,arg2);
                    found = true;
                    break;
                }
            }
            if found {
                newctr += i;
            }
        }
        println!();
    }
    println!("{}", newctr);
}
