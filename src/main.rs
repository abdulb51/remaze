/*
By: <Your Name Here>
Date: 2026-06-01
Program Details: <Program Description Here>
*/

mod modules;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;
use crate::modules::preload_image::TextureManager;
use crate::modules::preload_image::LoadingScreenOptions;
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
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
        100.0,  // width
        100.0,  // height
        700.0,  // x position
        800.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;


    let mut img_roy = StillImage::new(
        "assets/roy.png",
        100.0,  // width
        100.0,  // height
        700.0,  // x position
        800.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;


        let mut img_rui = StillImage::new(
        "assets/rui.png",
        100.0,  // width
        100.0,  // height
        700.0,  // x position
        800.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;


let mut img_player = StillImage::new(
        "assets/subaru.png",
        100.0,  // width
        100.0,  // height
        10.0,  // x position
        40.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

 let img_bg = StillImage::new(
        "assets/maze.png",
        1080.0,  // width
        1080.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;






    const MOVE_SPEED: f32 = 200.0;
    loop {
        clear_background(WHITE);

        img_bg.draw();

        draw_grid(50.0, BLACK);




// Direction to move in
    let mut move_dir = vec2(0.0, 0.0);

    // Keyboard input
    if is_key_down(KeyCode::Right) {
        move_dir.x += 1.0;
    }
    if is_key_down(KeyCode::Left) {
        move_dir.x -= 1.0;
    }
    if is_key_down(KeyCode::Down) {
        move_dir.y += 1.0;
    }
    if is_key_down(KeyCode::Up) {
        move_dir.y -= 1.0;
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
        if check_collision(&img_player, &img_bg, 1)  {
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
