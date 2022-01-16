use rand::seq::SliceRandom;
use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
pub enum BoardState {
  EMPTY = 0,
  PRIMARY = 1,
  SECONDARY = 2,
}

#[derive(PartialEq, Copy, Clone)]
enum FrameState {
  EMPTY,
  USED,
  INVERTED,
}

struct StackFrame {
  tile: usize,
  derived_tiles: Vec<usize>,
  state: FrameState,
}

pub struct BoardGenerator {
  pub size: usize,
}

fn inverse_state(state: BoardState) -> BoardState {
  match state {
    BoardState::PRIMARY => BoardState::SECONDARY,
    BoardState::SECONDARY => BoardState::PRIMARY,
    BoardState::EMPTY => BoardState::EMPTY,
  }
}

impl BoardGenerator {
  fn has_doubles(
    &self,
    board: &mut Vec<BoardState>,
    horizontal: bool,
    pruned: &mut bool,
    err: &mut bool,
  ) {
    if *err {
      return;
    }

    let idx = |i: usize, j: usize| match horizontal {
      true => i * self.size + j,
      false => j * self.size + i,
    };
    for i in 0..self.size {
      let mut last_state = BoardState::EMPTY;
      for j in 0..self.size {
        if board[idx(i, j)] == last_state && last_state != BoardState::EMPTY {
          let new_state = inverse_state(last_state);
          if j > 1 && board[idx(i, j - 2)] != new_state {
            if board[idx(i, j - 2)] == last_state {
              *err = true;
              return;
            }
            board[idx(i, j - 2)] = new_state;
            *pruned = true;
          }
          if j < self.size - 1 && board[idx(i, j + 1)] != new_state {
            if board[idx(i, j + 1)] == last_state {
              *err = true;
              return;
            }
            board[idx(i, j + 1)] = new_state;
            *pruned = true;
          }
        }
        last_state = board[idx(i, j)];
      }
    }
  }

  fn has_empty_middles(
    &self,
    board: &mut Vec<BoardState>,
    horizontal: bool,
    pruned: &mut bool,
    err: &mut bool,
  ) {
    if *err {
      return;
    }

    let idx = |i: usize, j: usize| match horizontal {
      true => i * self.size + j,
      false => j * self.size + i,
    };
    for i in 0..self.size {
      for j in 0..(self.size - 2) {
        let state = board[idx(i, j)];
        if state != BoardState::EMPTY && state == board[idx(i, j + 2)] {
          if board[idx(i, j + 1)] == state {
            *err = true;
            return;
          }

          if board[idx(i, j + 1)] != BoardState::EMPTY {
            continue;
          }
          board[idx(i, j + 1)] = inverse_state(state);
          *pruned = true;
        }
      }
    }
  }

  fn has_exhausted_states(
    &self,
    board: &mut Vec<BoardState>,
    horizontal: bool,
    pruned: &mut bool,
    err: &mut bool,
  ) {
    if *err {
      return;
    }

    let idx = |i: usize, j: usize| match horizontal {
      true => i * self.size + j,
      false => j * self.size + i,
    };

    let max = self.size / 2;
    for i in 0..self.size {
      let mut primaries = 0;
      let mut secondaries = 0;
      let mut zeroes = 0;
      for j in 0..self.size {
        let state = board[idx(i, j)];
        if state == BoardState::PRIMARY {
          primaries += 1;
        } else if state == BoardState::SECONDARY {
          secondaries += 1;
        } else {
          zeroes += 1;
        }
      }

      if primaries > max || secondaries > max {
        *err = true;
        return;
      }

      if zeroes == 0 {
        continue;
      }

      if primaries < max && secondaries < max {
        continue;
      }
      let new_state = match primaries == max {
        true => BoardState::SECONDARY,
        false => BoardState::PRIMARY,
      };
      for j in 0..self.size {
        if board[idx(i, j)] == BoardState::EMPTY {
          board[idx(i, j)] = new_state;
          *pruned = true;
        }
      }
    }
  }

