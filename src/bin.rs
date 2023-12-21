use sudoku_rs::Solver;

const SUDOKU: &str = r#"
                3, 1, 0, 0, 0, 0, 0, 2, 0,
                0, 0, 6, 1, 0, 9, 0, 0, 5,
                0, 0, 0, 0, 8, 0, 0, 0, 0,
                0, 2, 0, 8, 0, 4, 0, 5, 0,
                0, 0, 4, 0, 7, 0, 0, 0, 0,
                0, 0, 0, 0, 6, 0, 0, 0, 8,
                0, 6, 0, 0, 0, 0, 9, 0, 0,
                0, 0, 9, 4, 0, 5, 0, 0, 1,
                0, 0, 0, 0, 0, 7, 0, 0, 0,
"#;

fn main() {
    let mut sudoku = SUDOKU.parse::<Solver>().expect("Sudoku must be valid");
    println!("{}", sudoku);
    sudoku.solve();
    println!("{}", sudoku);
}
