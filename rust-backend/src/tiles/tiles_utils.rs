pub mod tiles_utils {
    use crate::tiles::routes_utils::routes_utils::Tile;

    /// Identify the possible tiles that can be moved to the empty space
    /// and return a list of possible new states for the board
    pub fn possible_moves(board: [Tile; 9]) -> Vec<[Tile; 9]> {
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

    pub fn swap_board(board: [Tile; 9], from: usize, to: usize) -> [Tile; 9] {
        let mut new_board = board;
        new_board.swap(from, to);
        new_board
    }

    pub fn is_board_correct(board: [Tile; 9]) -> bool {
        let mut correct = true;
        for i in 0..=7 {
            if board[i].content != String::from((i + 1).to_string()) {
                correct = false;
            }
        }
        correct
    }
}
