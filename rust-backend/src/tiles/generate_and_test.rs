pub mod generate_and_test {
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
    board
  }

  /// Generate all possible solutions for the current state
  pub fn generate(board: [Tile; 9]) {}

  /// The Manhattan distance between two points is the sum of the absolute values of their
  /// Cartesian coordinates. In a rectangular grid, at a right angle to the axes, it is the
  /// minimum number of orthogonal steps (change in one coordinate) required to go from one
  /// point to the other.
  pub fn heuristic(board: [Tile; 9]) {}
}
