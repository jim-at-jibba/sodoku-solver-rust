// What we need to do
// 1. Find empty (0) cell.
//   1.1. If there is no empty cells -> return the puzzle.
// 2. Find valid guesses for empty cell.
// 2.1. Try to recursively solve puzzle with this guess.
// 2.1.1. If there is no valid guesses, then assign cell value to empty (0) and prune this branch as dead-end. (backtracking)
// from: https://blog.cloudboost.io/sudoku-solver-rust-recursive-implementation-backtracking-technique-fecf87d0477

/**
    vocab
    ROW - array of 9 elements of the puzzle located horizontally
    COLUMN - array of 9 elements of the puzzle located vertically
    SQUARE - array of 9 elements of the puzzle located in the shape 3x3
    eg. [1,7,4,0,0,0,5,3,0]
*/
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

#[test]
fn returns_invalid_for_1_0_4() {
    assert!(!solver::valid(test_board(), 1, 0, 4))
}

#[test]
fn returns_invalid_for_1_0_1() {
    assert!(!solver::valid(test_board(), 1, 0, 1))
}

#[test]
fn returns_invalid_for_1_1_4() {
    assert!(!solver::valid(test_board(), 1, 1, 4))
}

#[test]
fn returns_invalid_for_1_2_4() {
    assert!(!solver::valid(test_board(), 1, 2, 4))
}
#[test]
fn returns_invalid_for_0_3_4() {
    assert!(!solver::valid(test_board(), 0, 3, 4))
}
#[test]
fn returns_invalid_for_0_3_3() {
    assert!(!solver::valid(test_board(), 0, 3, 3))
}
#[test]
fn returns_invalid_for_6_6_1() {
    assert!(!solver::valid(test_board(), 6, 6, 1))
}
#[test]
fn returns_valid_for_0_3_2() {
    assert!(!solver::valid(test_board(), 0, 3, 2))
}
