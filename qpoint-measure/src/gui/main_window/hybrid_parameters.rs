use iced::widget::{column, row, space, text, text_input};
use iced::{Alignment, Element, Length};

use crate::measure::HybridParameters;

#[derive(Clone, Debug)]
pub struct Config {
    u_ce: String,
    i_b: String,
    delta_u_ce: String,
    delta_i_b: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    UceChanged(String),
    IbChanged(String),
    DeltaUceChanged(String),
    DeltaIbChanged(String),
}

impl Config {
    pub fn new() -> Self {
        Self {
            u_ce: String::new(),
            i_b: String::new(),
            delta_u_ce: String::new(),
            delta_i_b: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UceChanged(v) => self.u_ce = v,
            Message::IbChanged(v) => self.i_b = v,
            Message::DeltaUceChanged(v) => self.delta_u_ce = v,
            Message::DeltaIbChanged(v) => self.delta_i_b = v,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let operating_point = column![
            text("DC Operating Point").style(text::primary),
            row![
                text("Base current:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(100),
                text_input("", &self.i_b)
                    .on_input(Message::IbChanged)
                    .width(64),
                space().width(4),
                text("uA").height(Length::Fill).align_y(Alignment::Center)
            ],
            row![
                text("Collector-Emitter Voltage:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(8),
                text_input("", &self.u_ce)
                    .on_input(Message::UceChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ],
        ]
        .width(Length::Shrink)
        .height(Length::Shrink)
        .spacing(4)
        .padding(4);

        let small_signal = column![
            text("Small-signal Amplitudes").style(text::primary),
            row![
                text("Base current:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(100),
                text_input("", &self.delta_i_b)
                    .on_input(Message::DeltaIbChanged)
                    .width(64),
                space().width(4),
                text("uA").height(Length::Fill).align_y(Alignment::Center)
            ],
            row![
                text("Collector-Emitter voltage:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(8),
                text_input("", &self.delta_u_ce)
                    .on_input(Message::DeltaUceChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ],
        ]
        .width(Length::Shrink)
        .height(Length::Shrink)
        .spacing(4)
        .padding(4);

        row![operating_point, small_signal]
            .width(Length::Fill)
            .height(Length::Shrink)
            .spacing(24)
            .padding(4)
            .into()
    }

    pub fn parse(&self) -> Result<HybridParameters, String> {
        HybridParameters::new(&self.u_ce, &self.i_b, &self.delta_u_ce, &self.delta_i_b)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            u_ce: String::from("2.5"),
            i_b: String::from("100.0"),
            delta_u_ce: String::from("1.0"),
            delta_i_b: String::from("25.0"),
        }
    }
}
