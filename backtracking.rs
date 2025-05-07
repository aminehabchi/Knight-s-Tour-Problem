const N: usize = 5;

const VECTORS: [[i32; 2]; 8] = [
    [2, 1],
    [-2, -1],
    [-2, 1],
    [2, -1],
    [1, 2],
    [-1, -2],
    [-1, 2],
    [1, -2],
];


fn back_track(board: &mut [[i32; N]; N], y: usize, x: usize, move_nbr: i32) -> bool {
    if board[y][x] != -1 {
        return false;
    }

    let m: i32 = move_nbr + 1;
    board[y][x] = m;

    if m as usize == (N * N) {
        return true;
    }

    for vec in &utils::VECTORS {
        let new_x = x as i32 + vec[1];
        let new_y = y as i32 + vec[0];
        if new_x >= 0 && new_y >= 0 && new_x < N as i32 && new_y < N as i32 {
            if back_track(board, new_y as usize, new_x as usize, m) {
                return true;
            }
        }
    }
    board[y][x] = -1;
    return false;
}

fn print_board(board: [[i32; N]; N]) {
    for row in &board {
        println!("{:?}", row);
    }
}

fn main() {
    let mut matrix: [[i32; N]; N] = [[-1; N]; N];

    back_track(&mut matrix, 0, 0, 0);

    print_board(matrix)
}
