use cursive::event::{Event, EventResult, Key};
use cursive::theme::Color;
use cursive::vec::Vec2;
use cursive::view::{ScrollBase, View};
use cursive::Printer;
use std::collections::HashMap;
use std::collections::VecDeque;

struct Row {
    start: usize,
    end: usize,
}

/// Syntax for CodeArea
///
/// Create your own set of symbols and words.
/// You can add words like 'for', 'in', ect.
/// and symbols like '+', '-', ect.
/// As well as TextArea, CodeArea should be wrapped by BoxView
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
    pub fn add_words(mut self, dictionary: &[(&str, Color)]) -> Self {
        for (word, color) in dictionary {
            self.words.insert(word.to_string(), *color);
        }
        self
    }
    pub fn add_symbols(mut self, dictionary: &[(char, Color)]) -> Self {
        for (symbol, color) in dictionary {
            self.symbols.insert(*symbol, *color);
        }
        self
    }
}

enum HistoryEvent {
    Erase(String),
    Type(String),
    Moved(usize),
}

struct History {
    sequence: VecDeque<HistoryEvent>,
    counter: usize,
}

impl History {
    fn new() -> Self {
        Self {
            sequence: VecDeque::new(),
            counter: 0,
        }
    }
    fn erase_used(&mut self) {
        while self.counter > 0 {
            self.sequence.pop_front();
            self.counter -= 1;
        }
    }
}

/// Multi-lines code editor.
///
/// CodeArea shows line numbers
/// and can highligh your code using
/// your syntax
pub struct CodeArea {
    history: History,

    syntax: Syntax,

    content: String,

    rows: Vec<Row>,

    enabled: bool,

    scrollbase: ScrollBase,

    last_size: Vec2,

    cursor: usize,
}

// Public interface
impl CodeArea {
    pub fn new() -> Self {
        CodeArea {
            history: History::new(),
            syntax: Syntax::new(),
            content: String::new(),
            rows: Vec::new(),
            enabled: true,
            scrollbase: ScrollBase::new().right_padding(0),
            last_size: Vec2::zero(),
            cursor: 0,
        }
    }
    pub fn add_syntax(mut self, syntax: Syntax) -> Self {
        self.syntax = syntax;
        self
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
}

// Text manage
impl CodeArea {
    fn insert(&mut self, ch: char) {
        self.content.insert(self.cursor, ch);
        self.cursor += 1;
        self.history.erase_used();
        match self.history.sequence.front_mut() {
            Some(HistoryEvent::Type(ref mut typed)) => typed.push(ch),
            _ => {
                let mut new_typed = String::new();
                new_typed.push(ch);
                self.history
                    .sequence
                    .push_front(HistoryEvent::Type(new_typed));
            }
        }
    }
    fn erase_symbol(&mut self) {
        if self.cursor != 0 {
            let erased_char = self.content.remove(self.cursor);
            self.cursor -= 1;
            self.history.erase_used();
            match self.history.sequence.front_mut() {
                Some(HistoryEvent::Erase(ref mut erased)) => erased.push(erased_char),
                _ => {
                    let mut new_erased = String::new();
                    new_erased.push(erased_char);
                    self.history
                        .sequence
                        .push_front(HistoryEvent::Erase(new_erased));
                }
            }
        };
    }
    fn erase_line(&mut self) {
        if self.cursor != 0 {
            let mut erased_string = String::new();
            let mut ch = ' ';
            while ch != '\n' && self.cursor > 0 {
                ch = self.content.remove(self.cursor);
                erased_string.push(ch);
                self.cursor -= 1;
            }
            self.history.erase_used();
            match self.history.sequence.front_mut() {
                Some(HistoryEvent::Erase(ref mut erased)) => erased.push_str(&erased_string),
                _ => self
                    .history
                    .sequence
                    .push_front(HistoryEvent::Erase(erased_string)),
            }
        }
    }
}

// Cursor manage
impl CodeArea {}

// Edit
impl CodeArea {
    fn undo(&mut self) {
        match self.history.sequence.front() {
            Some(event) => {
                self.history.counter += 1;
                match event {
                    HistoryEvent::Erase(erased) => {
                        let erased = erased.chars().rev().collect::<String>();
                        self.content.insert_str(self.cursor, &erased);
                    }
                    HistoryEvent::Type(typed) => {
                        self.content
                            .replace_range(self.cursor - typed.len()..self.cursor, "");
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }
    fn redo(&mut self) {}
}

impl View for CodeArea {
    fn draw(&self, printer: &Printer) {}
    fn on_event(&mut self, event: Event) -> EventResult {
        let mut consumed = true;
        match event {
            // Input
            Event::Char(ch) => self.insert(ch),
            Event::Key(Key::Tab) => self.insert('\t'),
            Event::Key(Key::Enter) => self.insert('\n'),
            // Erase
            Event::Ctrl(Key::Backspace) => self.erase_line(),
            Event::Key(Key::Backspace) => self.erase_symbol(),
            // Movement
            Event::Key(Key::Home) | Event::Ctrl(Key::Left) => unimplemented!(),
            Event::Key(Key::End) | Event::Ctrl(Key::Right) => unimplemented!(),
            Event::Ctrl(Key::Up) | Event::Ctrl(Key::Home) => unimplemented!(),
            Event::Ctrl(Key::Down) | Event::Ctrl(Key::End) => unimplemented!(),
            Event::Key(Key::Left) => unimplemented!(),
            Event::Key(Key::Right) => unimplemented!(),
            Event::Key(Key::Up) => unimplemented!(),
            Event::Key(Key::Down) => unimplemented!(),
            // Edit
            Event::CtrlChar('z') => self.undo(),
            Event::CtrlChar('y') => self.redo(),
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
