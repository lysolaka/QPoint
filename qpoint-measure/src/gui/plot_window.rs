use iced::advanced::svg::Handle;
use iced::widget::{button, column, row, space, svg, text};
use iced::{Element, Length, Task};
use native_dialog::MessageLevel;

use crate::gui;
use crate::measure::PlotData;

#[derive(Clone, Debug)]
pub struct PlotWindow {
    data: PlotData,
    image: Handle,
}

#[derive(Clone, Debug)]
pub enum Message {
    DisplayAlert(MessageLevel, String),
    SaveCSV,
    SaveSVG,
}

impl PlotWindow {
    pub fn new(plot_data: PlotData) -> Self {
        let image = Handle::from_memory(plot_data.render_svg().into_bytes());
        Self {
            data: plot_data,
            image,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DisplayAlert(level, message) => {
                gui::alert(level, message);
                Task::none()
            }
            Message::SaveCSV => match self.data.write_csv() {
                Ok(data) => Task::future(gui::save_file(data, "CSV", [".csv"])).then(ok_or_alert),
                Err(e) => Task::done(Message::DisplayAlert(
                    MessageLevel::Error,
                    format!("Failed to prepare data: {}", e),
                )),
            },
            Message::SaveSVG => {
                let image = self.data.render_svg();
                Task::future(gui::save_file(image, "SVG", [".svg"])).then(ok_or_alert)
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let image = svg(self.image.clone()).width(Length::Fill);

        let buttons = row![
            space().width(Length::FillPortion(1)),
            button(text("Save CSV").width(Length::Fill).center())
                .width(Length::FillPortion(1))
                .on_press(Message::SaveCSV),
            button(text("Save SVG").width(Length::Fill).center())
                .width(Length::FillPortion(1))
                .on_press(Message::SaveSVG),
            space().width(Length::FillPortion(1)),
        ]
        .width(Length::Fill)
        .spacing(8)
        .padding(4);

        column![image, space().height(Length::Fill), buttons]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(4)
            .padding(4)
            .into()
    }
}

fn ok_or_alert(r: Result<(), String>) -> Task<Message> {
    match r {
        Ok(_) => Task::none(),
        Err(e) => Task::done(Message::DisplayAlert(MessageLevel::Error, e)),
    }
}
