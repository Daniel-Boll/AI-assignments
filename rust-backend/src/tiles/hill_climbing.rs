pub mod hill_climbing {
    use crate::tiles::routes_utils::routes_utils::{SolveResult, Tile};

    pub fn solver(board: [Tile; 9]) -> SolveResult {
        SolveResult {
            solution_steps: vec![board],
        }
    }
}
