use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub struct BingoState {
    pub input_numbers: Vec<usize>,
    boards: Vec<BingoBoard>,
    last_number: usize,
}

impl BingoState {
    pub fn from_strs(
        chunk_size: usize,
        mut input_iter: impl Iterator<Item = String>,
    ) -> BingoState {
        let first_line = input_iter.next().unwrap();
        BingoState {
            input_numbers: first_line
                .split(',')
                .flat_map(|s| s.parse::<usize>())
                .collect(),
            boards: input_iter
                .chunks(chunk_size)
                .into_iter()
                .map(|s| BingoBoard::from_strs(s.collect()))
                .collect(),
            last_number: 0,
        }
    }

    pub fn run_until(self, predicate: fn(&BingoState) -> bool) -> BingoState {
        self.input_numbers
            .to_owned()
            .into_iter()
            .fold_while(self, |bs, next_number| {
                let next_state = bs.handle_number(next_number);
                if predicate(&next_state) {
                    Done(next_state)
                } else {
                    Continue(next_state)
                }
            })
            .into_inner()
    }

    pub fn handle_number(self, number: usize) -> BingoState {
        let next_boards: Vec<BingoBoard> = self
            .boards
            .into_iter()
            .map(|bb| bb.handle_number(number))
            .collect();
        BingoState {
            input_numbers: self.input_numbers,
            boards: next_boards,
            last_number: number,
        }
    }

    pub fn any_complete(&self) -> bool {
        self.boards.iter().any(|bb| bb.complete())
    }

    pub fn all_complete(&self) -> bool {
        self.boards.iter().all(|bb| bb.complete())
    }

    pub fn multiply_complete_sum_unmarked_by_last_number(&self) -> Option<usize> {
        self.boards
            .iter()
            .find(|bb| bb.complete() && self.last_number == bb.last_match.unwrap())
            .map(|complete_bb| complete_bb.sum_unmarked() * complete_bb.last_match.unwrap())
    }
}

struct Position {
    row: usize,
    col: usize,
}

pub struct BingoBoard {
    height: usize,
    width: usize,
    board: Vec<usize>,
    row_status: Vec<usize>,
    col_status: Vec<usize>,
    last_match: Option<usize>,
}

impl BingoBoard {
    pub fn new(height: usize, width: usize, board: Vec<usize>) -> BingoBoard {
        BingoBoard {
            height,
            width,
            board,
            row_status: vec![(1 << width) - 1; height],
            col_status: vec![(1 << height) - 1; width],
            last_match: None,
        }
    }

    pub fn from_strs(strs: Vec<String>) -> BingoBoard {
        let height = strs.len();
        let width = strs[0].split_whitespace().count();
        let board: Vec<usize> = strs
            .iter()
            .map(|v| v.split_whitespace().map(|s| s.parse::<usize>()).flatten())
            .flatten()
            .collect();
        BingoBoard::new(height, width, board)
    }

    fn locate_number(&self, number: usize) -> Option<Position> {
        self.board
            .iter()
            .position(|&n| n == number)
            .map(|position| Position {
                row: position / self.height,
                col: position % self.height,
            })
    }

    fn mark_pos(self, position: Position, number: usize) -> BingoBoard {
        let mut new_row_status = self.row_status;
        let mut new_col_status = self.col_status;
        new_row_status[position.row] &= !(1 << position.col);
        new_col_status[position.col] &= !(1 << position.row);

        BingoBoard {
            height: self.height,
            width: self.width,
            board: self.board,
            row_status: new_row_status,
            col_status: new_col_status,
            last_match: Some(number),
        }
    }

    fn pos_marked(&self, index: usize) -> bool {
        let row = index / self.height;
        let col = index % self.height;
        self.row_status[row] & (1 << col) == 0
    }

    pub fn handle_number(self, number: usize) -> BingoBoard {
        if self.complete() {
            self
        } else if let Some(position) = self.locate_number(number) {
            self.mark_pos(position, number)
        } else {
            self
        }
    }

    pub fn complete(&self) -> bool {
        self.row_status
            .iter()
            .chain(self.col_status.iter())
            .any(|&s| s == 0)
    }

    pub fn sum_unmarked(&self) -> usize {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, n)| if self.pos_marked(i) { None } else { Some(n) })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bingo() {
        let board = vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ];
        assert_eq!(
            BingoBoard::new(5, 5, board.to_owned())
                .handle_number(6)
                .handle_number(10)
                .handle_number(3)
                .handle_number(18)
                .handle_number(5)
                .complete(),
            true
        );
        assert_eq!(
            BingoBoard::new(5, 5, board.to_owned())
                .handle_number(17)
                .handle_number(23)
                .handle_number(14)
                .handle_number(20)
                .handle_number(3)
                .complete(),
            true
        );
        assert_eq!(
            BingoBoard::new(5, 5, board.to_owned())
                .handle_number(1)
                .handle_number(2)
                .handle_number(3)
                .handle_number(4)
                .handle_number(5)
                .complete(),
            false
        )
    }
}
