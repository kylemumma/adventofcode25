fn find_max_from(arr: &Vec<u32>, start: usize, end: usize) -> usize {
    let mut maxsofar = None;
    for i in start..end {
        let curr = arr[i];
        if let Some(max_idx) = maxsofar {
            if curr > arr[max_idx] {
                maxsofar = Some(i);
            }
        } else {
            maxsofar = Some(i);
        }
    }
    return maxsofar.unwrap();
}

fn get_max_2d(bank: &Vec<u32>) -> (usize, usize) {
    let left = find_max_from(bank, 0, bank.len() - 1);
    let right = find_max_from(bank, left + 1, bank.len());
    return (left, right);
}

pub fn run() {
    let data = include_str!("input.txt");
    let mut running_tot = 0;
    for line in data.trim().split("\n") {
        let bank_digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        println!("{:?}", bank_digits);
        let idx = get_max_2d(&bank_digits);
        println!("{}{}", bank_digits[idx.0], bank_digits[idx.1]);
        println!();
        running_tot += format!("{}{}", bank_digits[idx.0], bank_digits[idx.1])
            .parse::<u32>()
            .unwrap();
    }
    println!("{}", running_tot);
}
