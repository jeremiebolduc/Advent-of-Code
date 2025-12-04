use std::{fs::File, io::{self, Read}};

fn main() {
    let directions = read_file("input.txt").expect("file should be parsed to directions");

    let mut floor: i32 = 0;
    let mut first_basement_index = 0;

    for (index, dir) in directions.iter().enumerate(){
        match dir {
            Direction::Up => {
                floor += 1;
            },
            Direction::Down => {
                floor -= 1;
            },
        }

        if floor == -1 {
            first_basement_index = index + 1;
            break;
        }
    }

    println!("{first_basement_index}");
}

enum Direction {
    Up,
    Down,
}

fn read_file(path: &str) -> io::Result<Vec<Direction>> {
    let mut file = File::open(path)?;
    let mut content = String::from("");
    file.read_to_string(&mut content)?;
    
    let mut directions = Vec::with_capacity(content.len());

    for ch in content.chars() {
        let dir = match ch {
            '(' => Direction::Up,
            ')' => Direction::Down,
            _ => continue, // don't handle invalid chars
        };
        directions.push(dir);
    }

    Ok(directions)
}
