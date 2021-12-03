use crate::print_sudoku::print_json_set;
use num_cpus;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;

mod print_sudoku;
mod solver;

const USAGE_MSG: &'static str = "Usage: ./$PROG puzzlesPerThread minDifficulty maxDifficulty";

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4, "{}", USAGE_MSG);
    let iterations_per_thread = args[1].parse::<usize>().expect(USAGE_MSG);
    let min_diff = args[2].parse::<u32>().expect(USAGE_MSG);
    let max_diff = args[3].parse::<u32>().expect(USAGE_MSG);

    let threads = num_cpus::get();

    let start = std::time::Instant::now();
    let mut solutions: Vec<SudokuSolution> = Vec::with_capacity(threads * iterations_per_thread);

    let (tx, rx) = mpsc::channel::<SudokuSolution>();

    for _ in 0..num_cpus::get() {
        let tx_inner = tx.clone();
        thread::spawn(move || {
            for iteration_num in 0..iterations_per_thread {
                use solver::{create_puzzle_from_complete_grid, generate_filled_grid};

                let difficulty = min_diff
                    + ((max_diff - min_diff) as f32
                        * (iteration_num as f32 / iterations_per_thread as f32))
                        .floor() as u32;
                let solution = generate_filled_grid();
                let puzzle = create_puzzle_from_complete_grid(solution.clone(), difficulty);
                let num_clues = puzzle.iter().filter(|el| **el != 0).count();

                tx_inner
                    .send(SudokuSolution {
                        puzzle,
                        solution,
                        num_clues,
                    })
                    .expect("Failed to send solution to main thread");
            }
        });
    }

    // No messages will be sent by the original sender.
    drop(tx);

    loop {
        if let Ok(puzzle) = rx.recv() {
            solutions.push(puzzle);
        } else {
            break;
        }
    }

    dedup(&mut solutions);

    solutions.sort_by(|a, b| a.num_clues.cmp(&b.num_clues));

    let mut file = File::create("sudoku-puzzles-out.json").expect("Failed to open output file");
    file.write_all(print_json_set(&solutions).as_bytes()).expect("Failed to write file output");

    println!(
        "Generated {} unique puzzles in {}ms",
        solutions.len(),
        start.elapsed().as_millis()
    );
}

fn dedup(solutions: &mut Vec<SudokuSolution>) {
    let mut uniques = HashSet::new();
    solutions.retain(|el| uniques.insert(el.puzzle));
}

pub struct SudokuSolution {
    num_clues: usize,
    puzzle: [u8; 81],
    solution: [u8; 81],
}
