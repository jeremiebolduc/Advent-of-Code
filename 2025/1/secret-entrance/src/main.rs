use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    let rotations = read_file("input.txt").expect("the file should be read");
    let mut zero_count = 0;
    let mut start = 50;

    for rotation in rotations {
        start = rotation.rotate(start);
        if start == 0 {
            zero_count += 1
        }
    }

    println!("{zero_count}");
}

enum Rotation {
    Left(usize),
    Right(usize),
}

impl Rotation {
    pub fn rotate(&self, start: usize) -> usize {
        match self {
            Rotation::Left(offset) => {
                let mut diff = start as i32 - *offset as i32;
                if diff < 0 {
                    while diff < 0 {
                        diff += 100;
                    }
                }
                return diff as usize;
            }
            Rotation::Right(offset) => {
                let mut sum = start + *offset;
                if sum >= 100 {
                    while sum >= 100 {
                        sum -= 100;
                    }
                }
                return sum;
            }
        }
    }
}

fn read_file(path: &str) -> io::Result<Vec<Rotation>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut rotations = Vec::new();
    for line in reader.lines() {
        let content = line?;
        let (direction, offset) = content.split_at(1);
        let offset = offset.parse::<usize>().expect("should be a usize");

        let rotation = match direction.to_uppercase().as_str() {
            "L" => Rotation::Left(offset), 
            "R" => Rotation::Right(offset),
            _ => continue, // skip bad input
        };
        rotations.push(rotation);
    }

    Ok(rotations)
}
