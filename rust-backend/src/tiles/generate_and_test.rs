pub mod generate_and_test {
    use rand::Rng;

    use crate::tiles::routes_utils::routes_utils::Tile;

    /// This solver consist in three steps:
    ///   - Generate a possible solution. For some problems, this means generating a particular
    ///      point in the problem space. For others it means generating a path from a start state.
    ///   - Test to see if this is actually a solution by comparing the chosen point or the
    ///      endpoint of the chosen path to the set of acceptable goal states.
    ///   - If a solution has been found, quit, Otherwise return to step 1.
    ///
    /// The neighborhood for the 3x3 puzzle is the set of all possible moves from the current state,
    /// as long as the neighbor is empty. The goal state is the solved puzzle.
    ///
    /// The heuristic used is the sum of the Manhattan distances of each tile from its goal position.
    pub fn solver(board: [Tile; 9]) -> [Tile; 9] {
        let mut board = board;

        for i in 0..9 {
            print!("{} ", board[i].content);
            if board[i].content.len() == 0 {
                print!(" ");
            }
            if i % 3 == 2 {
                println!();
            }
        }
        println!("");

        while !is_board_correct(board.clone()) {
            let moves = generate(board.clone());
            let a = &moves;

            // Print every vector as a 3x3 grid of tiles
            a.into_iter().for_each(|move_| {
                for i in 0..9 {
                    print!("{} ", move_[i].content);
                    if move_[i].content.len() == 0 {
                        print!(" ");
                    }
                    if i % 3 == 2 {
                        println!();
                    }
                }
                println!("");
            });

            // Board will assume a random value from the moves vector
            let random_in_range_of_moves =
                rand::thread_rng().gen_range(0..generate(board.clone()).len());
            board = moves[random_in_range_of_moves].clone();
        }

        board
    }

    /// Generate all possible solutions for the current state
    pub fn generate(board: [Tile; 9]) -> Vec<[Tile; 9]> {
        possible_moves(board.clone())
    }

    /// Identify the possible tiles that can be moved to the empty space
    /// and return a list of possible new states for the board
    fn possible_moves(board: [Tile; 9]) -> Vec<[Tile; 9]> {
        let mut moves = Vec::new();
        let empty_tile = board
            .iter()
            .position(|x| x.content == String::from(""))
            .unwrap();

        // If the empty tile is in any of the four corners, there are only two possible moves
        if empty_tile == 0 {
            moves.push(swap_board(board.clone(), 0, 1));
            moves.push(swap_board(board.clone(), 0, 3));
        } else if empty_tile == 2 {
            moves.push(swap_board(board.clone(), 2, 1));
            moves.push(swap_board(board.clone(), 2, 5));
        } else if empty_tile == 6 {
            moves.push(swap_board(board.clone(), 6, 3));
            moves.push(swap_board(board.clone(), 6, 7));
        } else if empty_tile == 8 {
            moves.push(swap_board(board.clone(), 8, 5));
            moves.push(swap_board(board.clone(), 8, 7));
        }
        // If the empty tile is in any of the four sides, there are three possible moves
        else if empty_tile == 1 {
            moves.push(swap_board(board.clone(), 1, 0));
            moves.push(swap_board(board.clone(), 1, 2));
            moves.push(swap_board(board.clone(), 1, 4));
        } else if empty_tile == 3 {
            moves.push(swap_board(board.clone(), 3, 0));
            moves.push(swap_board(board.clone(), 3, 4));
            moves.push(swap_board(board.clone(), 3, 6));
        } else if empty_tile == 5 {
            moves.push(swap_board(board.clone(), 5, 2));
            moves.push(swap_board(board.clone(), 5, 4));
            moves.push(swap_board(board.clone(), 5, 8));
        } else if empty_tile == 7 {
            moves.push(swap_board(board.clone(), 7, 4));
            moves.push(swap_board(board.clone(), 7, 6));
            moves.push(swap_board(board.clone(), 7, 8));
        }
        // If the empty tile is in the middle, there are four possible moves
        else if empty_tile == 4 {
            moves.push(swap_board(board.clone(), 4, 1));
            moves.push(swap_board(board.clone(), 4, 3));
            moves.push(swap_board(board.clone(), 4, 5));
            moves.push(swap_board(board.clone(), 4, 7));
        }

        moves
    }

    fn swap_board(board: [Tile; 9], from: usize, to: usize) -> [Tile; 9] {
        let mut new_board = board;
        new_board.swap(from, to);
        new_board
    }

    fn is_board_correct(board: [Tile; 9]) -> bool {
        let mut correct = true;
        for i in 0..=8 {
            if board[i].content != String::from((i + 1).to_string()) {
                correct = false;
            }
        }
        correct
    }
}
