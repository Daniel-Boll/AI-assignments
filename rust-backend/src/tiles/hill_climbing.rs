pub mod hill_climbing {
    use crate::tiles::{
        routes_utils::routes_utils::{SolveResult, Tile},
        tiles_utils::tiles_utils::{is_board_correct, possible_moves},
    };

    const MAX_ITERATIONS: i32 = 1_000;

    pub fn solver(board: [Tile; 9]) -> SolveResult {
        let mut board = board;
        let mut solution_steps: Vec<[Tile; 9]> = vec![];
        let mut iterations = 0;

        let board_print = |board: [Tile; 9]| {
            for i in 0..3 {
                for j in 0..3 {
                    print!("{} ", board[i * 3 + j].content);
                    if board[i * 3 + j].content == "" {
                        print!(" ");
                    }
                }
                println!();
            }
            println!("");
        };

        while iterations < MAX_ITERATIONS && !is_board_correct(board.clone()) {
            let moves = possible_moves(board.clone());

            // For each move calculate the heuristic if the move is better than the current one then
            // apply it
            let mut best_move = moves[0].clone();
            let mut best_move_cost = heuristic(moves[0].clone());

            for move_ in moves {
                let move_cost = heuristic(move_.clone());

                if move_cost < best_move_cost {
                    best_move = move_.clone();
                    best_move_cost = move_cost;
                }
            }

            board = best_move.clone();
            solution_steps.push(board.clone());

            println!("Current cost {}\n", best_move_cost);
            board_print(board.clone());
            iterations += 1;
        }

        SolveResult { solution_steps }
    }

    /// The heuristic will be the sum of the distances of each tile to its goal position
    fn heuristic(board: [Tile; 9]) -> i32 {
        let mut sum = 0;
        for i in 0..=7 {
            let tile = if board[i].content != "" {
                board[i]
                    .content
                    .parse::<i32>()
                    .expect("Error parsing goal position")
            } else {
                9
            };

            // Calculate the distance between the tile and its goal position
            sum += manhattan_distance(i as i32, tile);
        }

        sum
    }

    fn manhattan_distance(tile_position: i32, tile_goal: i32) -> i32 {
        // Get the 2D coordinates of the tile
        let x = tile_position % 3;
        let y = tile_position / 3;

        // Get the 2D coordinates of the goal position
        let goal_x = tile_goal % 3;
        let goal_y = tile_goal / 3;

        (x - goal_x).abs() + (y - goal_y).abs()
    }
}
