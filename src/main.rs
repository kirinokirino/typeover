use macroquad::prelude::{
    clear_background, color_u8, draw_text_ex, get_char_pressed, load_ttf_font, next_frame, Color,
    TextParams,
};
use tree_sitter_highlight::{Highlight, HighlightConfiguration, HighlightEvent, Highlighter};
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
        for entry in WalkDir::new("..").into_iter().filter_map(check_path) {
            possible_paths.push(entry.into_path());
        }
        let path = possible_paths
            .get(fastrand::usize(..possible_paths.len()))
            .unwrap();
        let text = read_to_string(path).unwrap_or_else(|_| "Please press TAB!".to_owned());
        let highlight_names = &[
            "attribute",
            "comment",
            "constant.builtin",
            "constant",
            "embedded",
            "function.builtin",
            "function",
            "keyword",
            "number",
            "module",
            "operator",
            "punctuation.bracket",
            "punctuation.delimiter",
            "string.special",
            "string",
            "tag",
            "type",
            "type.builtin",
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

        let mut game = Self {
            text_to_type: text,
            highlights_orig: Vec::new(),
            text_typed: String::new(),
            font,
            is_running: true,
            possible_paths,
            highlighter,
            highlighter_config: rust_config,
        };
        game.update_highlighting();
        game
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
            text = read_to_string(path).ok();
        }
        self.text_to_type = text.unwrap();
        self.text_typed = String::new();
        self.update_highlighting();
    }

    pub fn draw(&mut self) {
        let bg_color = color_u8!(32, 35, 44, 255);
        clear_background(bg_color);

        let orig_text = &self.text_to_type;
        let mut last_style = Highlight(0);
        let mut line_num = 0;
        let mut x = self.font.width();
        for event in &self.highlights_orig {
            match event {
                HighlightEvent::Source { start, end } => {
                    let string =
                        String::from_utf8((orig_text.as_bytes()[*start..*end]).to_vec())
                            .unwrap();
                    for char in string.chars() {
                        match char {
                            '\n' => {
                            	line_num += 1;
                            	x = self.font.width();
                            },
                            '\t' => x += self.font.width() * 2.0,
                            ' ' => x += self.font.width(),
                            c => {
                                if !c.is_control() && !c.is_whitespace() {
                                    self.font.draw_line(x, line_num, &format!("{c}"));
                                    x += self.font.width();
                                }
                            }
                        }
                    }
                }
                HighlightEvent::HighlightStart(s) => {
                    if s == &last_style {
                        continue;
                    };
                    last_style = *s;
                    match s {
                        &Highlight(0) => self.font.change_color(color_from_xterm(124)),
                        &Highlight(1) => self.font.change_color(color_from_xterm(245)),
                        &Highlight(2) => self.font.change_color(color_from_xterm(94)),
                        &Highlight(3) => self.font.change_color(color_from_xterm(94)),
                        &Highlight(4) => self.font.change_color(color_from_xterm(136)),
                        &Highlight(5) => panic!(),
                        &Highlight(6) => self.font.change_color(color_from_xterm(26)),
                        &Highlight(7) => self.font.change_color(color_from_xterm(26)),
                        &Highlight(8) => self.font.change_color(color_from_xterm(56)),
                        &Highlight(9) => self.font.change_color(color_from_xterm(94)),
                        &Highlight(10) => self.font.change_color(color_from_xterm(136)),
                        &Highlight(11) => self.font.change_color(color_from_xterm(124)),
                        &Highlight(12) => self.font.change_color(color_from_xterm(239)),
                        &Highlight(13) => self.font.change_color(color_from_xterm(239)),
                        &Highlight(14) => self.font.change_color(color_from_xterm(239)),
                        &Highlight(15) => self.font.change_color(color_from_xterm(30)),
                        &Highlight(16) => self.font.change_color(color_from_xterm(28)),
                        &Highlight(17) => self.font.change_color(color_from_xterm(18)),
                        &Highlight(18) => self.font.change_color(color_from_xterm(23)),
                        &Highlight(19) => self.font.change_color(color_from_xterm(23)),
                        &Highlight(20) => self.font.change_color(color_from_xterm(23)),
                        &Highlight(21) => self.font.change_color(color_from_xterm(23)),
                        s => {
                            eprintln!("Unstyled style {s:?}");
                        }
                    }
                }
                _ => (),
            }
        }

        self.font.change_color(color_u8!(255, 255, 255, 255));
        for (line_num, line) in self.text_typed.lines().enumerate() {
            self.font.draw_line(self.font.width(), line_num, line);
        }
    }
}

