use std::collections::HashSet;

fn main() {
    let v1 = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
   assert!(is_valid_sudoku(v1));
}

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for row in 0..board.len() {
        let mut my_set = HashSet::new();
        for col in 0..board[row].len() {
            if board[row][col] == '.' {
                continue;
            }
            if my_set.contains(&board[row][col]) {
                return false;
            } else {
                my_set.insert(&board[row][col]);
            }
        }
    }

    for row in 0..board.len() {
        let mut my_set = HashSet::new();
        for col in 0..board[row].len() {
            if board[col][row] == '.' {
                continue;
            }
            if my_set.contains(&board[col][row]) {
                return false;
            } else {
                my_set.insert(&board[col][row]);
            }
        }
    }

    for row in 0..3 {
        for col in 0..3 {
            let mut my_set = HashSet::new();
            for irow in 0..3 {
                for icol in 0..3 {
                    let cell = board[3 * row + irow][3 * col + icol];
                    if cell == '.' {
                        continue;
                    }
                    if my_set.contains(&cell) {
                        return false;
                    } else {
                        my_set.insert(&board[3 * row + irow][ 3 * col + icol]);
                    }
                }
            }
        }
    }
    return true;
}
