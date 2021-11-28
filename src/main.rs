use fltk::{app, button::Button, frame::Frame, group::Flex, input::Input, prelude::*, window::Window};

fn creategui() -> app::App {
    let app = app::App::default();
    let mut wind = Window::default().with_size(640, 200).with_label("Searcher");
    // Vertical is default. You can choose horizontal using pack.set_type(PackType::Horizontal);
    let mut flex = Flex::default().size_of_parent().column();

    let mut flex_r1 = Flex::default().size_of_parent().row();
    let mut but_path = Button::default().with_size(10, 40).with_label("Path");
    flex_r1.set_size(&mut but_path, 60);
    let mut input_path = Input::default().with_size(300, 40);
    flex_r1.end();

    let mut flex_r2 = Flex::default().size_of_parent().row();
    let mut but_search = Button::default().with_size(10, 40).with_label("Search");
    flex_r2.set_size(&mut but_search, 60);
    let mut input_search = Input::default().with_size(300, 40);
    flex_r2.end();
    flex.end();
    wind.end();
    wind.show();
    app
}
fn main() {
    let mut app = creategui();
    app.run().unwrap();
}
