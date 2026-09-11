use iced::Element;
use iced::widget::{button, checkbox, column, pick_list, radio, row, space, text, text_input};
use iced::{Alignment, Length, Size};

use qpoint_common::Command;
use qpoint_common::measurement::{BaseSource, CollectorSource, EmitterSource};

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .window(iced::window::Settings {
            size: Size::new(512.0, 480.0),
            ..Default::default()
        })
        .title("QPoint Executor")
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandFlat {
    Attach,
    Detach,
    BaseSelect,
    BaseSet,
    CollectorSelect,
    CollectorSet,
    EmitterSelect,
}

impl CommandFlat {
    fn into_command(&self, values: &State) -> Command {
        match self {
            CommandFlat::Attach => Command::Attach,
            CommandFlat::Detach => Command::Detach,
            CommandFlat::BaseSelect => Command::BaseSelect(values.base_source.expect("never None")),
            CommandFlat::BaseSet => {
                let value = values.base_value.parse().unwrap();
                Command::BaseSet {
                    value,
                    measure: values.base_measure,
                }
            }
            CommandFlat::CollectorSelect => {
                Command::CollectorSelect(values.collector_source.expect("never None"))
            }
            CommandFlat::CollectorSet => {
                let value = values.collector_value.parse().unwrap();
                Command::CollectorSet {
                    value,
                    measure: values.collector_measure,
                }
            }
            CommandFlat::EmitterSelect => {
                Command::EmitterSelect(values.emitter_source.expect("never None"))
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ExecutePressed,
    CommandSelected(CommandFlat),
    BaseSourceSelected(BaseSource),
    BaseValueChanged(String),
    BaseMeasureToggled(bool),
    CollectorSourceSelected(CollectorSource),
    CollectorValueChanged(String),
    CollectorMeasureToggled(bool),
    EmitterSourceSelected(EmitterSource),
}

struct State {
    base_source: Option<BaseSource>,
    base_value: String,
    base_measure: bool,
    collector_source: Option<CollectorSource>,
    collector_value: String,
    collector_measure: bool,
    emitter_source: Option<EmitterSource>,

    selected_command: CommandFlat,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::ExecutePressed => {
                let cmd = self.selected_command.into_command(&self);
                eprintln!("Execute: {:#?}", cmd);
            }
            Message::CommandSelected(command) => self.selected_command = command,
            Message::BaseSourceSelected(base_source) => self.base_source = Some(base_source),
            Message::BaseValueChanged(v) => self.base_value = v,
            Message::BaseMeasureToggled(base_measure) => self.base_measure = base_measure,
            Message::CollectorSourceSelected(collector_source) => {
                self.collector_source = Some(collector_source)
            }
            Message::CollectorValueChanged(v) => self.collector_value = v,
            Message::CollectorMeasureToggled(collector_measure) => {
                self.collector_measure = collector_measure
            }
            Message::EmitterSourceSelected(emitter_source) => {
                self.emitter_source = Some(emitter_source)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let radio_attach = radio(
            "Attach",
            CommandFlat::Attach,
            Some(self.selected_command),
            Message::CommandSelected,
        );

        let radio_detach = radio(
            "Detach",
            CommandFlat::Detach,
            Some(self.selected_command),
            Message::CommandSelected,
        );

        let radio_base_select = radio(
            "Base select",
            CommandFlat::BaseSelect,
            Some(self.selected_command),
            Message::CommandSelected,
        );

        let radio_base_set = radio(
            "Base set",
            CommandFlat::BaseSet,
            Some(self.selected_command),
            Message::CommandSelected,
        );

        let radio_collector_select = radio(
            "Collector select",
            CommandFlat::CollectorSelect,
            Some(self.selected_command),
            Message::CommandSelected,
        );

        let radio_collector_set = radio(
            "Collector set",
            CommandFlat::CollectorSet,
            Some(self.selected_command),
            Message::CommandSelected,
        );

        let radio_emitter_select = radio(
            "Emitter select",
            CommandFlat::EmitterSelect,
            Some(self.selected_command),
            Message::CommandSelected,
        );

        let base_select = row![
            radio_base_select,
            space().width(36),
            pick_list(
                [
                    BaseSource::HighZ,
                    BaseSource::ISource,
                    BaseSource::ISink,
                    BaseSource::VSource,
                ],
                self.base_source,
                Message::BaseSourceSelected,
            )
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let base_set = row![
            radio_base_set,
            space().width(58),
            text("Value:"),
            text_input("value", &self.base_value)
                .on_input(Message::BaseValueChanged)
                .width(Length::Fixed(140.0)),
            checkbox(self.base_measure)
                .label("Measure")
                .on_toggle(Message::BaseMeasureToggled),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let collector_select = row![
            radio_collector_select,
            space().width(6),
            pick_list(
                [
                    CollectorSource::HighZ,
                    CollectorSource::VCC,
                    CollectorSource::VSource,
                    CollectorSource::GND,
                ],
                self.collector_source,
                Message::CollectorSourceSelected,
            )
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let collector_set = row![
            radio_collector_set,
            space().width(27),
            text("Value:"),
            text_input("value", &self.collector_value)
                .on_input(Message::CollectorValueChanged)
                .width(Length::Fixed(140.0)),
            checkbox(self.collector_measure)
                .label("Measure")
                .on_toggle(Message::CollectorMeasureToggled),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let emitter_select = row![
            radio_emitter_select,
            space().width(17),
            pick_list(
                [EmitterSource::HighZ, EmitterSource::VCC, EmitterSource::GND],
                self.emitter_source,
                Message::EmitterSourceSelected,
            )
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let execute_button = button(
            text("Execute")
                .align_x(Alignment::Center)
                .width(Length::Fill),
        )
        .on_press(Message::ExecutePressed)
        .width(Length::Fill)
        .padding([8, 24]);

        column![
            text("QPoint Command Executor")
                .size(24)
                .align_x(Alignment::Center)
                .width(Length::Fill),
            space().height(12),
            radio_attach,
            space().height(3),
            radio_detach,
            space().height(8),
            base_select,
            base_set,
            space().height(8),
            collector_select,
            collector_set,
            space().height(8),
            emitter_select,
            space().height(16),
            execute_button,
        ]
        .width(Length::Fill)
        .padding(20)
        .spacing(6)
        .align_x(Alignment::Start)
        .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            base_source: Some(BaseSource::HighZ),
            base_value: "0".to_string(),
            base_measure: false,
            collector_source: Some(CollectorSource::HighZ),
            collector_value: "0".to_string(),
            collector_measure: false,
            emitter_source: Some(EmitterSource::HighZ),
            selected_command: CommandFlat::Attach,
        }
    }
}
