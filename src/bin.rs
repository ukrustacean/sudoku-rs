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

fn main() {
    let mut sudoku = SUDOKU.parse::<Solver>().expect("Sudoku must be valid");
    println!("{}", sudoku);
    sudoku.solve();
    println!("{}", sudoku);
}
