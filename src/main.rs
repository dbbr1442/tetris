#![windows_subsystem = "windows"]

mod game_handler;
mod rect;
mod piece;
mod block;

use std::time::{SystemTime, UNIX_EPOCH};

use game_handler::{Game, MoveDirection};
use macroquad::{audio::{load_sound_from_bytes, play_sound, PlaySoundParams}, prelude::*, rand::srand};

const GREY: Color = GRAY; // i refuse to spell grey that way

fn config() -> Conf {
    Conf {
        window_title: "Tetris".to_string(),
        window_height: 1200,
        window_width: 1200,
        fullscreen: false,
        window_resizable: false,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    let korbeiniki = load_sound_from_bytes(include_bytes!("resources/korobeiniki.wav")).await.unwrap();
    let mut font = load_ttf_font_from_bytes(include_bytes!("resources/font.ttf")).unwrap();

    let sound_params = PlaySoundParams {
        looped: true,
        volume: 1.0,
    };
    play_sound(&korbeiniki, sound_params);

    font.set_filter(FilterMode::Nearest);

    srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

    let mut game = Game::new_game();

    'top_level: loop {
        clear_background(BLACK);
        
        for key in get_keys_pressed() {
            match key {
                KeyCode::A => {game.move_piece(MoveDirection::Left);},
                KeyCode::D => {game.move_piece(MoveDirection::Right);},
                KeyCode::W => {game.rotate_piece();},
                KeyCode::S => {game.move_piece(MoveDirection::Down);},
                KeyCode::Space => {
                    'drop: loop {
                        if game.move_piece(MoveDirection::Down) {
                            break 'drop;
                        }
                    }
                },
                KeyCode::C => game = game.hold_piece(),
                KeyCode::Enter => {
                    if game.lost {
                        game = Game::new_game();
                    }
                },
                KeyCode::Escape => break 'top_level,
                _ => (), 
            }
        }

        for piece in game.playfield.iter() {
            for block in piece.shape.iter() {
                let (x, y) = block.location; 
                let (x, y) = (x as f32, y as f32);
                draw_rectangle(x*60.0, y*60.0, 60.0, 60.0, piece.color);
            }
        }

        for block in game.inplay.shape.iter() {
                let (x, y) = block.location; 
                let (x, y) = (x as f32, y as f32);
                draw_rectangle(x*60.0, y*60.0, 60.0, 60.0, game.inplay.color);
        }

        for x in 0..=10 {
            draw_rectangle((x as f32*60.0)-1.0, 0.0, 2.0,1200.0, GREY);
        }

        for y in 0..=20 {
            draw_rectangle(0.0, (y as f32*60.0)-1.0, 600.0,2.0, GREY);
        }

        let next_piece = game.next_piece().as_rects();
        for rect in next_piece.0.iter() {
            let (x, y, w, h) = rect.get_data();
            draw_rectangle(x, y, w, h, next_piece.1);
        }

        if game.timer.elapsed().unwrap() > std::time::Duration::from_secs(1) && (!game.lost) {
            let collision = game.move_piece(MoveDirection::Down);
            if collision {
                game = game.place_piece();
                game.check_lose();
                game.check_line();
            }
            game.timer = SystemTime::now();
        }
        
        text_helper(&font, 128, 900.0, 150.0, "SCORE");
        text_helper(&font, 128, 900.0, 250.0, &game.score.to_string());

        text_helper(&font, 90, 900.0, 650.0, "NEXT PIECE");

        if game.lost {
            clear_background(BLACK);
            text_helper(&font, 200, 600.0, 200.0, "YOU  LOST");

            text_helper(&font, 120, 600.0, 600.0, "TRY AGAIN");
            text_helper(&font, 120, 600.0, 700.0, "ENTER");

            text_helper(&font, 120, 600.0, 900.0, "EXIT");
            text_helper(&font, 120, 600.0, 1000.0, "ESC");
        }

        next_frame().await
    }
}

fn text_helper(font: &Font, size: u16, x: f32, y: f32, text: &str) {
        let text_params = TextParams {
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color: WHITE,
            font: Some(font),
            font_size: size,
        };

        let center = get_text_center(text, Some(font), size, 1.0, 0.0);
        draw_text_ex(text, x-center.x, y-center.y, text_params);
}
