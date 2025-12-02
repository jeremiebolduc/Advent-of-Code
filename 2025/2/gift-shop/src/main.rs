use std::{
    fs::{self},
    io::{self},
};

fn main() {
    let ranges = read_file("input.txt").expect("file should be read");
    let mut sum = 0;

    for (lower, upper) in ranges {
        for invalid_id in find_invalid_ids(lower, upper) {
            sum += invalid_id;
        }
    }

    println!("{sum}");
}

fn read_file(path: &str) -> io::Result<Vec<(usize, usize)>> {
    let content = fs::read_to_string(path)?;
    let mut result = Vec::new();

    for range in content.split(',') {
        let mut splits = range.split('-');

        let lower = match splits.next() {
            Some(v) => {
                v.trim().parse::<usize>().expect("lower should be usize")
            },
            None => continue,
        };

        let upper = match splits.next() {
            Some(v) => {
                v.trim().parse::<usize>().expect("upper should be usize")
            },
            None => continue,
        };

        result.push((lower, upper));
    }

    Ok(result)
}

fn find_invalid_ids(lower: usize, upper: usize) -> Vec<usize> {
    (lower..=upper)
        .filter(|num| is_repeated_twice(num.to_string().as_str()))
        .collect()
}

fn is_repeated_twice(s: &str) -> bool {
    let n = s.len();
    if n % 2 != 0 {
        return false;
    }

    let half = n / 2;
    &s[..half] == &s[half..]
}