  fn prune_empty_tiles(&self, board: &mut Vec<BoardState>) -> Vec<usize> {
    let unpruned_board = board.to_vec();

    loop {
      let mut pruned = false;
      let mut err = false;

      self.has_doubles(board, true, &mut pruned, &mut err);
      self.has_doubles(board, false, &mut pruned, &mut err);
      self.has_empty_middles(board, true, &mut pruned, &mut err);
      self.has_empty_middles(board, false, &mut pruned, &mut err);
      self.has_exhausted_states(board, true, &mut pruned, &mut err);
      self.has_exhausted_states(board, false, &mut pruned, &mut err);

      if err || !pruned {
        break;
      }
    }

    let mut derived_tiles = vec![];
    for i in 0..(self.size * self.size) {
      if unpruned_board[i] != board[i] {
        derived_tiles.push(i);
      }
    }

    return derived_tiles;
  }

  fn gen_seeded_board(
    &self,
    base_board: &Vec<BoardState>,
  ) -> Result<(Vec<BoardState>, Vec<BoardState>), ()> {
    let mut rng = rand::thread_rng();
    let mut board = base_board.to_vec();

    let mut empty_count = base_board
      .into_iter()
      .filter(|tile| **tile == BoardState::EMPTY)
      .count();

    if empty_count == 0 {
      // We were given a completed board!
      return Ok((board.to_vec(), board.to_vec()));
    }

    let mut stack = vec![StackFrame {
      tile: self.nth_empty(&board, rng.gen_range(0..empty_count)),
      derived_tiles: vec![],
      state: FrameState::EMPTY,
    }];
    loop {
      if stack.is_empty() {
        // The board we received was invalid
        return Err(());
      }
      let frame = stack.pop().unwrap();
      if frame.state == FrameState::INVERTED {
        // if we've tried both tile types, a parent tile must be invalid
        empty_count += frame.derived_tiles.len() + 1;
        for tile in frame.derived_tiles {
          board[tile] = BoardState::EMPTY;
        }
        board[frame.tile] = BoardState::EMPTY;
        continue;
      }

      let mut new_state = FrameState::EMPTY;
      if frame.state == FrameState::USED {
        // the tile we guessed didn't work - on a valid board the inverse
        // of this tile must be valid then. If it's not, then it was an
        // invalid board to begin with.
        new_state = FrameState::INVERTED;

        empty_count += frame.derived_tiles.len();
        for tile in frame.derived_tiles {
          board[tile] = BoardState::EMPTY;
        }
        board[frame.tile] = inverse_state(board[frame.tile]);
      } else if frame.state == FrameState::EMPTY {
        new_state = FrameState::USED;

        empty_count -= 1;
        board[frame.tile] = match rng.gen::<bool>() {
          true => BoardState::PRIMARY,
          false => BoardState::SECONDARY,
        }
      }

      let derived_tiles = self.prune_empty_tiles(&mut board);
      empty_count -= derived_tiles.len();
      stack.push(StackFrame {
        tile: frame.tile,
        derived_tiles: derived_tiles,
        state: new_state,
      });

      if !self.validate_board(&board) {
        // if our derived tiles are invalid, then this tile doesn't work
        // (although, it may be a parent tile that's incorrect)
        continue;
      }

      if empty_count == 0 {
        break;
      }

      // This board may be valid, lets try another random tile
      stack.push(StackFrame {
        tile: self.nth_empty(&board, rng.gen_range(0..empty_count)),
        derived_tiles: vec![],
        state: FrameState::EMPTY,
      });
    }

    let solution = board.to_vec();
    board = base_board.to_vec();
    for frame in stack {
      board[frame.tile] = solution[frame.tile];
    }
    return Ok((board, solution));
  }

