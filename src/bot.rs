use crate::game::*;
//Position
type Pos = (u8, u8);
//Move: eval, from position, to position
type Mov = (i32, Pos, Pos);

//Pun on the country Rhodesia
pub struct Redesia {
    pub width: i16,
    pub height: i16,
    pub board: [[CellType; 7]; 7],
    pub positions: Vec<Mov>,
}

impl Redesia {
    pub fn new(ataxx: &Ataxx) -> Self {
        let redesia = Redesia {
            width: ataxx.width,
            height: ataxx.height,
            board: ataxx.board,
            positions: Vec::new(),
        };
        redesia
    }

    //steps once in thinking. or whatever. I'm just trying to keep as much bot code in the bot file
    pub fn step(&mut self, state: Ataxx) -> Mov {
        self.positions.clear();
        //update the internal board with current state
        self.board = state.board;

        self.collect_moves(&state);

        let mut best_move = (0, (0, 0), (0, 0));
        for index in &self.positions {
            if index.0 > best_move.0 {
                //bot is deterministic, will always choose the first move that maximizes eval
                best_move = *index;
            }
        }
        best_move
    }

    //The eval function simply counts how many red squares there are
    fn evaluate_move(&self, state: Ataxx, from: Position, to: Position) -> i32 {
        let mut eval = 0;
        let mut copy = state.clone();

        copy.select_cell(from);
        copy.select_cell(to);

        for x in 0..self.width {
            for y in 0..self.height {
                if copy.board[x as usize][y as usize] == CellType::Red {
                    eval += 1;
                }
            }
        }
        eval
    }
    //populates the positions vector with all valid moves
    fn collect_moves(&mut self, state: &Ataxx) {
        //The bot is always player 2, Red
        let color = CellType::Red;

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
                            //for each valid move...
                            if self.board[new_x as usize][new_y as usize] == CellType::Empty {
                                //evaluate the score...
                                let eval = self.evaluate_move(*state, (x, y), (new_x, new_y));
                                //and push the info to the positions vector
                                self.positions.push((
                                    eval,
                                    (x as u8, y as u8),
                                    (new_x as u8, new_y as u8),
                                ))
                            }
                        }
                    }
                }
            }
        }
    }
}
