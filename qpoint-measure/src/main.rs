mod gui;

fn main() -> iced::Result {
    iced::application(gui::State::default, gui::State::update, gui::State::view)
        .title("QPoint Measure")
        .run()
}
