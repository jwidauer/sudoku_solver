use anyhow::Result;
use clap::Parser;
use std::{
    fmt::{self},
    time::Instant,
};
use sudoku::Sudoku;

use sudoku_solver::SudokuSolver;

use crate::algorithm_x_solver::AlgorithmXSudokuSolver;

mod algorithm_x;
pub mod algorithm_x_solver;
mod backtracking_solver;
mod stats;
mod sudoku;
mod sudoku_solver;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = SolverType::AlgorithmX)]
    solver: SolverType,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum SolverType {
    Backtracking,
    AlgorithmX,
}

impl fmt::Display for SolverType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolverType::Backtracking => write!(f, "backtracking"),
            SolverType::AlgorithmX => write!(f, "algorithm-x"),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let content = std::fs::read_to_string("../test_sudokus.txt")?;

    let sudokus = content
        .lines()
        .map(Sudoku::try_from_str)
        .collect::<Result<Vec<_>>>()?;

    let solver: Box<dyn SudokuSolver> = match args.solver {
        SolverType::Backtracking => {
            println!("Using Backtracking solver.\n");
            Box::new(backtracking_solver::BacktrackingSolver {})
        }
        SolverType::AlgorithmX => {
            println!("Using Algorithm X solver.\n");
            Box::new(AlgorithmXSudokuSolver::new())
        }
    };

    println!("Starting to solve sudokus.\n");

    let nr_sudokus = sudokus.len();
    let durations = sudokus
        .into_iter()
        // .take(1)
        .enumerate()
        .map(|(n, sudoku)| {
            println!("Solving sudoku {}/{nr_sudokus}", n + 1);

            let now = Instant::now();

            let solution = solver.solve(sudoku.clone());

            let duration = now.elapsed();

            match solution {
                Some(sol) if !sol.is_solved() => {
                    panic!("Solution is wrong for sudoku!\n{sudoku}")
                }
                None => println!("No solution found for sudoku!\n{sudoku}"),
                _ => println!("Solved sudoku in {}us\n", duration.as_micros()),
            }

            duration
        })
        .collect::<Vec<_>>();

    let duration_ms = durations
        .iter()
        .map(|d| d.as_micros() as f64)
        .collect::<Vec<_>>();

    let duration_stats = stats::compute_statistics(duration_ms.as_slice());

    println!("Statistics: {}", duration_stats);

    Ok(())
}
