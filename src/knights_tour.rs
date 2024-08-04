use std::io::{self, Write};

const N: usize = 8; // 8x8 chessboard

// Helper function to print the chessboard
fn print_board(board: &Vec<Vec<i32>>) {
    for i in 0..N {
        for j in 0..N {
            if i % 2 == j % 2 {
                print!("\x1b[7m{:3}\x1b[0m", board[i][j]);
            } else {
                print!("{:3}", board[i][j]);
            }
        }
        println!();
    }
    io::stdout().flush().unwrap();
}

// Helper function to check if a move is valid
fn can_move(x: i32, y: i32, board: &Vec<Vec<i32>>) -> bool {
    x >= 0 && x < N as i32 && y >= 0 && y < N as i32 && board[x as usize][y as usize] == -1
}

// Recursive function to walk the knight on the chessboard
fn walk_board(x: i32, y: i32, m: i32, board: &mut Vec<Vec<i32>>, xmoves: &[i32; 8], ymoves: &[i32; 8]) -> bool {
    if m == (N * N) as i32 {
        print_board(board);
        return true;
    }

    for i in 0..8 {
        let next_x = x + xmoves[i];
        let next_y = y + ymoves[i];
        if can_move(next_x, next_y, board) {
            board[next_x as usize][next_y as usize] = m;
            if walk_board(next_x, next_y, m + 1, board, xmoves, ymoves) {
                return true;
            }
            board[next_x as usize][next_y as usize] = -1;
        }
    }
    false
}

fn main() {
    let mut board = vec![vec![-1; N]; N];
    let xmoves = [2, 1, -1, -2, -2, -1, 1, 2];
    let ymoves = [1, 2, 2, 1, -1, -2, -2, -1];

    board[0][0] = 0;
    if walk_board(0, 0, 1, &mut board, &xmoves, &ymoves) {
        println!("Solution found!");
    } else {
        println!("No solution exists!");
    }
}