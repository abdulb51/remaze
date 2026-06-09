/*
By: <Abdul Baig>
Date: 2026-06-01
Program Details: <a pacman and re:zero inspired maze game where you are trying to survive in a maze while being chased by 3 characters with different movement paths.>
*/


use crate::modules::collision::check_collision;
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::preload_image::LoadingScreenOptions;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
use macroquad::prelude::KeyCode::Right;

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

pub async fn run() -> String {


    
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
        200.0, // width
        200.0, // height
        860.0, // x position
        810.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    let mut img_rui = StillImage::new(
        "assets/rui.png",
        130.0, // width
        130.0, // height
        650.0, // x position
        260.0, // y position
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

    const MOVE_SPEED: f32 = 220.0;

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
        //println!("Lye position: x:{:.1}, y:{:.1}", lye_pos.x, lye_pos.y);
        if lye_pos.x == 900.0 && lye_pos.y == 70.0{
            lye_x = 0.0;
            lye_y = 2.0;
        }

        if lye_pos.x == 900.0 && lye_pos.y == 500.0{
             lye_x  = -2.0;
             lye_y  = 0.0;
            }

if lye_pos.x == 500.0 && lye_pos.y == 500.0 {
    lye_x = 0.0;
    lye_y = -2.0;
}

if lye_pos.x == 500.0 && lye_pos.y == 450.0 {
    lye_x = -2.0;
    lye_y = 0.0;
}

if lye_pos.x == 330.0 && lye_pos.y == 450.0 {
    lye_x = 0.0;
    lye_y = 2.0;
}

if lye_pos.x == 330.0 && lye_pos.y == 620.0 {
    lye_x = -2.0;
    lye_y = 0.0;
}

if lye_pos.x == 26.0 && lye_pos.y == 620.0 {
    lye_x = 0.0;
    lye_y = -2.0;
}

if lye_pos.x == 26.0 && lye_pos.y == 70.0 {
    lye_x = 2.0;
    lye_y = 0.0;
}

          lye_pos.y += lye_y;
        lye_pos.x += lye_x;
        img_lye.set_position(lye_pos);


        // Roy's "AI"
        let mut roy_pos = img_roy.pos();
         //println!("Roy position: x:{:.1}, y:{:.1}", roy_pos.x, roy_pos.y);
        if roy_pos.x == 860.0 && roy_pos.y == 810.0 {
    roy_x = -2.0;
    roy_y = 0.0;
        }

  if roy_pos.x == 450.0 && roy_pos.y == 810.0 {
    roy_x = 0.0;
    roy_y = -2.0;

        }
         if roy_pos.x == 450.0 && roy_pos.y == 440.0 {
    roy_x = -2.0;
    roy_y = 0.0;
        }

     if roy_pos.x == 300.0 && roy_pos.y == 440.0 {
    roy_x = 0.0;
    roy_y  = 2.0;
        }

        if roy_pos.x == 300.0 && roy_pos.y == 800.0 {
    roy_x = -2.0;
    roy_y = 0.0;
        }

         if roy_pos.x == 30.0 && roy_pos.y == 800.0 {
    roy_x = 0.0;
    roy_y  = 2.0;
        }
      if roy_pos.x == 30.0 && roy_pos.y == 850.0 {
    roy_x = 2.0;
    roy_y = 0.0;
        }
    if roy_pos.x == 860.0 && roy_pos.y == 850.0 {
    roy_x = 0.0;
    roy_y = -2.0;
        }

       roy_pos.y += roy_y;
        roy_pos.x += roy_x;
        img_roy.set_position(roy_pos);

        // Rui's "AI"
        let mut rui_pos = img_rui.pos();
         //println!("Rui position: x:{:.1}, y:{:.1}", rui_pos.x, rui_pos.y);
        if rui_pos.x == 650.0 && rui_pos.y == 260.0 {
    rui_x = 1.0;
    rui_y = 0.0;
}

if rui_pos.x == 900.0 && rui_pos.y == 260.0 {
    rui_x = 0.0;
    rui_y = 1.0;
}

if rui_pos.x == 900.0 && rui_pos.y == 450.0 {
    rui_x = -1.0;
    rui_y = 0.0;
}


if rui_pos.x == 420.0 && rui_pos.y == 450.0 {
    rui_x = 0.0;
    rui_y = -1.0;
}

if rui_pos.x == 420.0 && rui_pos.y == 65.0 {
    rui_x = 1.0;
    rui_y = 0.0;
}


if rui_pos.x == 905.0 && rui_pos.y == 65.0 {
    rui_x = 0.0;
    rui_y = 1.0;
}

if rui_pos.x == 905.0 && rui_pos.y == 455.0 {
    rui_x = -1.0;
    rui_y = 0.0;
}

if rui_pos.x == 300.0 && rui_pos.y == 455.0 {
    rui_x = 0.0;
    rui_y = -1.0;
}
if rui_pos.x == 300.0 && rui_pos.y == 260.0 {
    rui_x = 1.0;
    rui_y = 0.0;
}

          rui_pos.y += rui_y;
        rui_pos.x += rui_x;
        img_rui.set_position(rui_pos);


        // check for collision with player
        if check_collision(&img_player, &img_lye, 0)
        {
            
           

        }

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

