use iced::widget::{button, column, row, text, text_input};
use iced::{Alignment, Element, Length, Task};

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
    SaveCSV,
    TextInputNoop,
}

impl ResultWindow {
    pub fn new(results: HybridMatrix) -> Self {
        let hie = results
            .hie
            .map_or_else(|| String::from("Failed!"), |v| format!("{:+09}", v));
        let hre = format!("{:+09}", results.hre);
        let hfe = format!("{:+09}", results.hfe);
        let hoe = results
            .hoe
            .map_or_else(|| String::from("Failed!"), |v| format!("{:+09}", v));

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
            Message::SaveCSV => {
                eprintln!("{:#?}", self.results);
                Task::none()
            }
            Message::TextInputNoop => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let hie = column![
            text("Input Impedance").style(text::primary),
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
            text("Output Admittance").style(text::primary),
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
