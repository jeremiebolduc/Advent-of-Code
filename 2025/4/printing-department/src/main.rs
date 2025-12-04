use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader},
};

use nalgebra::DMatrix;

fn main() {
    let symbols = read_file("input.txt").expect("file should be parsed");
    let directions = [
        [-1, -1], // NW
        [-1, 0],  // W
        [-1, 1],  // SW
        [0, -1],  // S
        [1, -1],  // SE
        [1, 0],   // E
        [1, 1],   // NE
        [0, 1],   // N
    ];

    let rows = symbols.nrows();
    let cols = symbols.ncols();

    let mut rolls_adjacency = DMatrix::<u8>::zeros(rows, cols);
    let mut removed = DMatrix::<bool>::from_element(rows, cols, false);
    let mut rolls_pos = Vec::new();

    // precompute rolls adjacency
    for i in 0..rows {
        for j in 0..cols {
            if let Symbol::PaperRoll = symbols[(i, j)] {
                rolls_adjacency[(i, j)] = get_adj_rolls(i, j, &symbols, &directions);
                rolls_pos.push((i, j))
            }
        }
    }

    // move removable into a queue for BFS and mark them as removed
    let mut queue = VecDeque::new();
    for (i, j) in rolls_pos {
        if rolls_adjacency[(i, j)] < 4 {
            queue.push_back((i, j));
            removed[(i, j)] = true;
        }
    }

    let mut removed_count = 0;

    // BFS
    while let Some((i, j)) = queue.pop_front() {
        removed_count += 1;

        for [dx, dy] in &directions {
            let ni = i as isize + dx;
            let nj = j as isize + dy;

            if ni >= 0 && ni < rows as isize && nj >= 0 && nj < cols as isize {
                let (ni, nj) = (ni as usize, nj as usize);

                if let Symbol::PaperRoll = symbols[(ni, nj)]
                    && !removed[(ni, nj)]
                {
                    rolls_adjacency[(ni, nj)] -= 1;

                    if rolls_adjacency[(ni, nj)] < 4 {
                        removed[(ni, nj)] = true;
                        queue.push_back((ni, nj));
                    }
                }
            }
        }
    }

    println!("{removed_count}");
}

fn get_adj_rolls(
    i: usize,
    j: usize,
    symbols: &DMatrix<Symbol>,
    directions: &[[isize; 2]; 8],
) -> u8 {
    let mut paper_roll_count = 0;

    for [dx, dy] in directions {
        let ni = i as isize + dx;
        let nj = j as isize + dy;
        if ni >= 0 && ni < symbols.nrows() as isize && nj >= 0 && nj < symbols.ncols() as isize {
            let (ni, nj) = (ni as usize, nj as usize);

            if let Symbol::PaperRoll = symbols[(ni, nj)] {
                paper_roll_count += 1;
            }
        }
    }

    paper_roll_count
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Symbol {
    PaperRoll,
    Empty,
}

fn read_file(path: &str) -> io::Result<DMatrix<Symbol>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut rows: Vec<Vec<Symbol>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut row = Vec::with_capacity(line.len());

        for ch in line.chars() {
            let symbol = match ch {
                '@' => Symbol::PaperRoll,
                '.' => Symbol::Empty,
                _ => panic!("unsupported input character {ch}"),
            };
            row.push(symbol);
        }

        rows.push(row);
    }

    if rows.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "file is empty"));
    }

    let cols_count = rows[0].len();
    let rows_count = rows.len();

    for (i, row) in rows.iter().enumerate() {
        if row.len() != cols_count {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Row {} has inconsistent length: expected {}, got {}",
                    i,
                    cols_count,
                    row.len()
                ),
            ));
        }
    }

    let flat: Vec<Symbol> = rows.into_iter().flatten().collect();

    let matrix = DMatrix::<Symbol>::from_row_slice(rows_count, cols_count, &flat);

    Ok(matrix)
}
