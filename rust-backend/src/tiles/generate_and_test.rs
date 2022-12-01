pub mod generate_and_test {
    use rand::Rng;

    use crate::tiles::{
        routes_utils::routes_utils::{SolveResult, Tile},
        tiles_utils::tiles_utils::{is_board_correct, possible_moves},
    };

    const MAX_ITERATIONS: i32 = 1_000;

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
    pub fn solver(board: [Tile; 9]) -> SolveResult {
        let mut board = board;
        let mut solution_steps: Vec<[Tile; 9]> = vec![];
        let mut iterations = 0;

        // Board print closure
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

            // Board will assume a random value from the moves vector
            let random_in_range_of_moves =
                rand::thread_rng().gen_range(0..possible_moves(board.clone()).len());
            board = moves[random_in_range_of_moves].clone();
            solution_steps.push(board.clone());
            iterations += 1;
        }

        SolveResult { solution_steps }
    }
}
