use cursive::event::{Event, EventResult, Key, MouseButton, MouseEvent};
use cursive::theme::Color;
use cursive::vec::Vec2;
use cursive::view::{ScrollBase, View};
use cursive::Printer;
use std::collections::HashMap;

struct Row {
    start: usize,
    end: usize,
}

/// Syntax for CodeArea
///
/// Create your own set of symbols and words.
/// You can add words like 'for', 'in', ect.
/// and symbols like '+', '-', ect.
pub struct Syntax {
    symbols: HashMap<char, Color>,
    words: HashMap<String, Color>,
}

impl Default for Syntax {
    fn default() -> Self {
        Self {
            symbols: HashMap::new(),
            words: HashMap::new(),
        }
    }
}

impl Syntax {
    pub fn new() -> Self {
        Syntax::default()
    }
    pub fn add_word(mut self, word: &str, color: Color) -> Self {
        self.words.insert(word.to_string(), color);
        self
    }
    pub fn add_symbol(mut self, symbol: char, color: Color) -> Self {
        self.symbols.insert(symbol, color);
        self
    }
    pub fn add_one_color_words(mut self, words: &[&str], color: Color) -> Self {
        for &word in words {
            self.words.insert(word.to_string(), color);
        }
        self
    }
    pub fn add_one_color_symbols(mut self, symbols: &[char], color: Color) -> Self {
        for &symbol in symbols {
            self.symbols.insert(symbol, color);
        }
        self
    }
}

/// Multi-lines code editor.
///
/// CodeArea shows line numbers
/// and can highligh your code using
/// your syntax
pub struct CodeArea {
    syntax: Syntax,

    content: String,

    rows: Vec<Row>,

    enabled: bool,

    scrollbase: ScrollBase,

    last_size: Vec2,

    visible_cursor: usize,

    internal_cursor: usize,
}

impl CodeArea {
    pub fn new() -> Self {
        CodeArea {
            syntax: Syntax::new(),
            content: String::new(),
            rows: Vec::new(),
            enabled: true,
            scrollbase: ScrollBase::new().right_padding(0),
            last_size: Vec2::zero(),
            visible_cursor: 0,
            internal_cursor: 0,
        }
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
}

impl View for CodeArea {
    fn draw(&self, printer: &Printer) {}
    fn on_event(&mut self, event: Event) -> EventResult {
        let mut consumed = true;
        match event {
            // Input
            Event::Char(_) => unimplemented!(),
            Event::Key(Key::Tab) => unimplemented!(),
            Event::Key(Key::Enter) => unimplemented!(),
            // Erase
            Event::Ctrl(Key::Backspace) => unimplemented!(),
            Event::Key(Key::Backspace) => unimplemented!(),
            // Movement
            Event::Key(Key::Home) | Event::Ctrl(Key::Left) => unimplemented!(),
            Event::Key(Key::End) | Event::Ctrl(Key::Right) => unimplemented!(),
            Event::Ctrl(Key::Up) | Event::Ctrl(Key::Home) => unimplemented!(),
            Event::Ctrl(Key::Down) | Event::Ctrl(Key::End) => unimplemented!(),
            Event::Key(Key::Left) => unimplemented!(),
            Event::Key(Key::Right) => unimplemented!(),
            Event::Key(Key::Up) => unimplemented!(),
            Event::Key(Key::Down) => unimplemented!(),
            // TODO: Mouse events
            _ => consumed = false,
        }
        if consumed {
            EventResult::Consumed(None)
        } else {
            EventResult::Ignored
        }
    }
}
