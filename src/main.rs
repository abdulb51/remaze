/*
By: <Abdul Baig>
Date: 2026-06-01
Program Details: <a pacman and re:zero inspired maze game where you are trying to survive in a maze while being chased by 3 characters with different movement paths.>
*/

mod modules;
use crate::modules::collision::check_collision;
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::preload_image::LoadingScreenOptions;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use macroquad::input::KeyCode::Right;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "remaze".to_string(),
        window_width: 1080,
        window_height: 1080,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut img_lye = StillImage::new(
        "assets/lye.png",
        130.0, // width
        130.0, // height
        900.0, // x position
        70.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    let mut img_roy = StillImage::new(
        "assets/roy.png",
        150.0, // width
        150.0, // height
        700.0, // x position
        800.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    let mut img_rui = StillImage::new(
        "assets/rui.png",
        130.0, // width
        130.0, // height
        660.0, // x position
        270.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    let mut img_player = StillImage::new(
        "assets/subaru.png",
        150.0, // width
        150.0, // height
        10.0,  // x position
        40.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    let img_bg = StillImage::new(
        "assets/maze.png",
        1080.0, // width
        1080.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;

    const MOVE_SPEED: f32 = 200.0;

    let mut lye_x = 0.0;
    let mut lye_y = 0.0;

    let mut roy_x = 0.0;
    let mut roy_y = 0.0;

    let mut rui_x = 0.0;
    let mut rui_y = 0.0;

    let subaru_X = 0.0;
    let subaru_Y = 0.0;

    let mut subarupos = Label::new("x:{}, y:{}", 500.0, 80.0, 30);
    loop {
        clear_background(WHITE);
        img_bg.draw();
        draw_grid(50.0, BLACK);

        subarupos.draw();
        subarupos.set_text(format!("x:{:.1}, y:{:.1}", img_player.get_x(), img_player.get_y()));

        // Lye's "AI"
        let mut lye_pos = img_lye.pos();
        println!("Lye position: x:{:.1}, y:{:.1}", lye_pos.x, lye_pos.y);
        if lye_pos.x == 900.0 && lye_pos.y == 70.0 {
            lye_x = 0.0;
            lye_y = 1.0;
        }
        if lye_pos.x == 900.0 && lye_pos.y == 500.0 {
             lye_x  = -1.0;
             lye_y  = 0.0;

if lye_pos.x == 500.0 && lye_pos.y == 500.0 {
    lye_x = 0.0;
    lye_y = -1.0;

        }
          lye_pos.y += lye_y;
        lye_pos.x += lye_x;
        img_lye.set_position(lye_pos);


        // Roy's "AI"
        let mut roy_pos = img_roy.pos();
        {}

        // Rui's "AI"
        let mut rui_pos = img_rui.pos();
        {}

        // Direction to move in
        let mut move_dir = vec2(0.0, 0.0);

        // Keyboard input
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            move_dir.x += 2.0;
          
        }
        if is_key_pressed(Right)|| is_key_pressed(KeyCode::D)
        {
        img_player.set_image("assets/subaru.png").await; // Change to right looking image
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            move_dir.x -= 2.0;
           
           
           if is_key_pressed(KeyCode::Left)|| is_key_pressed(KeyCode::A){
            img_player.set_image("assets/subaruflip.png").await; // Change to left looking image
           }
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            move_dir.y += 2.0;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            move_dir.y -= 2.0;
        }

        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        // Apply movement based on frame time
        let movement = move_dir * MOVE_SPEED * get_frame_time();

        // Save old position in case of collision
        let old_pos = img_player.pos();

        // Move X first
        if movement.x != 0.0 {
            img_player.set_x(img_player.get_x() + movement.x);
            if check_collision(&img_player, &img_bg, 1) {
                img_player.set_x(old_pos.x); // Undo if collision happens
            }
        }

        // Move Y next
        if movement.y != 0.0 {
            img_player.set_y(img_player.get_y() + movement.y);
            if check_collision(&img_player, &img_bg, 1) {
                img_player.set_y(old_pos.y); // Undo if collision happens
            }
        }

        img_lye.draw();
        img_roy.draw();
        img_rui.draw();
        img_player.draw();

        next_frame().await;
    }
}
}