  fn prune_unnecessary_tiles(&self, board: &mut Vec<BoardState>) {
    let tiles = board
      .to_vec()
      .into_iter()
      .enumerate()
      .filter(|(_, state)| *state == BoardState::EMPTY)
      .map(|(idx, _)| idx);

    for tile in tiles {
      let mut scrap_board = board.to_vec();
      scrap_board[tile] = BoardState::EMPTY;
      self.prune_empty_tiles(&mut scrap_board);
      if scrap_board[tile] == board[tile] {
        // We were able to derive our tiles from a minimal board, so it's
        // trivially prunable.
        board[tile] = BoardState::EMPTY;
      }
    }
  }

  pub fn gen_board_optimized(&self) -> (Vec<BoardState>, Vec<BoardState>) {
    let mut empty_board = Vec::with_capacity(self.size * self.size);
    for _ in 0..(self.size * self.size) {
      empty_board.push(BoardState::EMPTY);
    }

    println!("start");
    let (mut board, solution) = self.gen_seeded_board(&empty_board).unwrap();
    println!("lame board");
    self.prune_unnecessary_tiles(&mut board);
    println!("lightly pruned board");

    let mut rng = rand::thread_rng();

    let mut tiles = board
      .to_vec()
      .into_iter()
      .enumerate()
      .filter(|(_, state)| *state != BoardState::EMPTY)
      .map(|(idx, _)| idx)
      .collect::<Vec<usize>>();
    tiles.shuffle(&mut rng);

    for tile in tiles {
      let last_state = board[tile];
      if board[tile] == BoardState::PRIMARY {
        board[tile] = BoardState::SECONDARY;
      } else if board[tile] == BoardState::SECONDARY {
        board[tile] = BoardState::PRIMARY;
      } else {
        print!("Oh no! This shouldn't happen!!! :-(");
      }

      println!("Inverting tile {}", tile);
      if self.gen_seeded_board(&board).is_err() {
        // each tile can only have one value in a final solution, so we can
        // prune this tile if inverting it would produce an invalid solution.
        board[tile] = BoardState::EMPTY;
      } else {
        // restore our old state
        board[tile] = last_state;
      }
    }

    println!("Done");

    return (board, solution);
    /*
     * TODO: We're going to rewrite this algorithm to hopefully be
     * significantly faster. The goal is to add a random space, use a
     * heuristic to see which spaces can be derived, and check the validity
     * of the board. If it's valid, push that state onto our stack. If it's
     * invalid, then this piece is wrong and we should flip it. If that's
     * still invalid, then we can pop it off our stack and resetting all the
     * derived spots. If we fill up the board, then yay! We have a valid game
     * if we remove all the derived spots. Next, we want to trim unnecessary
     * board pieces. We can use an inverse heuristic to find guaranteed spots
     * and remove them (e.g. remove the end caps of a two piece spot). We can
     * also try flipping each base tile one at a time, and seeing if that
     * produces an unsolvable board. If it does, then we can remove that
     * tile.
     *
     * fn gen_board(board):
     *   stack = [{tiles: [board.random_empty_tile()], state: EMPTY}]
     *
     *   while board.not_full():
     *     if stack.is_empty():
     *       # The board we received was invalid
     *       return INVALID
     *
     *     frame = stack.pop()
     *     if frame.state == INVERTED:
     *       # if we've tried both tile types, a parent tile must be invalid
     *       board.clear(frame.tiles)
     *       continue
     *
     *     new_state = USED
     *     else if frame.state == USED:
     *       # the tile we guessed didn't work - on a valid board the inverse
     *       # tile must be valid. If it's not, a parent tile must be invalid
     *       new_state = INVERTED
     *       board.clear(frame.tiles.slice(1))
     *       board.invert(frame.tiles[0])
     *
     *     derived_tiles = board.prune_empty_tiles()
     *     stack.push({tiles: [tile, ...derived_tiles], state: new_state})
     *     if !board.valid():
     *       # if our derived tiles are invalid, then this tile doesn't work
     *       # (although, it may be a parent tile that's incorrect)
     *       continue
     *
     *     # This board may be valid, lets try another random tile
     *     stack.push({tiles: [board.random_empty_tile()], state: EMPTY})
     *
     *   tiles = stack.map(tiles => tiles[0])
     *   while stack.not_empty():
     *     board.clear(stack.pop())
     *   return tiles
     *
     * fn gen_minimal_board(size)
     *   board = new Board(size)
     *   tiles = gen_board(board)
     *   board.set_tiles(tiles)
     *   board.prune_unnecessary_tiles()
     *
     *   for tile in tiles:
     *     board.invert(tile)
     *     if gen_board(board) is INVALID:
     *       # tile can only have one value, so it can be pruned
     *       board.clear(tile)
     *     else
     *       # set it back
     *       board.invert(tile)
     *
     *   return board
     *
     */
  }

