use std::{
    fs::File, io::{self, BufRead, BufReader}
};

fn main() {
    let mut ranges = read_file("input.txt").expect("file should be parsed");

    ranges.sort_by_key(|x| x.0);

    let merged = merge_ranges(ranges);

    let total: usize = merged.into_iter()
        .map(|(lower, upper)| upper + 1 - lower)
        .sum();

    println!("{total}");
}

fn merge_ranges(ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if ranges.is_empty() {
        return ranges;
    }

    let mut merged = Vec::new();
    let mut current = ranges[0];

    for &(lower, upper) in &ranges[1..] {
        if lower <= current.1 + 1 { 
            // overlap -> extend the range
            current.1 = current.1.max(upper);
        } else {
            merged.push(current);
            current = (lower, upper);
        }
    }

    merged.push(current);
    merged
}

fn read_file(path: &str) -> io::Result<Vec<(usize, usize)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut fresh = Vec::new();

    for line in reader.lines() {
        let content = line?;

        if content.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = content.split("-").collect();
        match parts.len() {
            2 => {
                let lower = parts[0].parse::<usize>().expect("should be a usize");
                let upper = parts[1].parse::<usize>().expect("should be a usize");

                fresh.push((lower, upper));
            }
            _ => continue,
        }
    }

    Ok(fresh)
}
