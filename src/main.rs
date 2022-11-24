use macroquad::prelude::{
    clear_background, color_u8, draw_text_ex, get_char_pressed, load_ttf_font, next_frame, Color,
    TextParams,
};

use std::fs::read_to_string;

#[macroquad::main("game")]
async fn main() {
    let font = load_ttf_font("./assets/FiraCodeMono.ttf").await.unwrap();
    let mut game = Game::new(Font::new(font));
    while game.is_running {
        game.update();
        game.draw();
        next_frame().await
    }
}

struct Game {
    font: Font,
    text_to_type: String,
    text_typed: String,
    is_running: bool,
}

impl Game {
    pub fn new(font: Font) -> Self {
        let text = read_to_string("./src/main.rs").unwrap();

        Self {
            text_to_type: text,
            text_typed: String::new(),
            font,
            is_running: true,
        }
    }

    pub fn update(&mut self) {
        while let Some(character) = get_char_pressed() {
            if !character.is_ascii() {
                match character {
                    '\u{f028}' => self.text_typed.push('\n'),
                    '\u{f029}' => self.is_running = false,
                    _ => {
                        println!("{:?}", character);
                        continue;
                    }
                }
                // Backspace Esc Enter
            }
            self.text_typed.push(character);
        }
    }

    pub fn draw(&self) {
        let bg_color = color_u8!(32, 35, 44, 255);
        clear_background(bg_color);

        for (line_num, line) in self.text_to_type.lines().enumerate() {
            self.font.draw_line(12.0, line_num, line);
        }
        for (line_num, line) in self.text_typed.lines().enumerate() {
            self.font
                .draw(12.0, line_num, line, color_u8![255, 255, 255, 255]);
        }
    }
}

struct Font {
    font_size: u16,
    font: TextParams,
}

impl Font {
    pub fn new(font: macroquad::prelude::Font) -> Self {
        let font_color = color_u8!(178, 184, 194, 255);
        let font_size = 14;
        let font = TextParams {
            font_size,
            font,
            color: font_color,
            ..Default::default()
        };
        Self { font_size, font }
    }

    pub fn draw(&self, x: f32, line_num: usize, line: &str, color: Color) {
        let TextParams {
            font_size, font, ..
        } = self.font;
        let font = TextParams {
            font_size,
            font,
            color,
            ..Default::default()
        };
        draw_text_ex(
            &line,
            x,
            (self.font_size as f32 * 1.3).floor() * (1 + line_num) as f32,
            self.font,
        );
    }

    pub fn draw_line(&self, x: f32, line_num: usize, line: &str) {
        draw_text_ex(
            &line,
            x,
            (self.font_size as f32 * 1.3).floor() * (1 + line_num) as f32,
            self.font,
        );
    }
}