  pub fn gen_board(&self) -> Vec<BoardState> {
    let mut board = Vec::with_capacity(self.size * self.size);
    for _ in 0..(self.size * self.size) {
      board.push(BoardState::EMPTY);
    }

    let mut rng = rand::thread_rng();
    let mut empty_count = self.size * self.size;

    loop {
      let idx = self.nth_empty(&board, rng.gen_range(0..empty_count));
      board[idx] = match rng.gen::<bool>() {
        true => BoardState::PRIMARY,
        false => BoardState::SECONDARY,
      };
      empty_count -= 1;

      let mut solution_count = self.count_solutions(&board);
      if solution_count == 0 {
        // If this spot is invalid, then the inverse must be valid
        board[idx] = match board[idx] {
          BoardState::PRIMARY => BoardState::SECONDARY,
          _ => BoardState::PRIMARY,
        };
        solution_count = self.count_solutions(&board);
      }

      if solution_count == 1 {
        break;
      }
    }

    return board;
  }

  fn count_solutions(&self, base_board: &Vec<BoardState>) -> usize {
    let mut board = base_board.clone();
    let mut solutions = 0;
    let mut idx_stack: Vec<usize> = Vec::with_capacity(self.size * self.size + 1);
    idx_stack.push(self.nth_empty(&board, 0));

    loop {
      if idx_stack.len() == 0 {
        // We've tried every combo
        break;
      }

      let idx = idx_stack.pop().unwrap();

      if idx == usize::MAX {
        // We filled the board! Woo!
        solutions += 1;
        if solutions > 1 {
          // prune this function to only generate 2 solutions
          break;
        }
        continue;
      }

      if board[idx] == BoardState::SECONDARY {
        // We've fully exhausted this state. Just pop it.
        board[idx] = BoardState::EMPTY;
        continue;
      }

      // Try the next state.
      if board[idx] == BoardState::EMPTY {
        board[idx] = BoardState::PRIMARY;
      } else {
        board[idx] = BoardState::SECONDARY;
      }
      idx_stack.push(idx);

      // We can progress through the board if this new state is valid.
      if self.validate_board(&board) {
        idx_stack.push(self.nth_empty(&board, 0));
      }
    }

    return solutions;
  }

  fn nth_empty(&self, board: &Vec<BoardState>, n: usize) -> usize {
    let mut empty_count = 0;
    for i in 0..(self.size * self.size) {
      if board[i] == BoardState::EMPTY {
        empty_count += 1;

        if empty_count == n + 1 {
          return i;
        }
      }
    }

    return usize::MAX;
  }

