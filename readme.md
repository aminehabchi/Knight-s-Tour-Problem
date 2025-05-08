# Knight's Tour Problem Solver

This repository contains an implementation of the Knight's Tour problem in Rust.

## What is the Knight's Tour Problem?

The Knight's Tour is a classic chess puzzle where the objective is to find a sequence of moves for a knight such that it visits every square on an NxN chessboard exactly once. The knight moves according to standard chess rules - in an L-shape pattern (2 squares in one direction and 1 square perpendicular to that direction).

## Implementation Details

This implementation provides two different algorithms to solve the Knight's Tour problem:

### 1. Backtracking Algorithm (`back_track`)

The backtracking algorithm is a depth-first search approach that:
- Places the knight on a cell
- Recursively tries all possible knight moves from the current position
- If a move leads to a solution, it keeps that move
- If a move doesn't lead to a solution, it undoes that move (backtracks) and tries another move
- Returns true when all squares have been visited (successful tour found)

```rust
fn back_track(board: &mut [[i32; N]; N], y: usize, x: usize, move_nbr: i32) -> bool {
    // Implementation details...
}
```

### 2. Warnsdorff's Algorithm (`warnsdorff`)

Warnsdorff's algorithm is a heuristic method that:
- Always chooses the next move that has the fewest onward moves
- This greedy approach significantly improves performance over pure backtracking
- Works very well for larger board sizes where backtracking would be too slow

```rust
fn warnsdorff(board: &mut [[i32; N]; N], y: usize, x: usize, move_nbr: i32) {
    // Implementation details...
}
```

## How to Use

1. Ensure you have Rust installed on your system
2. Define the board size `N` and the `VECTORS` representing knight's possible moves
3. Initialize your board with `-1` in all cells
4. Choose your starting position (x, y)
5. Call either algorithm to solve the tour:
   ```rust
   // For backtracking:
   back_track(&mut board, start_y, start_x, 0);
   
   // For Warnsdorff's algorithm:
   warnsdorff(&mut board, start_y, start_x, 0);
   ```

## Time Complexity

- Backtracking algorithm: O(8^(N²)) in the worst case
- Warnsdorff's algorithm: O(N²) in practice, making it much more efficient for large boards

## Visualization

The board is represented as a 2D array where:
- `-1` indicates an unvisited square
- A positive integer indicates the move number in the knight's tour sequence

## Example

For a standard 8×8 chessboard, a complete knight's tour will have numbers from 0 to 63, representing the sequence of the knight's moves.

## Notes

- Not all starting positions may lead to a solution, especially for certain board sizes
- Warnsdorff's algorithm is probabilistic and may not always find a solution even when one exists
- The backtracking algorithm guarantees finding a solution if one exists, but may be too slow for larger boards
