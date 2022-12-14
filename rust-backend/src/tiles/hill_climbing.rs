pub mod hill_climbing {
    use crate::tiles::{
        routes_utils::routes_utils::{SolveResult, Tile},
        tiles_utils::tiles_utils::{is_board_correct, possible_moves},
    };

    const MAX_ITERATIONS: i32 = 5_000;
    const RANDOM_MOVES: i32 = 2;

    pub fn solver(board: [Tile; 9]) -> SolveResult {
        let mut board = board;
        let mut solution_steps: Vec<[Tile; 9]> = vec![];
        let mut iterations = 0;
        // Store the last 4 boards to avoid infinite loops
        let mut last_boards: Vec<[Tile; 9]> = vec![];
        let mut returns = 0;

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

            for move_ in &moves {
                let move_cost = heuristic(move_.clone());

                if move_cost < best_move_cost {
                    best_move = move_.clone();
                    best_move_cost = move_cost;
                    // break;
                }
            }

            // Analyze if the last 10 boards are very similar to the current one
            // If they are then the algorithm is stuck in a local minimum and therefore we can pick a random move
            let mut similar_boards = 0;
            // Check for similar costs
            for last_board in last_boards.clone() {
                if heuristic(last_board.clone()) == heuristic(board.clone()) {
                    similar_boards += 1;
                }
            }

            if similar_boards > 5 {
                returns = RANDOM_MOVES;
            }

            if returns > 0 {
                let random_move = rand::random::<usize>() % &moves.len();
                best_move = moves[random_move].clone();
                returns -= 1;
            }

            // Sanitize the last boards
            if last_boards.len() > 10 {
                last_boards.remove(0);
            }

            board = best_move.clone();
            solution_steps.push(board.clone());
            last_boards.push(board.clone());

            println!(
                "Current cost {}; Random choices: {}\n",
                best_move_cost, returns
            );
            board_print(board.clone());
            iterations += 1;
        }

        println!("Solution cost {}\n", heuristic(board.clone()));
        SolveResult { solution_steps }
    }

    /// The heuristic will be the sum of the distances of each tile to its goal position
    fn heuristic(board: [Tile; 9]) -> i32 {
        // The goal position the following board
        // ┌─┬─┬─┐
        // │1│2│3│
        // ├─┼─┼─┤
        // │4│5│6│ This board has an heuristic of 0
        // ├─┼─┼─┤
        // │7│8│ │
        // └─┴─┴─┘
        //
        // The heuristic of the following board is 2
        // ┌─┬─┬─┐
        // │1│2│3│
        // ├─┼─┼─┤
        // │4│5│6│
        // ├─┼─┼─┤
        // │ │7│8│
        // └─┴─┴─┘

        // Calculate the manhattan distance of each tile to its goal position
        let mut cost = 0;

        for i in 0..9 {
            let tile = board[i].clone();

            if tile.is_empty() {
                continue;
            }

            let goal_position = tile.goal_position();
            let current_position = i;

            cost += manhattan_distance(current_position as i32, goal_position as i32);
        }

        cost
    }

    // ┌─┬─┬─┐
    // │0│1│2│
    // ├─┼─┼─┤
    // │3│4│5│
    // ├─┼─┼─┤
    // │6│7│ │
    // └─┴─┴─┘
    fn manhattan_distance(tile_position: i32, tile_goal: i32) -> i32 {
        // Get the 2D coordinates of the tile
        let x = tile_position % 3;
        let y = tile_position / 3;

        // Get the 2D coordinates of the goal position
        let goal_x = tile_goal % 3;
        let goal_y = tile_goal / 3;

        (x - goal_x).abs() + (y - goal_y).abs()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_manhattan_distance() {
            // goal 0
            // ┌─┬─┬─┐
            // │X│ │ │
            // ├─┼─┼─┤
            // │ │ │ │ -> 0
            // ├─┼─┼─┤
            // │ │ │ │
            // └─┴─┴─┘
            assert_eq!(manhattan_distance(0, 0), 0);

            // goal 1
            // ┌─┬─┬─┐
            // │X│.│ │
            // ├─┼─┼─┤
            // │ │ │ │ -> 1
            // ├─┼─┼─┤
            // │ │ │ │
            // └─┴─┴─┘
            assert_eq!(manhattan_distance(0, 1), 1);

            // goal 2
            // ┌─┬─┬─┐
            // │X│ │.│
            // ├─┼─┼─┤
            // │ │ │ │ -> 2
            // ├─┼─┼─┤
            // │ │ │ │
            // └─┴─┴─┘
            assert_eq!(manhattan_distance(0, 2), 2);

            // goal 3
            // ┌─┬─┬─┐
            // │X│ │ │
            // ├─┼─┼─┤
            // │.│ │ │ -> 1
            // ├─┼─┼─┤
            // │ │ │ │
            // └─┴─┴─┘
            assert_eq!(manhattan_distance(0, 3), 1);

            // goal 4
            // ┌─┬─┬─┐
            // │X│ │ │
            // ├─┼─┼─┤
            // │ │.│ │ -> 2
            // ├─┼─┼─┤
            // │ │ │ │
            // └─┴─┴─┘
            assert_eq!(manhattan_distance(0, 4), 2);

            // goal 5
            // ┌─┬─┬─┐
            // │X│ │ │
            // ├─┼─┼─┤
            // │ │ │.│ -> 3
            // ├─┼─┼─┤
            // │ │ │ │
            // └─┴─┴─┘
            assert_eq!(manhattan_distance(0, 5), 3);

            // goal 6
            // ┌─┬─┬─┐
            // │X│ │ │
            // ├─┼─┼─┤
            // │ │ │ │ -> 2
            // ├─┼─┼─┤
            // │.│ │ │
            // └─┴─┴─┘
            assert_eq!(manhattan_distance(0, 6), 2);

            // goal 7
            // ┌─┬─┬─┐
            // │X│ │ │
            // ├─┼─┼─┤
            // │ │ │ │ -> 3
            // ├─┼─┼─┤
            // │ │.│ │
            // └─┴─┴─┘
            assert_eq!(manhattan_distance(0, 7), 3);

            // goal 8
            // ┌─┬─┬─┐
            // │X│ │ │
            // ├─┼─┼─┤
            // │ │ │ │ -> 4
            // ├─┼─┼─┤
            // │ │ │.│
            // └─┴─┴─┘
            assert_eq!(manhattan_distance(0, 8), 4);
        }

        #[test]
        fn board_heuristic() {
            // ┌─┬─┬─┐
            // │1│2│3│
            // ├─┼─┼─┤
            // │4│5│6│
            // ├─┼─┼─┤
            // │7│8│ │
            // └─┴─┴─┘
            let board = [
                Tile {
                    content: "1".to_string(),
                },
                Tile {
                    content: "2".to_string(),
                },
                Tile {
                    content: "3".to_string(),
                },
                Tile {
                    content: "4".to_string(),
                },
                Tile {
                    content: "5".to_string(),
                },
                Tile {
                    content: "6".to_string(),
                },
                Tile {
                    content: "7".to_string(),
                },
                Tile {
                    content: "8".to_string(),
                },
                Tile {
                    content: "".to_string(),
                },
            ];

            assert_eq!(heuristic(board), 0);

            // ┌─┬─┬─┐
            // │1│2│3│
            // ├─┼─┼─┤
            // │4│5│6│
            // ├─┼─┼─┤
            // │7│ │8│
            // └─┴─┴─┘
            let board = [
                Tile {
                    content: "1".to_string(),
                },
                Tile {
                    content: "2".to_string(),
                },
                Tile {
                    content: "3".to_string(),
                },
                Tile {
                    content: "4".to_string(),
                },
                Tile {
                    content: "5".to_string(),
                },
                Tile {
                    content: "6".to_string(),
                },
                Tile {
                    content: "7".to_string(),
                },
                Tile {
                    content: "".to_string(),
                },
                Tile {
                    content: "8".to_string(),
                },
            ];

            assert_eq!(heuristic(board), 1);

            // ┌─┬─┬─┐
            // │1│2│3│
            // ├─┼─┼─┤
            // │4│5│6│
            // ├─┼─┼─┤
            // │ │7│8│
            // └─┴─┴─┘
            let board = [
                Tile {
                    content: "1".to_string(),
                },
                Tile {
                    content: "2".to_string(),
                },
                Tile {
                    content: "3".to_string(),
                },
                Tile {
                    content: "4".to_string(),
                },
                Tile {
                    content: "5".to_string(),
                },
                Tile {
                    content: "6".to_string(),
                },
                Tile {
                    content: "".to_string(),
                },
                Tile {
                    content: "7".to_string(),
                },
                Tile {
                    content: "8".to_string(),
                },
            ];

            assert_eq!(heuristic(board), 2);
        }
    }
}
