use fltk::{app, button::Button, group::Flex, input::Input, prelude::*, window::Window};

fn creategui() -> app::App {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default().with_size(640, 200).with_label("Searcher");
    // Vertical is default. You can choose horizontal using pack.set_type(PackType::Horizontal);
    let mut main_col = Flex::default_fill().column();
    main_col.set_pad(10);
    let mut first_row = Flex::default().size_of_parent().row();
    let mut but_path = Button::default().with_size(0, 40).with_label("Path");
    let mut input_path = Input::default().with_size(300, 40);
    first_row.end();
    let mut second_row = Flex::default().size_of_parent().row();
    let mut but_search = Button::default().with_size(0, 40).with_label("Search");
    let mut input_search = Input::default().with_size(300, 40);
    second_row.end();
    main_col.end();

    wind.end();
    wind.show();
    app
}
fn main() {
    let mut app = creategui();
    app.run().unwrap();
}
