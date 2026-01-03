use anyhow::Result;
use clap::Parser;
use indicatif::ProgressIterator;
use std::{fmt, path::PathBuf, time::Instant};
use thiserror::Error;

use sudoku_solver::algorithm_x;
use sudoku_solver::backtracking;
use sudoku_solver::Sudoku;
use sudoku_solver::SudokuSolver;

mod stats;

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
            Box::new(backtracking::Solver::new())
        }
        SolverType::AlgorithmX => {
            println!("Using Algorithm X solver.\n");
            Box::new(algorithm_x::Solver::new())
        }
    };

    println!("Starting to solve sudokus.\n");

    let durations = sudokus
        .into_iter()
        .progress()
        .map(|sudoku| solve_and_time_sudoku(solver.as_ref(), sudoku))
        .collect::<Result<Vec<_>>>()?;

    let duration_stats = stats::Statistics::from_durations(&durations)?;

    println!("Statistics: {}", duration_stats);

    Ok(())
}

fn solve_and_time_sudoku(solver: &dyn SudokuSolver, sudoku: Sudoku) -> Result<std::time::Duration> {
    let now = Instant::now();
    let solution = solver.solve(sudoku.clone());
    let duration = now.elapsed();

    let solution = solution.ok_or_else(|| SolverError::NoSolution(sudoku.clone()))?;

    if !solution.is_solved() {
        return Err(SolverError::WrongSolution(sudoku).into());
    }

    Ok(duration)
}
