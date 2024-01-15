use sudoku_rs::Solver;

#[cfg(not(feature = "read_from_stdin"))]
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

#[cfg(not(feature = "read_from_stdin"))]
fn main() {
    let mut sudoku = SUDOKU.parse::<Solver>().expect("Sudoku must be valid");
    println!("{}", sudoku);
    sudoku.solve();
    println!("{}", sudoku);
}

#[cfg(feature = "read_from_stdin")]
fn main() {
    use std::io::Read;
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .expect("Must be able to read sudoku from stdin");

    let mut sudoku = buf.parse::<Solver>().expect("Sudoku must be valid");
    sudoku.solve();
    println!();
    println!("{sudoku}");
}
