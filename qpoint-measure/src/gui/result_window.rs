use iced::widget::{button, column, row, text, text_input};
use iced::{Alignment, Element, Length, Task};
use native_dialog::MessageLevel;

use crate::gui;
use crate::measure::HybridMatrix;

#[derive(Clone, Debug)]
pub struct ResultWindow {
    results: HybridMatrix,
    hie: String,
    hre: String,
    hfe: String,
    hoe: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    DisplayAlert(MessageLevel, String),
    SaveCSV,
    TextInputNoop,
}

impl ResultWindow {
    pub fn new(results: HybridMatrix) -> Self {
        let hie = results
            .hie
            .map_or_else(|| String::from("Failed!"), |v| format!("{:+04.6}", v));
        let hre = format!("{:+04.6}", results.hre);
        let hfe = format!("{:+04.6}", results.hfe);
        let hoe = results
            .hoe
            .map_or_else(|| String::from("Failed!"), |v| format!("{:+04.6}", v));

        Self {
            results,
            hie,
            hre,
            hfe,
            hoe,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DisplayAlert(level, message) => {
                match level {
                    MessageLevel::Info => log::info!("{}", message),
                    MessageLevel::Warning => log::warn!("{}", message),
                    MessageLevel::Error => log::error!("{}", message),
                }
                gui::alert(level, message);
                Task::none()
            }
            Message::SaveCSV => match self.results.write_csv() {
                Ok(data) => Task::future(gui::save_file(data, "CSV", [".csv"])).then(ok_or_alert),
                Err(e) => Task::done(Message::DisplayAlert(
                    MessageLevel::Error,
                    format!("Failed to prepare data: {}", e),
                )),
            },
            Message::TextInputNoop => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let hie = column![
            text("Input Impedance [k\u{03a9}]").style(text::primary),
            text_input("", &self.hie).on_input(|_| Message::TextInputNoop)
        ]
        .spacing(8)
        .align_x(Alignment::Start);

        let hre = column![
            text("Reverse Voltage Transfer").style(text::primary),
            text_input("", &self.hre).on_input(|_| Message::TextInputNoop)
        ]
        .spacing(8)
        .align_x(Alignment::Start);

        let hfe = column![
            text("Forward Current Gain").style(text::primary),
            text_input("", &self.hfe).on_input(|_| Message::TextInputNoop)
        ]
        .spacing(8)
        .align_x(Alignment::Start);

        let hoe = column![
            text("Output Admittance [mS]").style(text::primary),
            text_input("", &self.hoe).on_input(|_| Message::TextInputNoop)
        ]
        .spacing(8)
        .align_x(Alignment::Start);

        column![
            text("Hybrid Parameters").size(28),
            row![hie, hre].spacing(8),
            row![hfe, hoe].spacing(8),
            button(text("Save CSV").width(Length::Fill).center())
                .width(Length::Fill)
                .on_press(Message::SaveCSV),
        ]
        .padding(8)
        .spacing(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }
}

fn ok_or_alert(r: Result<(), String>) -> Task<Message> {
    match r {
        Ok(_) => Task::none(),
        Err(e) => Task::done(Message::DisplayAlert(MessageLevel::Error, e)),
    }
}
