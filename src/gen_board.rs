use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
pub enum BoardState {
  EMPTY = 0,
  PRIMARY = 1,
  SECONDARY = 2,
}

pub struct BoardGenerator {
  pub size: usize,
}

impl BoardGenerator {
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
