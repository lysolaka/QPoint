mod gui;

fn main() -> iced::Result {
    iced::application(gui::State::default, gui::State::update, gui::State::view)
        .window_size((632, 859))
        .title("QPoint Measure")
        .run()
}
