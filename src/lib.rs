use cursive::event::{Event, EventResult, Key};
use cursive::theme::{Color, ColorStyle, Effect};
use cursive::vec::Vec2;
use cursive::view::{ScrollBase, View};
use cursive::Printer;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

const TAB_LEN: usize = 4;

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

struct Content {
    before: String,
    after: String,
    selected_column: usize,
    selected_line: usize,
}

impl Content {
    fn new() -> Self {
        Self {
            before: String::new(),
            after: String::new(),
            selected_column: 0,
            selected_line: 0,
        }
    }
    fn column(&self) -> usize {
        self.before
            .chars()
            .rev()
            .take_while(|&ch| ch != '\n')
            .count()
    }
}

/// Multi-lines code editor.
///
/// CodeArea shows line numbers
/// and can highligh your code using
/// your syntax
pub struct CodeArea {
    syntax: Syntax,

    content: Content,

    enabled: bool,

    scrollbase_ver: ScrollBase,

    scrollbase_hor: ScrollBase,

    last_size: Vec2,
}

// Public interface
impl CodeArea {
    pub fn new() -> Self {
        CodeArea {
            syntax: Syntax::new(),
            content: Content::new(),
            enabled: true,
            scrollbase_ver: ScrollBase::new().right_padding(0),
            scrollbase_hor: ScrollBase::new().right_padding(0),
            last_size: Vec2::zero(),
        }
    }
    pub fn with_text(_text: String) -> Self {
        // TODO
        Self::new()
    }
    pub fn use_syntax(mut self, syntax: Syntax) -> Self {
        self.syntax = syntax;
        self
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn get_content(&self) -> String {
        let mut content = self.content.before.clone();
        content.push_str(&self.content.after.chars().rev().collect::<String>());
        content
    }
    pub fn get_cursor_pos(&self) -> (usize, usize) {
        (self.content.selected_column, self.content.selected_line)
    }
}

// Text manage
// interface
impl CodeArea {
    pub fn insert(&mut self, ch: char) {
        self.content.before.push(ch);
        self.content.selected_column += 1;
    }
    pub fn tab(&mut self) {
        self.content.before.push('\t');
        self.content.selected_column += TAB_LEN;
    }
    pub fn new_line(&mut self) {
        self.content.selected_column = 0;
        self.content.selected_line += 1;
        self.content.before.push('\n');
    }
    pub fn erase(&mut self) {
        self.erase_symbol();
    }
    pub fn erase_line(&mut self) {
        while let Some(ch) = self.erase_symbol() {
            if ch == '\n' {
                break;
            }
        }
    }
}

// Text manage
// Auxiliary functional
impl CodeArea {
    pub fn erase_symbol(&mut self) -> Option<char> {
        let ch = self.content.before.pop();
        match ch {
            Some('\n') => {
                self.content.selected_line -= 1;
                self.content.selected_column = self
                    .content
                    .before
                    .chars()
                    .rev()
                    .take_while(|&ch| ch != '\n')
                    .count();
            }
            Some('\t') => self.content.selected_column -= TAB_LEN,
            Some(_) => self.content.selected_column -= 1,
            None => {}
        };
        ch
    }
}

// Cursor manage
// Intefrace
impl CodeArea {
    pub fn right(&mut self) {
        self.move_right();
    }
    pub fn end(&mut self) {
        self.right_to_end();
    }
    pub fn left(&mut self) {
        self.move_left();
    }
    pub fn home(&mut self) {
        self.left_to_home();
    }
    pub fn beginning_of_file(&mut self) {
        while let Some(_) = self.move_left() {}
    }
    pub fn end_of_file(&mut self) {
        while let Some(_) = self.move_right() {}
    }
    pub fn up(&mut self) {
        let column = self.left_to_home();
        self.left();
        let upper_line_width = self.content.column();
        if upper_line_width > column {
            for _ in 0..(upper_line_width - column) {
                self.move_left();
            }
        }
    }
    pub fn down(&mut self) {
        let mut column = self.content.column();
        self.right_to_end();
        self.right();
        while let Some(ch) = self.move_right() {
            column -= 1;
            if ch == '\n' || column == 0 {
                break;
            }
        }
    }
}

// Cursor manage
// Auxiliary functional
impl CodeArea {
    pub fn move_right(&mut self) -> Option<char> {
        if let Some(ch) = self.content.after.pop() {
            self.content.before.push(ch);
            match ch {
                '\n' => {
                    self.content.selected_column = 0;
                    self.content.selected_line += 1;
                }
                '\t' => self.content.selected_column += TAB_LEN,
                _ => self.content.selected_column += 1,
            }
            Some(ch)
        } else {
            None
        }
    }
    pub fn move_left(&mut self) -> Option<char> {
        if let Some(ch) = self.content.before.pop() {
            self.content.after.push(ch);
            match ch {
                '\n' => {
                    self.content.selected_line -= 1;
                    self.content.selected_column = self.content.column();
                }
                '\t' => self.content.selected_column -= TAB_LEN,
                _ => self.content.selected_column -= 1,
            }
            Some(ch)
        } else {
            None
        }
    }
    pub fn right_to_end(&mut self) -> usize {
        let mut counter = 0;
        loop {
            match self.content.after.pop() {
                Some('\n') => {
                    self.content.after.push('\n');
                    break;
                }
                Some('\t') => {
                    self.content.selected_column += TAB_LEN;
                    self.content.before.push('\t');
                }
                Some(ch) => {
                    self.content.selected_column += 1;
                    self.content.before.push(ch);
                }
                None => break,
            }
            counter += 1;
        }
        counter
    }
    pub fn left_to_home(&mut self) -> usize {
        let mut counter = 0;
        loop {
            match self.content.before.pop() {
                Some('\n') => {
                    self.content.before.push('\n');
                    break;
                }
                Some('\t') => {
                    self.content.selected_column -= TAB_LEN;
                    self.content.after.push('\t');
                }
                Some(ch) => {
                    self.content.selected_column -= 1;
                    self.content.after.push(ch);
                }
                None => break,
            }
            counter += 1;
        }
        counter
    }
}

impl View for CodeArea {
    fn draw(&self, printer: &Printer) {
        printer.with_color(ColorStyle::secondary(), |printer| {
            let effect = Effect::Reverse;

            let h = if self.scrollbase_hor.scrollable() {
                printer.size.y - 1
            } else {
                printer.size.y
            };

            let line_number_len = {
                let mut number = std::cmp::max(0, h);
                let mut len = 0;
                while number > 0 {
                    len += 1;
                    number /= 10;
                }
                len
            };

            let w = if self.scrollbase_ver.scrollable() {
                printer.size.x - 1 - line_number_len
            } else {
                printer.size.x - line_number_len
            };

            // Background and line numbers
            for y in 0..h {
                let line_number = self.scrollbase_ver.start_line + y + 1;
                let line_number_str = format_line_number(line_number_len, line_number);
                printer.print((1, y), &line_number_str);
                printer.with_effect(effect, |printer| {
                    printer.print_hline((line_number_len + 2, y), w, " ");
                });
            }
            /*self.scrollbase_ver.draw(printer, |printer, i| {
                if self.rows.count() >= i {
                    return;
                }
                let row = self.rows.row(i);
                let mut text = &self.content[row.grapheme_start..row.grapheme_end];
                if w < text.len() {
                    text = &self.content[row.grapheme_start..w];
                }
                printer.with_effect(effect, |printer| {
                    printer.print((0, 0), text);
                });
            });*/
        });
    }
    fn on_event(&mut self, event: Event) -> EventResult {
        if !self.enabled {
            return EventResult::Ignored;
        }
        let mut consumed = true;
        match event {
            // Input
            Event::Char(ch) => self.insert(ch),
            Event::Key(Key::Tab) => self.tab(),
            Event::Key(Key::Enter) => self.new_line(),
            // Erase
            Event::Ctrl(Key::Backspace) => self.erase_line(),
            Event::Key(Key::Backspace) => self.erase(),
            // Movement
            Event::Key(Key::Home) | Event::Ctrl(Key::Left) => self.home(),
            Event::Key(Key::End) | Event::Ctrl(Key::Right) => self.end(),
            Event::Ctrl(Key::Up) | Event::Ctrl(Key::Home) => self.beginning_of_file(),
            Event::Ctrl(Key::Down) | Event::Ctrl(Key::End) => self.end_of_file(),
            Event::Key(Key::Left) => self.left(),
            Event::Key(Key::Right) => self.right(),
            Event::Key(Key::Up) => self.up(),
            Event::Key(Key::Down) => self.down(),
            // Stop event handling
            Event::Key(Key::Esc) => self.disable(),
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

fn format_line_number(len: usize, number: usize) -> String {
    let mut number_str = format!("{}|", number);
    for _ in 0..len - (number_str.len() - 1) {
        number_str.insert(0, ' ');
    }
    number_str
}

mod test {
    use super::*;
    #[test]
    fn insert_text() {
        let mut area = CodeArea::new();
        area.insert('a');
        assert_eq!(&area.get_content(), "a");
        area.insert('b');
        assert_eq!(&area.get_content(), "ab");
        area.new_line();
        assert_eq!(&area.get_content(), "ab\n");
        area.tab();
        assert_eq!(&area.get_content(), "ab\n\t");
    }
    #[test]
    fn insert_cursor() {
        let mut area = CodeArea::new();
        assert_eq!(area.get_cursor_pos(), (0, 0));
        area.insert('a');
        assert_eq!(area.get_cursor_pos(), (1, 0));
        area.insert('b');
        assert_eq!(area.get_cursor_pos(), (2, 0));
        area.new_line();
        assert_eq!(area.get_cursor_pos(), (0, 1));
        area.insert('c');
        assert_eq!(area.get_cursor_pos(), (1, 1));
        area.tab();
        assert_eq!(area.get_cursor_pos(), (1 + TAB_LEN, 1));
    }
    #[test]
    fn erase_symbol_text() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.tab();
        area.new_line();
        area.erase_symbol();
        assert_eq!(&area.get_content(), "ab\t");
        area.erase_symbol();
        assert_eq!(&area.get_content(), "ab");
        area.erase_symbol();
        assert_eq!(&area.get_content(), "a");
    }
    #[test]
    fn erase_symbol_cursor() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.new_line();
        area.erase_symbol();
        assert_eq!(area.get_cursor_pos(), (2, 0));
        area.erase_symbol();
        assert_eq!(area.get_cursor_pos(), (1, 0));
    }
    #[test]
    fn erase_line_text() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.new_line();
        area.erase_line();
        assert_eq!(&area.get_content(), "ab");
        area.erase_line();
        assert_eq!(&area.get_content(), "");
    }
    #[test]
    fn erase_line_cursor() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.new_line();
        area.erase_line();
        assert_eq!(area.get_cursor_pos(), (2, 0));
        area.erase_line();
        assert_eq!(area.get_cursor_pos(), (0, 0));
    }
    #[test]
    fn left_text() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.new_line();
        area.left();
        area.insert('c');
        assert_eq!(&area.get_content(), "abc\n");
        area.left();
        area.insert('d');
        assert_eq!(&area.get_content(), "abdc\n");
    }

    #[test]
    fn left_cursor() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.new_line();
        area.left();
        assert_eq!(area.get_cursor_pos(), (2, 0));
        area.left();
        assert_eq!(area.get_cursor_pos(), (1, 0));
    }
    #[test]
    fn right_text() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.new_line();
        area.left();
        area.left();
        area.right();
        area.insert('c');
        assert_eq!(&area.get_content(), "abc\n");
        area.right();
        area.insert('d');
        assert_eq!(&area.get_content(), "abc\nd");
    }
    #[test]
    fn right_cursor() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.new_line();
        area.left();
        area.left();
        area.right();
        assert_eq!(area.get_cursor_pos(), (2, 0));
        area.right();
        assert_eq!(area.get_cursor_pos(), (0, 1));
    }
    #[test]
    fn up_text() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.insert('c');
        area.new_line();
        area.insert('d');
        area.insert('e');

        area.up();
        area.insert('f');
        assert_eq!(&area.get_content(), "abfc\nde");

        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.insert('c');
        area.insert('d');
        area.insert('e');

        area.up();
        area.insert('f');
        assert_eq!(&area.get_content(), "fabcde");

        let mut area = CodeArea::new();
        area.up();
        area.insert('f');
        assert_eq!(&area.get_content(), "f");
    }
    #[test]
    fn up_cursor() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.insert('c');
        area.new_line();
        area.insert('d');
        area.insert('e');

        area.up();
        assert_eq!(area.get_cursor_pos(), (2,0));

        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.insert('c');
        area.insert('d');
        area.insert('e');

        area.up();
        assert_eq!(area.get_cursor_pos(), (0,0));

        let mut area = CodeArea::new();
        area.up();
        assert_eq!(area.get_cursor_pos(), (0,0));
    }
    #[test]
    fn down_text() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.insert('c');
        area.new_line();
        area.insert('d');
        area.insert('e');

        area.up();
        area.down();
        area.insert('f');
        assert_eq!(&area.get_content(), "abc\ndef");

        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.insert('c');
        area.insert('d');
        area.insert('e');

        area.up();
        area.down();
        area.insert('f');
        assert_eq!(&area.get_content(), "abcdef");

        let mut area = CodeArea::new();
        area.down();
        area.insert('f');
        assert_eq!(&area.get_content(), "f");
    }
    #[test]
    fn down_cursor() {
        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.insert('c');
        area.new_line();
        area.insert('d');
        area.insert('e');

        area.up();
        area.down();
        assert_eq!(area.get_cursor_pos(), (2,1));

        let mut area = CodeArea::new();
        area.insert('a');
        area.insert('b');
        area.insert('c');
        area.insert('d');
        area.insert('e');

        area.down();
        assert_eq!(area.get_cursor_pos(), (5,0));

        let mut area = CodeArea::new();
        area.down();
        assert_eq!(area.get_cursor_pos(), (0,0));
    }
}
