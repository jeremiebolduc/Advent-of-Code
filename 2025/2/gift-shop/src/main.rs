use std::{
    cmp::min,
    fs::{self},
    io::{self},
    iter::Filter,
    ops::RangeInclusive,
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
            Some(v) => v.trim().parse::<usize>().expect("lower should be usize"),
            None => continue,
        };

        let upper = match splits.next() {
            Some(v) => v.trim().parse::<usize>().expect("upper should be usize"),
            None => continue,
        };

        result.push((lower, upper));
    }

    Ok(result)
}

fn find_invalid_ids(
    lower: usize,
    upper: usize,
) -> Filter<RangeInclusive<usize>, impl FnMut(&usize) -> bool> {
    (lower..=upper).filter(|num| is_repeated_at_least_twice(num.to_string().as_str()))
}

fn is_repeated_at_least_twice(s: &str) -> bool {
    let mut pattern = String::from("");
    for current in s.chars() {
        pattern.push(current);
        let mut matches = 0;
        loop {
            let lower = min(pattern.len() * (1 + matches), s.len());
            let upper = min(pattern.len() * (2 + matches), s.len());
            let immediate = &s[lower..upper];

            if matches > 0 && immediate.is_empty() {
                return true;
            }

            if pattern != immediate {
                break;
            }

            matches += 1;
        }
    }

    false
}
