use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    usize,
};

fn main() {
    let banks = parse_file_into_banks("input.txt").expect("file should be parsed into banks");
    let mut sum = 0;

    for bank in banks {
        let joltage = find_max_joltage(&bank, 12);
        sum += joltage;
    }

    println!("{sum}");
}

fn find_max_joltage(bank: &Vec<u8>, n: usize) -> u128 {
    let mut maxs = Vec::new();
    let mut max_index = 0;
    for i in (0..=n-1).rev() {
        let slice = &bank[max_index..bank.len() - i];
        let (max, index) = find_max_in_slice(max_index, bank.len() - i, slice);
        maxs.push(max);
        max_index = index + 1;
    }

    maxs.iter()
        .fold(String::from(""), |acc, max| format!("{acc}{max}"))
        .parse::<u128>()
        .unwrap()
}

fn find_max_in_slice(lower: usize, upper: usize, slice: &[u8]) -> (u8, usize) {
    let mut max: u8 = 0;
    let mut max_index = lower;
    for (&num, index) in slice.iter().zip(lower..upper) {
        if num > max {
            max = num;
            max_index = index;
        }
    }

    (max, max_index)
}

fn parse_file_into_banks(path: &str) -> io::Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut banks = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let bank: Vec<u8> = line
            .chars()
            .filter_map(|num| {
                if !num.is_numeric() {
                    return None;
                }

                num.to_digit(10).map(|parsed| {
                    parsed as u8 // should never overflow because it's a single digit
                })
            })
            .collect();

        banks.push(bank);
    }

    Ok(banks)
}
