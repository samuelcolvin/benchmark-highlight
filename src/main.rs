use std::time::Instant;
use std::fs;

use syntect::parsing::{ParseState, SyntaxSet, SyntaxReference, ScopeStack};
use syntect::highlighting::{Highlighter, HighlightState, HighlightIterator, ThemeSet, Theme};
use syntect::util::LinesWithEndings;
use syntect::highlighting::FontStyle;

use ansi_term::Color::Fixed;
use ansi_term::{Style as AnsiStyle};



struct Highlight {
    syntax_set: SyntaxSet,
    syntax: SyntaxReference,
    theme: Theme,
}

impl Highlight {
    fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let syntax = syntax_set.find_syntax_by_extension("py").unwrap().clone();
        let ts = ThemeSet::load_defaults();
        Highlight {
            syntax_set,
            syntax,
            theme: ts.themes["base16-ocean.dark"].clone(),
        }
    }

    fn to_ansi(&self, source_code: String) -> String {
        let mut result = String::with_capacity(source_code.len() * 8);
        let highlighter = Highlighter::new(&self.theme);
        let mut highlight_state = HighlightState::new(&highlighter, ScopeStack::new());
        let mut parse_state = ParseState::new(&self.syntax);
        for line in LinesWithEndings::from(&source_code) {
            let ops = parse_state.parse_line(&line, &self.syntax_set);
            let iter = HighlightIterator::new(&mut highlight_state, &ops, &line, &highlighter);
            for (style, text) in iter {
                if !text.is_empty() {
                    let fg = style.foreground;
                    let mut color = AnsiStyle {
                        foreground: Some(Fixed(ansi_colours::ansi256_from_rgb((fg.r, fg.g, fg.b)))),
                        ..AnsiStyle::default()
                    };
                    if style.font_style.contains(FontStyle::BOLD) {
                        color = color.bold();
                    }
                    if style.font_style.contains(FontStyle::UNDERLINE) {
                        color = color.underline();
                    }

                    result.push_str(&color.paint(text).to_string());
                }
            }
        }
        result
    }
}

fn main() {
    let code = fs::read_to_string("/home/samuel/code/pydantic/pydantic/main.py").unwrap();
    let load_start = Instant::now();
    let hl = Highlight::new();
    let load_time = load_start.elapsed();
    let highlight_start = Instant::now();
    let ansi_code = hl.to_ansi(code);
    let highlight_time = highlight_start.elapsed();
    // println!("{}", ansi_code);
    println!("load time:          {:?}", load_time);
    println!("highlight time:     {:?}", highlight_time);
    println!("output ansi length: {}", ansi_code.len());
}
