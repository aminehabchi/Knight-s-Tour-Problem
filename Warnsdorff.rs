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
fn print_board(board: [[i32; N]; N]) {
    for row in &board {
        println!("{:?}", row);
    }
}

fn fewest__onward_moves(board: &mut [[i32; N]; N], x: i32, y: i32) -> i32 {
    let mut counter=0;
    for vec in &VECTORS {
        let new_x = x as i32 + vec[1];
        let new_y = y as i32 + vec[0];
        if new_x >= 0 && new_y >= 0 && new_x < N as i32 && new_y < N as i32 {
            if board[new_y as usize ,new_y as usize]!=-1{
                counter+=1;
            }
        }
    }
    counter
}

fn Warnsdorff(board: &mut [[i32; N]; N], y: usize, x: usize, move_nbr: i32) -> bool {
    if board[y][x] != -1 {
        return false;
    }

    let m: i32 = move_nbr + 1;
    board[y][x] = m;


    if m as usize == (N * N) {
        return true;
    }


    let mut new_x = -1
    let mut new_y = -1
    let mut c=15
    let index

    for i in 1..n {
        new_x = x as i32 + VECTORS[i][1];
        new_y = y as i32 + VECTORS[i][0];
        if new_x >= 0 && new_y >= 0 && (new_x as usize) < N && (new_y as usize) < N{
          let counter= fewest__onward_moves(board,new_y,new_x)
          if counter<
        }
    }
    
    if Warnsdorff(board, new_y as usize, new_x as usize, m) {
        return true;
    }

       
    board[y][x] = -1;
    return false;
}

fn main() {
    let mut matrix: [[i32; N]; N] = [[-1; N]; N];

    Warnsdorff(&mut matrix, 0, 0, 0);

    print_board(matrix);
}
