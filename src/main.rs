use macroquad::prelude::*;

use std::fs::read_to_string;

#[macroquad::main("game")]
async fn main() {
    let font = load_ttf_font("./assets/FiraCodeMono.ttf").await.unwrap();

    let text = read_to_string("./src/main.rs").unwrap();

    let bg_color = color_u8!(32, 35, 44, 255);
    let font_color = color_u8!(178, 184, 194, 255);
    let font_size = 14;
    let font = TextParams {
        font_size,
        font,
        color: font_color,
        ..Default::default()
    };
    loop {
        clear_background(bg_color);

        for (y, line) in text.lines().enumerate() {
            draw_text_ex(
                &line,
                12.0,
                (font_size as f32 * 1.3).floor() * (1 + y) as f32,
                font,
            );
        }

        next_frame().await
    }
}
