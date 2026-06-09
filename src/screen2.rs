use macroquad::prelude::*;
use crate::modules::text_button::TextButton;


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


    let btn_play = TextButton::new(
        540.0,
        1000.0,
        200.0,
        60.0,
        "PLAY",
        GREEN,
        DARKGREEN,
        30
    );
   
   
   
    loop {
        clear_background(DARKGRAY);

        if btn_play.click() {
            return "screen1".to_string();
        }
 if is_key_pressed(KeyCode::Space) {
            return "screen1".to_string();
        }
        next_frame().await;
    }
}