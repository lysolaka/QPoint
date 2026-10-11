use iced::widget::{column, row, space, text, text_input};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    HieStartChanged(String),
    HieEndChanged(String),
    HieStepChanged(String),
    HreCurrentChanged(String),
    HreVoltageChanged(String),
    HfeCurrent1Changed(String),
    HfeCurrent2Changed(String),
    HfeVoltageChanged(String),
    HoeCurrentChanged(String),
    HoeStartChanged(String),
    HoeEndChanged(String),
    HoeStepChanged(String),
}

#[derive(Default, Debug, Clone)]
pub struct State {
    hie_start: String,
    hie_end: String,
    hie_step: String,
    hre_current: String,
    hre_voltage: String,
    hfe_current1: String,
    hfe_current2: String,
    hfe_voltage: String,
    hoe_current: String,
    hoe_start: String,
    hoe_end: String,
    hoe_step: String,
}

impl State {
    pub fn update(&mut self, m: Message) {
        match m {
            Message::HieStartChanged(v) => self.hie_start = v,
            Message::HieEndChanged(v) => self.hie_end = v,
            Message::HieStepChanged(v) => self.hie_step = v,
            Message::HreCurrentChanged(v) => self.hre_current = v,
            Message::HreVoltageChanged(v) => self.hre_voltage = v,
            Message::HfeCurrent1Changed(v) => self.hfe_current1 = v,
            Message::HfeCurrent2Changed(v) => self.hfe_current2 = v,
            Message::HfeVoltageChanged(v) => self.hfe_voltage = v,
            Message::HoeCurrentChanged(v) => self.hoe_current = v,
            Message::HoeStartChanged(v) => self.hoe_start = v,
            Message::HoeEndChanged(v) => self.hoe_end = v,
            Message::HoeStepChanged(v) => self.hoe_step = v,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let input_impedance = column![
            text("Input Impedance").style(text::primary),
            // text("Base-Emitter voltage"),
            row![
                text("UBE Start:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(8),
                text_input("", &self.hie_start)
                    .on_input(Message::HieStartChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ],
            row![
                text("UBE End:").height(Length::Fill).align_y(Alignment::Center),
                space().width(15),
                text_input("", &self.hie_end)
                    .on_input(Message::HieEndChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ],
            row![
                text("UBE Step:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(10),
                text_input("", &self.hie_step)
                    .on_input(Message::HieStepChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ]
        ]
        .width(Length::Shrink)
        .height(Length::Shrink)
        .spacing(4)
        .padding(4);

        let reverse_transfer = column![
            text("Reverse Voltage Transfer").style(text::primary),
            row![
                text("Base current:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(100),
                text_input("", &self.hre_current)
                    .on_input(Message::HreCurrentChanged)
                    .width(64),
                space().width(4),
                text("uA").height(Length::Fill).align_y(Alignment::Center)
            ],
            row![
                text("Collector-Emitter voltage:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(8),
                text_input("", &self.hre_voltage)
                    .on_input(Message::HreVoltageChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ],
        ]
        .width(Length::Shrink)
        .height(Length::Shrink)
        .spacing(4)
        .padding(4);

        let forward_current = column![
            text("Forward Current Gain").style(text::primary),
            row![
                text("Base current 1:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(86),
                text_input("", &self.hfe_current1)
                    .on_input(Message::HfeCurrent1Changed)
                    .width(64),
                space().width(4),
                text("uA").height(Length::Fill).align_y(Alignment::Center)
            ],
            row![
                text("Base current 2:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(86),
                text_input("", &self.hfe_current2)
                    .on_input(Message::HfeCurrent2Changed)
                    .width(64),
                space().width(4),
                text("uA").height(Length::Fill).align_y(Alignment::Center)
            ],
            row![
                text("Collector-Emitter voltage:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(8),
                text_input("", &self.hfe_voltage)
                    .on_input(Message::HfeVoltageChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ],
        ]
        .width(Length::Shrink)
        .height(Length::Shrink)
        .spacing(4)
        .padding(4);

        let output_admittance = column![
            text("Output Admittance").style(text::primary),
            row![
                text("Base current:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(8),
                text_input("", &self.hoe_current)
                    .on_input(Message::HoeCurrentChanged)
                    .width(64),
                space().width(4),
                text("uA").height(Length::Fill).align_y(Alignment::Center)
            ],
            // text("Collector-Emitter voltage"),
            row![
                text("UCE Start:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(33),
                text_input("", &self.hoe_start)
                    .on_input(Message::HoeStartChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ],
            row![
                text("UCE End:").height(Length::Fill).align_y(Alignment::Center),
                space().width(40),
                text_input("", &self.hoe_end)
                    .on_input(Message::HoeEndChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ],
            row![
                text("UCE Step:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(35),
                text_input("", &self.hoe_step)
                    .on_input(Message::HoeStepChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ]
        ]
        .width(Length::Shrink)
        .height(Length::Shrink)
        .spacing(4)
        .padding(4);

        row![
            column![input_impedance, forward_current].spacing(24),
            column![reverse_transfer, output_admittance].spacing(24),
        ]
        .width(Length::Fill)
        .height(Length::Shrink)
        .spacing(24)
        .padding(4)
        .into()
    }
}
