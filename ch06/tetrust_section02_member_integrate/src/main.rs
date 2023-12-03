mod block;
mod game;

use block::BLOCKS;
use game::*;
use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let game = Arc::new(Mutex::new(Game::new()));

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");
    // フィールドを描画
    draw(&game.lock().unwrap());

    // 自然落下処理
    {
        let game = Arc::clone(&game);
        let _ = thread::spawn(move || {
            loop {
                // 1秒間スリーブする
                thread::sleep(time::Duration::from_millis(1000));
                // 自然落下
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                if !is_collision(&game.field, &new_pos, game.block) {
                    // posの座標を更新
                    game.pos = new_pos;
                } else {
                    // ブロックをフィールドに固定
                    let gy = game.pos.y;
                    let gx = game.pos.x;
                    for y in 0..4 {
                        for x in 0..4 {
                            if BLOCKS[game.block as usize][y][x] == 1 {
                                game.field[y + gy][x + gx] = 1;
                            }
                        }
                    }
                    // ラインの削除処理
                    for y in 1..FIELD_HEIGHT - 1 {
                        let mut can_erase = true;
                        for x in 1..FIELD_WIDTH - 1 {
                            if game.field[y][x] == 0 {
                                can_erase = false;
                                break;
                            }
                        }
                        if can_erase {
                            for y2 in (2..=y).rev() {
                                game.field[y2] = game.field[y2 - 1];
                            }
                        }
                    }
                    // posの座標を初期値へ
                    game.pos = Position::init();
                    game.block = rand::random();
                }
                // フィールドを描画
                draw(&game);
            }
        });
    }

    // キー入力処理
    let g = Getch::new();
    loop {
        // `q`キーでループを抜ける
        // キー入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or_else(|| game.pos.x),
                    y: game.pos.y,
                };
                if !is_collision(&game.field, &new_pos, game.block) {
                    // posの座標を更新
                    game.pos = new_pos;
                }
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                if !is_collision(&game.field, &new_pos, game.block) {
                    // posの座標を更新
                    game.pos = new_pos;
                }
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                if !is_collision(&game.field, &new_pos, game.block) {
                    // posの座標を更新
                    game.pos = new_pos;
                }
                draw(&game);
            }
            Ok(Key::Char('q')) => {
                // カーソルを再表示
                println!("\x1b[?25h");
                return;
            }
            _ => (), // 何もしない
        }
    }
}
