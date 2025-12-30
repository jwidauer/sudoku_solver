use anyhow::Result;
use clap::Parser;
use std::{
    fmt::{self},
    path::PathBuf,
    time::Instant,
};
use sudoku::Sudoku;
use thiserror::Error;

use sudoku_solver::SudokuSolver;

use crate::algorithm_x_solver::AlgorithmXSudokuSolver;

mod algorithm_x;
pub mod algorithm_x_solver;
mod backtracking_solver;
mod stats;
mod sudoku;
mod sudoku_solver;

const DEFAULT_INPUT_FILE: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/resources/bench_sudokus.txt");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file containing sudokus to solve
    #[arg(default_value = DEFAULT_INPUT_FILE)]
    input: PathBuf,

    /// Solver type to use
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

#[derive(Debug, Error)]
enum SolverError {
    #[error("No solution found for sudoku:\n{0}")]
    NoSolution(Sudoku),
    #[error("Wrong solution for sudoku:\n{0}")]
    WrongSolution(Sudoku),
}

fn main() -> Result<()> {
    let args = Args::parse();

    let content = std::fs::read_to_string(args.input)?;

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
                    return Err(SolverError::WrongSolution(sudoku).into());
                }
                None => {
                    return Err(SolverError::NoSolution(sudoku).into());
                }
                _ => println!("Solved sudoku in {}us\n", duration.as_micros()),
            }

            Ok(duration)
        })
        .collect::<Result<Vec<_>>>()?;

    let duration_ms = durations
        .iter()
        .map(|d| d.as_micros() as f64)
        .collect::<Vec<_>>();

    let duration_stats = stats::compute_statistics(duration_ms.as_slice());

    println!("Statistics: {}", duration_stats);

    Ok(())
}
