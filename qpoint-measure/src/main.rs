use std::collections::HashMap;

use iced::widget::operation;
use iced::window;
use iced::window::Settings;
use iced::{Element, Function, Size, Subscription, Task};

mod com;
mod gui;
mod measure;

use crate::gui::main_window::{self, MainWindow};
use crate::gui::plot_window::{self, PlotWindow};

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title("QPoint Measure")
        .run()
}

#[derive(Debug)]
struct App {
    window_id: window::Id,
    window_state: MainWindow,

    plot_windows: HashMap<window::Id, PlotWindow>,
    result_windows: HashMap<window::Id, Box<u32>>,
}

#[derive(Debug, Clone)]
enum Message {
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    MainWindow(main_window::Message),
    PlotWindow(window::Id, plot_window::Message),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let (id, open) = window::open(Settings {
            size: Size::new(632.0, 859.0),
            ..Settings::default()
        });

        (
            Self {
                window_id: id,
                window_state: MainWindow::new(),
                plot_windows: HashMap::new(),
                result_windows: HashMap::new(),
            },
            open.map(Message::WindowOpened),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened(id) => operation::focus(format!("input-{}", id)),
            Message::WindowClosed(id) => {
                if id == self.window_id {
                    iced::exit()
                } else {
                    self.plot_windows.remove(&id);
                    self.result_windows.remove(&id);
                    Task::none()
                }
            }
            Message::MainWindow(main_window::Message::OpenPlot(plot_data)) => {
                let (id, open) = window::open(Settings {
                    size: Size::new(1288.0, 900.0),
                    ..Settings::default()
                });

                self.plot_windows.insert(id, PlotWindow::new(plot_data));

                open.map(Message::WindowOpened)
            }
            Message::MainWindow(main_window::Message::OpenResults(hybrid_matrix)) => {
                todo!()
            }
            Message::MainWindow(m) => self.window_state.update(m).map(Message::MainWindow),
            Message::PlotWindow(id, m) => {
                if let Some(window) = self.plot_windows.get_mut(&id) {
                    window.update(m).map(Message::PlotWindow.with(id))
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if window_id == self.window_id {
            self.window_state.view().map(Message::MainWindow)
        } else if let Some(window) = self.plot_windows.get(&window_id) {
            window.view().map(Message::PlotWindow.with(window_id))
        } else if let Some(_window) = self.result_windows.get(&window_id) {
            todo!()
        } else {
            iced::widget::space().into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }
}
