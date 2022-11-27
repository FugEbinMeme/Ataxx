use bot::Redesia;
use macroquad::prelude::*;

mod bot;
mod game;

use crate::game::*;

fn ataxx_game() -> Conf {
    Conf {
        window_title: "Ataxx".to_owned(),
        window_width: 448,
        window_height: 480,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(ataxx_game)]
async fn main() {
    let mut ataxx = Ataxx::new();

    let mut use_ai = true;

    let mut redesia = Redesia::new(&ataxx);

    loop {
        let size_w = screen_width() / 7.0; //64 pixels per tile
        let size_h = (screen_height() - 32.0) / 7.0; //32 pixels for bottom bar
        clear_background(BLACK);

        //toggle bot usage on / off
        if is_key_pressed(miniquad::KeyCode::B) {
            use_ai = !use_ai;
        }

        if use_ai && ataxx.p1_turn == false {
            let (_, from, to) = redesia.step(ataxx);
            ataxx.select_cell((from.0 as i16, from.1 as i16));
            ataxx.select_cell((to.0 as i16, to.1 as i16));
        }

        if is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
            if use_ai == false || ataxx.p1_turn == true {
                let board_loc = noramlize_loc();

                //clicking the bottom bar will result in a normalized location of 7, which is OoB
                if board_loc.1 < 7 {
                    ataxx.select_cell(board_loc);
                }
            }
        }

        if is_mouse_button_pressed(macroquad::input::MouseButton::Right) {
            if use_ai == false || ataxx.p1_turn == true {
                ataxx.selected_piece = None;
            }
        }

        if is_key_pressed(miniquad::KeyCode::R) {
            ataxx.board = [[CellType::Empty; 7]; 7];

            ataxx.board[0][0] = CellType::Blue;
            ataxx.board[6][6] = CellType::Blue;
            ataxx.board[6][0] = CellType::Red;
            ataxx.board[0][6] = CellType::Red;
            ataxx.board[3][3] = CellType::Solid;
        }

        //DEBUG FUNCTION BLOCK START
        // if is_key_pressed(miniquad::KeyCode::D) {
        //     let board_loc = noramlize_loc();
        //     ataxx.board[board_loc.0 as usize][board_loc.1 as usize] = CellType::Blue;
        // }

        // if is_key_pressed(miniquad::KeyCode::E) {
        //     let board_loc = noramlize_loc();
        //     ataxx.board[board_loc.0 as usize][board_loc.1 as usize] = CellType::Red;
        // }
        //DEBUG FUNCTION BLOCK END

        //draw the board
        let selection = ataxx.selected_piece;

        for x in 0..ataxx.width {
            for y in 0..ataxx.height {
                let position = ataxx.board[x as usize][y as usize];

                let mut color = match position {
                    CellType::Blue => BLUE,
                    CellType::Red => RED,
                    CellType::Empty => DARKGRAY,
                    CellType::Solid => BLACK,
                };
                //selected piece is colored differently
                if Some((x, y)) == selection {
                    color = match position {
                        CellType::Blue => SKYBLUE,
                        CellType::Red => PINK,
                        _ => unreachable!(),
                    }
                }

                if selection.is_some() {
                    let sel = selection.unwrap();
                    let dist = (x - sel.0).abs().max((y - sel.1).abs());

                    if dist <= 2 && color == DARKGRAY {
                        color = GRAY;
                    }
                }
                draw_rectangle(x as f32 * size_w, y as f32 * size_h, size_w, size_h, color)
            }
        }

        //draw matrix outline
        for x in 0..ataxx.width {
            draw_line(
                x as f32 * size_w,
                0.0,
                x as f32 * size_w,
                screen_height() - 32.0,
                2.0,
                BLACK,
            );
        }
        for y in 0..ataxx.height {
            draw_line(
                0.0,
                y as f32 * size_h,
                screen_width(),
                y as f32 * size_h,
                2.0,
                BLACK,
            );
        }

        //draw information bar
        let mut blue = 0;
        let mut red = 0;
        let mut empty = 0;

        for x in 0..ataxx.width {
            for y in 0..ataxx.height {
                match ataxx.board[x as usize][y as usize] {
                    CellType::Blue => blue += 1,
                    CellType::Red => red += 1,
                    CellType::Empty => empty += 1,
                    _ => (),
                }
            }
        }

        let b = format!("Blue: {blue}");
        let r = format!("Red: {red}");
        let e = format!("Empty: {empty}");
        draw_text(&b, 10.0, 470.0, 30.0, BLUE);
        draw_text(&r, 180.0, 470.0, 30.0, RED);
        if empty == 0 {
            if blue > red {
                draw_text("Blue Wins!", 315.0, 470.0, 30.0, BLUE);
            } else {
                draw_text("Red Wins!", 315.0, 470.0, 30.0, RED);
            }
        } else {
            draw_text(&e, 315.0, 470.0, 30.0, GRAY);
        }

        next_frame().await
    }
}

pub fn noramlize_loc() -> (i16, i16) {
    let mut location = macroquad::input::mouse_position();
    location.0 /= screen_width() / 7.0;
    location.1 /= (screen_height() - 32.0) / 7.0;
    (location.0.trunc() as i16, location.1.trunc() as i16)
}
