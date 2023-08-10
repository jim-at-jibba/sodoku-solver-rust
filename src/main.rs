// What we need to do
// 1. Find empty (0) cell.
//   1.1. If there is no empty cells -> return the puzzle.
// 2. Find valid guesses for empty cell.
// 2.1. Try to recursively solve puzzle with this guess.
// 2.1.1. If there is no valid guesses, then assign cell value to empty (0) and prune this branch as dead-end. (backtracking)
// from: https://blog.cloudboost.io/sudoku-solver-rust-recursive-implementation-backtracking-technique-fecf87d0477
fn main() {
    println!("Hello, world!");
}

fn test_board() -> [[i8; 9]; 9] {
    [
        [1, 7, 4, 0, 9, 0, 6, 0, 0],
        [0, 0, 0, 0, 3, 8, 1, 5, 7],
        [5, 3, 0, 7, 0, 1, 0, 0, 4],
        [0, 0, 7, 3, 4, 9, 8, 0, 0],
        [8, 4, 0, 5, 0, 0, 3, 6, 0],
        [3, 0, 5, 0, 0, 6, 4, 7, 0],
        [2, 8, 6, 9, 0, 0, 0, 0, 1],
        [0, 0, 0, 6, 2, 7, 0, 3, 8],
        [0, 5, 3, 0, 8, 0, 0, 9, 6],
    ]
}
