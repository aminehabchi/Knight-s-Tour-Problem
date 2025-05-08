const N: usize = 50;

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

fn print_board(board: [[i32; N]; N]) {
    for row in &board {
        println!("{:?}", row);
    }
}

fn count_moves(board: &[[i32; N]; N], y: i32, x: i32) -> usize {
    let mut counter = 0;
    for vec in &VECTORS {
        let newx = x + vec[0];
        let newy = y + vec[1];
        if newx >= 0 && newy >= 0 && newx < N as i32 && newy < N as i32 {
            if board[newy as usize][newx as usize] == -1 {
                counter += 1;
            }
        }
    }
    counter
}

fn warnsdorff(board: &mut [[i32; N]; N], y: usize, x: usize, move_nbr: i32) {
    board[y][x] = move_nbr;

    let mut fewest = 9;
    let mut next_x = 0;
    let mut next_y = 0;
    let mut found = false;

    for (_, vec) in VECTORS.iter().enumerate() {
        let newx = x as i32 + vec[0];
        let newy = y as i32 + vec[1];

        if newx >= 0 && newy >= 0 && newx < N as i32 && newy < N as i32 {
            let newx_usize = newx as usize;
            let newy_usize = newy as usize;

            if board[newy_usize][newx_usize] == -1 {
                let c = count_moves(board, newy, newx);
                if c < fewest {
                    fewest = c;
                    next_x = newx_usize;
                    next_y = newy_usize;
                    found = true;
                }
            }
        }
    }

    if found {
        warnsdorff(board, next_y, next_x, move_nbr + 1);
    }
}

fn main() {
    let mut matrix: [[i32; N]; N] = [[-1; N]; N];
    warnsdorff(&mut matrix, 0, 0, 0);
    print_board(matrix);
}
