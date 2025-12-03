use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    let rotations = read_file("input.txt").expect("the file should be read");
    let mut zeros = 0;
    let mut start = 50;

    for rotation in rotations {
        let (pos, clicks) = rotation.rotate(start);
        start = pos;
        zeros += clicks
    }

    println!("{zeros}");
}

enum Rotation {
    Left(usize),
    Right(usize),
}

impl Rotation {
    pub fn rotate(&self, start: usize) -> (usize, usize) {
        let mut pos = start;
        let mut zeros = 0;

        match self {
            Rotation::Left(k) => {
                for _ in 0..*k {
                    pos = (pos + 99) % 100;
                    if pos == 0 {
                        zeros += 1;
                    }
                }
            }
            Rotation::Right(k) => {
                for _ in 0..*k {
                    pos = (pos + 1) % 100;
                    if pos == 0 {
                        zeros += 1;
                    }
                }
            }
        }

        (pos, zeros)
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
