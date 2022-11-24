use macroquad::prelude::{
    clear_background, color_u8, draw_text_ex, get_char_pressed, load_ttf_font, next_frame, Color,
    TextParams,
};
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};
use walkdir::{DirEntry, Error, WalkDir};

use std::{fs::read_to_string, path::PathBuf};

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
    highlights_orig: Vec<HighlightEvent>,
    text_typed: String,
    is_running: bool,
    possible_paths: Vec<PathBuf>,
    highlighter: Highlighter,
    highlighter_config: HighlightConfiguration,
}

impl Game {
    pub fn new(font: Font) -> Self {
        let mut possible_paths = Vec::new();
        for entry in WalkDir::new("..").into_iter().filter_map(|e| check_path(e)) {
            possible_paths.push(entry.into_path());
        }
        let path = possible_paths
            .get(fastrand::usize(..possible_paths.len()))
            .unwrap();
        let text = read_to_string(path).unwrap_or("Please press TAB!".to_owned());

        let highlight_names = &[
            "attribute",
            "constant",
            "function.builtin",
            "function",
            "keyword",
            "operator",
            "property",
            "punctuation",
            "punctuation.bracket",
            "punctuation.delimiter",
            "string",
            "string.special",
            "tag",
            "type",
            "type.builtin",
            "variable",
            "variable.builtin",
            "variable.parameter",
        ];

        let highlighter = Highlighter::new();

        let mut rust_config = HighlightConfiguration::new(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();

        rust_config.configure(highlight_names);

        Self {
            text_to_type: text,
            highlights_orig: Vec::new(),
            text_typed: String::new(),
            font,
            is_running: true,
            possible_paths,
            highlighter,
            highlighter_config: rust_config,
        }
    }

    pub fn update_highlighting(&mut self) {
        let text = &self.text_to_type;
        let highlights = self
            .highlighter
            .highlight(&self.highlighter_config, text.as_bytes(), None, |_| None)
            .unwrap();

        self.highlights_orig = highlights.map(|event| event.unwrap()).collect();
    }

    pub fn update(&mut self) {
        while let Some(character) = get_char_pressed() {
            if !character.is_ascii() {
                match character {
                    '\u{f028}' => self.text_typed.push('\n'),
                    '\u{f029}' => self.is_running = false,
                    '\u{f02b}' => self.next_text(),
                    _ => {
                        println!("{:?}", character);
                        continue;
                    }
                }
                // Backspace Esc Enter
            } else {
                self.text_typed.push(character);
            }
        }
    }

    pub fn next_text(&mut self) {
        let mut text = None;
        let mut emergency_stop = 0;
        while text.is_none() && emergency_stop < 100 {
            emergency_stop += 1;
            let path = self
                .possible_paths
                .get(fastrand::usize(..self.possible_paths.len()))
                .unwrap();
            text = read_to_string(path).map_or(None, |s| Some(s));
        }
        self.text_to_type = text.unwrap();
        self.text_typed = String::new();
        self.update_highlighting();
    }

    pub fn draw(&self) {
        let bg_color = color_u8!(32, 35, 44, 255);
        clear_background(bg_color);

        for (line_num, line) in self.text_to_type.lines().enumerate() {
            self.font.draw_line(12.0, line_num, line);
        }

        for event in &self.highlights_orig{
            match event {
                HighlightEvent::Source { start, end } => {
                    todo!();
                }
                HighlightEvent::HighlightStart(s) => {
                    todo!();
                }
                _ => (),
            }
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
            font,
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

fn check_path(e: Result<DirEntry, Error>) -> Option<DirEntry> {
    if e.is_err() {
        return None;
    }
    let e = unsafe { e.unwrap_unchecked() };
    if is_hidden(&e) {
        return None;
    }
    if is_rust_file(&e) {
        return Some(e);
    }
    None
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_rust_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".rs"))
        .unwrap_or(false)
}
