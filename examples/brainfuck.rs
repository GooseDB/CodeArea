use code_area::{CodeArea, Syntax};
use cursive::theme::Color;
use cursive::views::{BoxView, PaddedView};
use cursive::view::Margins;
use cursive::Cursive;

fn main() {
    let mut ui = Cursive::default();

    ui.add_fullscreen_layer(PaddedView::new(
        Margins::from((1, 1, 1, 1)),BoxView::with_full_screen(
        CodeArea::new().use_syntax(
            Syntax::new()
                .add_one_color_symbols(&['+', '-'], Color::from_256colors(0))
                .add_one_color_symbols(&['[', ']'], Color::from_256colors(1))
                .add_one_color_symbols(&['>', '<'], Color::from_256colors(2))
                .add_one_color_symbols(&['.', ','], Color::from_256colors(3))
        )),
    ));
    
    ui.run();
}
