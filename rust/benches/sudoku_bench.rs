use criterion::{criterion_group, criterion_main, Criterion};
use std::path::Path;
use sudoku_solver::algorithm_x::Solver;
use sudoku_solver::SudokuSolver;

fn benchmark(c: &mut Criterion) {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/bench_sudokus.txt");
    let content = std::fs::read_to_string(file).unwrap();

    let sudokus = content
        .lines()
        .map(|line| sudoku_solver::Sudoku::try_from_str(line).unwrap())
        .collect::<Vec<_>>();

    let solver = Solver::new();

    c.bench_function("solve sudokus", |b| {
        b.iter(|| {
            for sudoku in &sudokus {
                let _ = solver.solve(sudoku.clone()).unwrap();
            }
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        // .sample_size(1000)
        // .nresamples(700_000)
        .measurement_time(std::time::Duration::from_secs(15));
    targets = benchmark
}
criterion_main!(benches);
