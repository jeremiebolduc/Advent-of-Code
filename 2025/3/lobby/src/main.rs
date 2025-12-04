use std::{
    fs::File,
    io::{self, BufRead, BufReader},
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

fn find_max_joltage(bank: &[u8], n: usize) -> usize {
    let mut joltage = 0;
    let mut index = 0;
    for i in (0..=n - 1).rev() {
        let slice = &bank[index..bank.len() - i];
        let (max, rel) = find_max_in_slice(slice);
        index = index + rel + 1;
        joltage = joltage * 10 + max as usize;
    }

    joltage
}

fn find_max_in_slice(slice: &[u8]) -> (u8, usize) {
    let mut max = 0;
    let mut idx = 0;

    for (i, &n) in slice.iter().enumerate() {
        if n > max {
            max = n;
            idx = i;
        }
    }

    (max, idx)
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
