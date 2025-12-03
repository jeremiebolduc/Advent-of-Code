use std::{
    fs::File,
    io::{self, BufRead, BufReader}, usize,
};

fn main() {
    let banks = parse_file_into_banks("input.txt").expect("file should be parsed into banks");
    let mut sum: usize = 0;

    for bank in banks {
        let joltage = find_max_joltage(bank);
        sum += joltage;
    }

    println!("{sum}");
}

fn find_max_joltage(bank: Vec<u8>) -> usize {
    let mut max1: u8 = 0;
    let mut max1_index = 0;
    let slice1 = &bank[0..bank.len() - 1]; // to prevent capping it at 9
    for (&num, i) in slice1.iter().zip(0..bank.len() - 1) {
        if num > max1 {
            max1 = num;
            max1_index = i;
        }
    }

    let mut max2: u8 = 0;
    let slice2 = &bank[max1_index + 1..bank.len()];
    for &num in slice2 {
        if num > max2 {
            max2 = num;
        }
    }

    format!("{max1}{max2}").parse::<usize>().unwrap()
}

fn parse_file_into_banks(path: &str) -> io::Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut banks = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let bank: Vec<u8> = line.chars().filter_map(|num| {
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
