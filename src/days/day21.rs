use core::panic;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use crate::util::*;

#[derive(Clone, Debug, PartialEq)]
enum Plots {
    Rock,
    Garden,
    Start,
}

impl FromChar for Plots {
    fn default() -> Self {
        Self::Garden
    }
    fn from_char(char: &char) -> Self {
        match char {
            '#' => Self::Rock,
            '.' => Self::Garden,
            'S' => Self::Start,
            _ => panic!("invalid plot encountered!"),
        }
    }
}

impl MatrixElement for Plots {}

fn take_step(matrix: &Matrix<Plots>, positions: &HashSet<MatrixIdx>) -> HashSet<MatrixIdx> {
    positions
        .iter()
        .flat_map(|position| matrix.neighbour_idzs(position, |elem| elem != &Plots::Rock))
        .collect()
}

fn take_step_inf(matrix: &Matrix<Plots>, positions: &HashSet<MatrixIdx>) -> HashSet<MatrixIdx> {
    positions
        .iter()
        .flat_map(|position| {
            ALL_DIRECTIONS.iter().filter_map(|direction| {
                let idx = &matrix.next_unchecked(position, &direction);
                (matrix.get_wrapped(idx) != &Plots::Rock).then_some(*idx)
            })
        })
        .collect()
}
pub fn part1(input: &str) -> i64 {
    let matrix = Matrix::<Plots>::from_string(input);
    let mut positions: HashSet<MatrixIdx> = matrix.find(&Plots::Start).into_iter().collect();

    for _ in 0..64 {
        positions = take_step(&matrix, &positions);
    }
    positions.len() as i64
}
pub fn part2(input: &str) -> i64 {
    let matrix = Matrix::<Plots>::from_string(input);
    let mut front: HashSet<MatrixIdx> = matrix.find(&Plots::Start).into_iter().collect();
    let mut odd = HashSet::new();
    let mut ys = Vec::new();
    let mut xs = Vec::new();

    for step in 1..4097 {
        front = take_step_inf(&matrix, &front);
        if step % 2 == 1 {
            front = front.difference(&odd).copied().collect();
            odd.extend(front.iter());
            xs.push(step);
            ys.push(odd.len() as i32)
        }
        dbg!(step);
    }

    // Open or create a file for writing
    let mut file = File::create("output.txt").unwrap();

    // Integer values to write

    // Iterate over the integers and write each one to a new line in the file
    for (x,y) in xs.iter().zip(ys.iter()) {
        writeln!(file, "{}, {}", x, y).unwrap();
    }


    0
}
