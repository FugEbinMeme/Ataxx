use std::fmt::{Display, Write};

#[derive(Copy, Clone, PartialEq)]
pub enum CellType {
    Empty,
    Blue, //Player 1
    Red,  //Player 2
    Solid,
}
pub type Position = (i16, i16);

#[derive(Copy, Clone)]
pub struct Ataxx {
    pub width: i16,
    pub height: i16,
    pub board: [[CellType; 7]; 7],
    pub p1_turn: bool,
    pub selected_piece: Option<Position>,
}

impl Ataxx {
    pub fn new() -> Self {
        let mut ataxx = Ataxx {
            width: 7,
            height: 7,
            board: [[CellType::Empty; 7]; 7],
            p1_turn: true,
            selected_piece: None,
        };

        //starting pieces for both players
        ataxx.board[0][0] = CellType::Blue;
        ataxx.board[6][6] = CellType::Blue;
        ataxx.board[6][0] = CellType::Red;
        ataxx.board[0][6] = CellType::Red;
        ataxx.board[3][3] = CellType::Solid;

        ataxx
    }

    pub fn select_cell(&mut self, selection: Position) {
        if self.selected_piece.is_none() {
            //selects a cell, or if the cell isn't a valid selection, deselects whatever is currently selected
            match (
                self.board[selection.0 as usize][selection.1 as usize],
                self.p1_turn,
            ) {
                (CellType::Blue, true) => self.selected_piece = Some(selection),
                (CellType::Blue, false) => self.selected_piece = None,
                (CellType::Red, true) => self.selected_piece = None,
                (CellType::Red, false) => self.selected_piece = Some(selection),
                _ => self.selected_piece = None,
            }
        } else {
            //if a cell is already selected, check for valid move on second selection

            //can't move to full tiles, if your second selection is occupied empty the selection and return
            if self.board[selection.0 as usize][selection.1 as usize] != CellType::Empty {
                self.selected_piece = None;
                return;
            }

            //bounds checking, theoretically not needed but just in case
            if selection.0 < 0
                || selection.0 >= self.width
                || selection.1 < 0
                || selection.1 >= self.height
            {
                self.selected_piece = None;
                return;
            }

            let previous_selection = self.selected_piece.unwrap();

            //check for duplicate or jump movement
            let dist = (selection.0 - previous_selection.0)
                .abs()
                .max((selection.1 - previous_selection.1).abs());
            //if the selected distance is >2, it isn't a valid selection
            if dist > 2 {
                self.selected_piece = None;
                return;
            }
            let dupe = match dist {
                1 => true,
                _ => false,
            };

            //check for valid cell
            if self.board[selection.0 as usize][selection.1 as usize] == CellType::Empty {
                self.place_piece(selection, dupe);
            }
        }
    }

    fn place_piece(&mut self, (x, y): Position, dupe: bool) {
        let color = if self.p1_turn {
            CellType::Blue
        } else {
            CellType::Red
        };

        let color_enemy = if color == CellType::Red {
            CellType::Blue
        } else {
            CellType::Red
        };
        self.board[x as usize][y as usize] = color;

        //if piece is not to be duplicated, set the old position to empty
        if !dupe {
            let (x, y) = self.selected_piece.unwrap();
            self.board[x as usize][y as usize] = CellType::Empty;
        }

        //set all occupied neighbors to color
        for i in -1..2 {
            for j in -1..2 {
                let (new_x, new_y) = (x + i, y + j);
                //bounds checking :(
                if new_x >= 0 && new_y >= 0 && new_x < self.width && new_y < self.width {
                    //set cell
                    if self.board[new_x as usize][new_y as usize] == color_enemy {
                        self.board[new_x as usize][new_y as usize] = color;
                    }
                }
            }
        }
        //toggle p1_turn
        if !self.skip_turn() {
            self.p1_turn = !self.p1_turn;
        }
        //clear selection
        self.selected_piece = None;
    }

    fn skip_turn(&self) -> bool {
        //inverted colors
        let color = if self.p1_turn {
            CellType::Red
        } else {
            CellType::Blue
        };

        //go through every board tile
        for x in 0..self.width {
            for y in 0..self.height {
                //for every piece that is yours...
                if self.board[x as usize][y as usize] != color {
                    continue;
                }
                //iterate all neighbors within 2 tiles and...
                for dx in -2..3 {
                    for dy in -2..3 {
                        let (new_x, new_y) = (x + dx, y + dy);
                        //bounds checking :(
                        if new_x >= 0 && new_y >= 0 && new_x < self.width && new_y < self.width {
                            //if a single valid move / empty square is found, return false
                            if self.board[new_x as usize][new_y as usize] == CellType::Empty {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        //if the entire board is iterated over and no tile of your own has a valid move, return true
        println!("Skipping Turn!");
        true
    }
}

impl Display for Ataxx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.board[x as usize][y as usize] {
                    CellType::Blue => f.write_str("🟦")?,
                    CellType::Red => f.write_str("🟥")?,
                    CellType::Empty => f.write_str("⬛")?,
                    CellType::Solid => f.write_str("❌")?,
                };
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}
