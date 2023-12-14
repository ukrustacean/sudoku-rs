use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::str::FromStr;
use sudoku_rs::Solver;

const SUDOKU: &str = r#"
024007000
600000000
003680415
431005000
500000032
790000060
209710800
040093000
310004750
"#;

fn sudoku_bench(c: &mut Criterion) {
    c.bench_function("sudoku", |b| {
        b.iter(|| {
            let mut solver = Solver::from_str(black_box(SUDOKU)).unwrap();
            solver.solve();
        })
    });
}

criterion_group!(benches, sudoku_bench);
criterion_main!(benches);