fn color_from_xterm(xterm_id: usize) -> Color {
    let s = XTERM_COLORS[xterm_id];
    let r = u8::from_str_radix(&s[0..2], 16).unwrap();
    let g = u8::from_str_radix(&s[2..4], 16).unwrap();
    let b = u8::from_str_radix(&s[4..6], 16).unwrap();
    Color::from_rgba(r, g, b, 255)
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

    pub fn change_color(&mut self, color: Color) {
        let TextParams {
            font_size, font, ..
        } = self.font;
        self.font = TextParams {
            font_size,
            font,
            color,
            ..Default::default()
        };
    }

    pub fn draw_line(&self, x: f32, line_num: usize, line: &str) {
        draw_text_ex(
            line,
            x,
            (self.font_size as f32 * 1.3).floor() * (1 + line_num) as f32,
            self.font,
        );
    }

    pub fn width(&self) -> f32 {
        self.font_size as f32 * 0.615
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
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn is_rust_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".rs"))
        .unwrap_or(false)
}

static XTERM_COLORS: [&str; 256] = [
    "000000", "800000", "008000", "808000", "000080", "800080", "008080", "c0c0c0", "808080",
    "ff0000", "00ff00", "ffff00", "0000ff", "ff00ff", "00ffff", "ffffff", "000000", "00005f",
    "000087", "0000af", "0000d7", "0000ff", "005f00", "005f5f", "005f87", "005faf", "005fd7",
    "005fff", "008700", "00875f", "008787", "0087af", "0087d7", "0087ff", "00af00", "00af5f",
    "00af87", "00afaf", "00afd7", "00afff", "00d700", "00d75f", "00d787", "00d7af", "00d7d7",
    "00d7ff", "00ff00", "00ff5f", "00ff87", "00ffaf", "00ffd7", "00ffff", "5f0000", "5f005f",
    "5f0087", "5f00af", "5f00d7", "5f00ff", "5f5f00", "5f5f5f", "5f5f87", "5f5faf", "5f5fd7",
    "5f5fff", "5f8700", "5f875f", "5f8787", "5f87af", "5f87d7", "5f87ff", "5faf00", "5faf5f",
    "5faf87", "5fafaf", "5fafd7", "5fafff", "5fd700", "5fd75f", "5fd787", "5fd7af", "5fd7d7",
    "5fd7ff", "5fff00", "5fff5f", "5fff87", "5fffaf", "5fffd7", "5fffff", "870000", "87005f",
    "870087", "8700af", "8700d7", "8700ff", "875f00", "875f5f", "875f87", "875faf", "875fd7",
    "875fff", "878700", "87875f", "878787", "8787af", "8787d7", "8787ff", "87af00", "87af5f",
    "87af87", "87afaf", "87afd7", "87afff", "87d700", "87d75f", "87d787", "87d7af", "87d7d7",
    "87d7ff", "87ff00", "87ff5f", "87ff87", "87ffaf", "87ffd7", "87ffff", "af0000", "af005f",
    "af0087", "af00af", "af00d7", "af00ff", "af5f00", "af5f5f", "af5f87", "af5faf", "af5fd7",
    "af5fff", "af8700", "af875f", "af8787", "af87af", "af87d7", "af87ff", "afaf00", "afaf5f",
    "afaf87", "afafaf", "afafd7", "afafff", "afd700", "afd75f", "afd787", "afd7af", "afd7d7",
    "afd7ff", "afff00", "afff5f", "afff87", "afffaf", "afffd7", "afffff", "d70000", "d7005f",
    "d70087", "d700af", "d700d7", "d700ff", "d75f00", "d75f5f", "d75f87", "d75faf", "d75fd7",
    "d75fff", "d78700", "d7875f", "d78787", "d787af", "d787d7", "d787ff", "d7af00", "d7af5f",
    "d7af87", "d7afaf", "d7afd7", "d7afff", "d7d700", "d7d75f", "d7d787", "d7d7af", "d7d7d7",
    "d7d7ff", "d7ff00", "d7ff5f", "d7ff87", "d7ffaf", "d7ffd7", "d7ffff", "ff0000", "ff005f",
    "ff0087", "ff00af", "ff00d7", "ff00ff", "ff5f00", "ff5f5f", "ff5f87", "ff5faf", "ff5fd7",
    "ff5fff", "ff8700", "ff875f", "ff8787", "ff87af", "ff87d7", "ff87ff", "ffaf00", "ffaf5f",
    "ffaf87", "ffafaf", "ffafd7", "ffafff", "ffd700", "ffd75f", "ffd787", "ffd7af", "ffd7d7",
    "ffd7ff", "ffff00", "ffff5f", "ffff87", "ffffaf", "ffffd7", "ffffff", "080808", "121212",
    "1c1c1c", "262626", "303030", "3a3a3a", "444444", "4e4e4e", "585858", "606060", "666666",
    "767676", "808080", "8a8a8a", "949494", "9e9e9e", "a8a8a8", "b2b2b2", "bcbcbc", "c6c6c6",
    "d0d0d0", "dadada", "e4e4e4", "eeeeee",
];