  fn validate_board(&self, board: &Vec<BoardState>) -> bool {
    let state_max = self.size / 2;

    // No 3 in a row in each row, and max of size/2 of each state.
    for row in 0..self.size {
      let mut consecutive_count = 0;
      let mut consecutive_state = BoardState::EMPTY;
      let mut primary_count = 0;
      let mut secondary_count = 0;

      for col in 0..self.size {
        let cur_state = board[row * self.size + col];
        if cur_state == BoardState::EMPTY {
          consecutive_state = BoardState::EMPTY;
          consecutive_count = 0;
          continue;
        }

        if cur_state == BoardState::PRIMARY {
          primary_count += 1;
        } else {
          secondary_count += 1;
        }

        if cur_state == consecutive_state {
          consecutive_count += 1;
        } else {
          consecutive_state = cur_state;
          consecutive_count = 1;
        }

        if consecutive_count > 2 {
          return false;
        }
      }

      if primary_count > state_max || secondary_count > state_max {
        return false;
      }
    }

    // No 3 in a row in each col, and max of size/2 of each state
    for col in 0..self.size {
      let mut consecutive_count = 0;
      let mut consecutive_state = BoardState::EMPTY;
      let mut primary_count = 0;
      let mut secondary_count = 0;

      for row in 0..self.size {
        let cur_state = board[row * self.size + col];
        if cur_state == BoardState::EMPTY {
          consecutive_state = BoardState::EMPTY;
          consecutive_count = 0;
          continue;
        }

        if cur_state == BoardState::PRIMARY {
          primary_count += 1;
        } else {
          secondary_count += 1;
        }

        if cur_state == consecutive_state {
          consecutive_count += 1;
        } else {
          consecutive_state = cur_state;
          consecutive_count = 1;
        }

        if consecutive_count > 2 {
          return false;
        }
      }

      if primary_count > state_max || secondary_count > state_max {
        return false;
      }
    }

    // check for identical rows
    for row in 0..self.size {
      for comp_row in (row + 1)..self.size {
        let mut has_dupe = true;
        for col in 0..self.size {
          if board[row * self.size + col] != board[comp_row * self.size + col] {
            has_dupe = false;
            break;
          }

          if board[row * self.size + col] == BoardState::EMPTY {
            has_dupe = false;
            break;
          }
        }
        if has_dupe {
          return false;
        }
      }
    }

    // check for identical cols
    for col in 0..self.size {
      for comp_col in (col + 1)..self.size {
        let mut has_dupe = true;
        for row in 0..self.size {
          if board[row * self.size + col] != board[row * self.size + comp_col] {
            has_dupe = false;
            break;
          }

          if board[row * self.size + col] == BoardState::EMPTY {
            has_dupe = false;
            break;
          }
        }
        if has_dupe {
          return false;
        }
      }
    }

    return true;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn to_board(board: &Vec<i8>) -> Vec<BoardState> {
    return board
      .into_iter()
      .map(|state| match state {
        1 => BoardState::PRIMARY,
        2 => BoardState::SECONDARY,
        _ => BoardState::EMPTY,
      })
      .collect();
  }

  #[test]
  fn empty() {
    let validator = BoardGenerator { size: 6 };

    let empty: Vec<i8> = vec![
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
    ];
    let board = to_board(&empty);

    assert_eq!(true, validator.validate_board(&board));
  }

  #[test]
  fn three_in_a_row() {
    let validator = BoardGenerator { size: 6 };

    let three_row: Vec<i8> = vec![
      1, 1, 1, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
    ];

    let three_col: Vec<i8> = vec![
      2, 0, 0, 0, 0, 0, // no-fmt
      2, 0, 0, 0, 0, 0, // no-fmt
      2, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
    ];
    let board_row = to_board(&three_row);
    let board_col = to_board(&three_col);

    assert_eq!(false, validator.validate_board(&board_row));
    assert_eq!(false, validator.validate_board(&board_col));
  }

  #[test]
  fn too_many_in_row() {
    let validator = BoardGenerator { size: 6 };

    let too_many_in_row: Vec<i8> = vec![
      1, 1, 2, 1, 1, 2, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
    ];

    let too_many_in_col: Vec<i8> = vec![
      2, 0, 0, 0, 0, 0, // no-fmt
      2, 0, 0, 0, 0, 0, // no-fmt
      1, 0, 0, 0, 0, 0, // no-fmt
      2, 0, 0, 0, 0, 0, // no-fmt
      2, 0, 0, 0, 0, 0, // no-fmt
      1, 0, 0, 0, 0, 0, // no-fmt
    ];
    let board_row = to_board(&too_many_in_row);
    let board_col = to_board(&too_many_in_col);

    assert_eq!(false, validator.validate_board(&board_row));
    assert_eq!(false, validator.validate_board(&board_col));
  }

  #[test]
  fn identicals() {
    let validator = BoardGenerator { size: 6 };

    let same_rows: Vec<i8> = vec![
      2, 2, 1, 1, 2, 1, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      2, 2, 1, 2, 1, 1, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      2, 2, 1, 1, 2, 1, // no-fmt
    ];

    let same_rows_with_empties: Vec<i8> = vec![
      1, 1, 2, 2, 1, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      1, 1, 2, 2, 1, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
    ];

    let same_cols: Vec<i8> = vec![
      1, 1, 0, 1, 0, 0, // no-fmt
      1, 1, 0, 1, 0, 0, // no-fmt
      2, 2, 0, 2, 0, 0, // no-fmt
      2, 1, 0, 2, 0, 0, // no-fmt
      1, 2, 0, 1, 0, 0, // no-fmt
      2, 2, 0, 2, 0, 0, // no-fmt
    ];

    let same_cols_with_empties: Vec<i8> = vec![
      2, 0, 2, 0, 0, 0, // no-fmt
      2, 0, 2, 0, 0, 0, // no-fmt
      1, 0, 1, 0, 0, 0, // no-fmt
      1, 0, 1, 0, 0, 0, // no-fmt
      2, 0, 2, 0, 0, 0, // no-fmt
      0, 0, 0, 0, 0, 0, // no-fmt
    ];

    let board_row = to_board(&same_rows);
    let board_row_with_empties = to_board(&same_rows_with_empties);
    let board_col = to_board(&same_cols);
    let board_col_with_empties = to_board(&same_cols_with_empties);

    assert_eq!(false, validator.validate_board(&board_row));
    assert_eq!(true, validator.validate_board(&board_row_with_empties));
    assert_eq!(false, validator.validate_board(&board_col));
    assert_eq!(true, validator.validate_board(&board_col_with_empties));
  }

  #[test]
  fn valid() {
    let validator = BoardGenerator { size: 6 };

    let valid: Vec<i8> = vec![
      1, 1, 2, 2, 1, 2, // no-fmt
      1, 1, 2, 1, 2, 2, // no-fmt
      2, 2, 1, 1, 2, 1, // no-fmt
      2, 1, 1, 2, 1, 2, // no-fmt
      1, 2, 2, 1, 2, 1, // no-fmt
      2, 2, 1, 2, 1, 1, // no-fmt
    ];
    let board = to_board(&valid);

    assert_eq!(true, validator.validate_board(&board));
  }

  #[test]
  fn many_solutions() {
    let generator = BoardGenerator { size: 4 };
    let two_solutions: Vec<i8> = vec![
      0, 2, 1, 0, // no-fmt
      1, 1, 2, 2, // no-fmt
      2, 1, 2, 1, // no-fmt
      0, 2, 1, 0, // no-fmt
    ];
    let board = to_board(&two_solutions);
    assert_eq!(2, generator.count_solutions(&board));
  }

  #[test]
  fn one_solutions() {
    let generator = BoardGenerator { size: 4 };
    let two_solutions: Vec<i8> = vec![
      0, 1, 2, 0, // no-fmt
      1, 1, 2, 2, // no-fmt
      2, 2, 1, 1, // no-fmt
      0, 2, 1, 0, // no-fmt
    ];
    let board = to_board(&two_solutions);
    assert_eq!(1, generator.count_solutions(&board));
  }

  #[test]
  fn no_solutions() {
    let generator = BoardGenerator { size: 4 };
    let two_solutions: Vec<i8> = vec![
      0, 2, 2, 0, // no-fmt
      0, 0, 0, 0, // no-fmt
      0, 0, 0, 0, // no-fmt
      0, 2, 2, 0, // no-fmt
    ];
    let board = to_board(&two_solutions);
    assert_eq!(0, generator.count_solutions(&board));
  }
}
