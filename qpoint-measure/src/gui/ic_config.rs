use std::mem;

use iced::widget::{button, column, pick_list, row, space, text, text_input};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    ConditionInputChanged(String),
    AddCondition,
    ConditionSelected(String),
    RemoveCondition,
    StartChanged(String),
    EndChanged(String),
    StepChanged(String),
}

#[derive(Default, Debug, Clone)]
pub struct State {
    conditions: Vec<String>,
    selected_condition: Option<String>,
    condition_input: String,
    start: String,
    end: String,
    step: String,
}

impl State {
    pub fn update(&mut self, m: Message) {
        match m {
            Message::ConditionInputChanged(v) => self.condition_input = v,
            Message::AddCondition => {
                if !self.conditions.contains(&self.condition_input) {
                    let cond = mem::replace(&mut self.condition_input, String::new());
                    self.conditions.push(cond);
                }
            }
            Message::ConditionSelected(v) => self.selected_condition = Some(v),
            Message::RemoveCondition => {
                if let Some(cond) = self.selected_condition.as_ref() {
                    if let Some((idx, _)) =
                        self.conditions.iter().enumerate().find(|&(_, c)| c == cond)
                    {
                        self.conditions.remove(idx);
                        self.selected_condition = None;
                    }
                }
            }
            Message::StartChanged(v) => self.start = v,
            Message::EndChanged(v) => self.end = v,
            Message::StepChanged(v) => self.step = v,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let condition_input = text_input("", &self.condition_input)
            .width(100)
            .on_input(Message::ConditionInputChanged)
            .on_submit(Message::AddCondition);

        let condition_selection = pick_list(
            self.conditions.as_slice(),
            self.selected_condition.as_ref(),
            Message::ConditionSelected,
        )
        .width(100);

        let condition_box = column![
            text("Base current conditions").style(text::primary),
            row![
                button(text("+").width(Length::Fill).height(Length::Fill).center())
                    .style(button::success)
                    .width(31)
                    .on_press(Message::AddCondition),
                condition_input,
                text("uA").height(Length::Fill).align_y(Alignment::Center),
            ]
            .spacing(4),
            row![
                button(text("-").width(Length::Fill).height(Length::Fill).center())
                    .style(button::danger)
                    .width(31)
                    .on_press(Message::RemoveCondition),
                condition_selection,
                text("uA").height(Length::Fill).align_y(Alignment::Center),
            ]
            .spacing(4),
        ]
        .width(Length::Shrink)
        .height(Length::Shrink)
        .spacing(4)
        .padding(4);

        let range_box = column![
            text("Collector-Emitter voltage range").style(text::primary),
            row![
                text("Start:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(8),
                text_input("", &self.start)
                    .on_input(Message::StartChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ],
            row![
                text("End:").height(Length::Fill).align_y(Alignment::Center),
                space().width(15),
                text_input("", &self.end)
                    .on_input(Message::EndChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ],
            row![
                text("Step:")
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                space().width(10),
                text_input("", &self.step)
                    .on_input(Message::StepChanged)
                    .width(64),
                space().width(4),
                text("V").height(Length::Fill).align_y(Alignment::Center)
            ]
        ]
        .width(Length::Shrink)
        .height(Length::Shrink)
        .spacing(4)
        .padding(4);

        row![condition_box, range_box]
            .width(Length::Fill)
            .height(Length::Shrink)
            .spacing(36)
            .padding(4)
            .into()
    }
}
