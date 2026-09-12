use iced::Element;
use iced::widget::{button, checkbox, column, pick_list, radio, row, space, text, text_input};
use iced::{Alignment, Length, Size};

use native_dialog::{DialogBuilder, MessageLevel};

use qpoint_common::Command;
use qpoint_common::measurement::{BaseSource, CollectorSource, EmitterSource};

mod serial;

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .window(iced::window::Settings {
            size: Size::new(512.0, 490.0),
            ..Default::default()
        })
        .title("QPoint Executor")
        .run()
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("{0}")]
    SerialPort(#[from] serialport::Error),
    #[error("{0}")]
    Serialization(#[from] postcard::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("{0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("The device is not found")]
    NotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandFlat {
    Attach,
    Detach,
    LedSet,
    BaseSelect,
    BaseSet,
    CollectorSelect,
    CollectorSet,
    EmitterSelect,
}

impl CommandFlat {
    fn into_command(&self, values: &State) -> Result<Command, crate::Error> {
        let cmd = match self {
            CommandFlat::Attach => Command::Attach,
            CommandFlat::Detach => Command::Detach,
            CommandFlat::LedSet => Command::LedSet(
                u8::from_str_radix(&values.led_r, 16)?,
                u8::from_str_radix(&values.led_g, 16)?,
                u8::from_str_radix(&values.led_b, 16)?,
            ),
            CommandFlat::BaseSelect => Command::BaseSelect(values.base_source.expect("never None")),
            CommandFlat::BaseSet => {
                let value = values.base_value.parse()?;
                Command::BaseSet {
                    value,
                    measure: values.base_measure,
                }
            }
            CommandFlat::CollectorSelect => {
                Command::CollectorSelect(values.collector_source.expect("never None"))
            }
            CommandFlat::CollectorSet => {
                let value = values.collector_value.parse()?;
                Command::CollectorSet {
                    value,
                    measure: values.collector_measure,
                }
            }
            CommandFlat::EmitterSelect => {
                Command::EmitterSelect(values.emitter_source.expect("never None"))
            }
        };

        Ok(cmd)
    }
}

#[derive(Debug, Clone)]
enum Message {
    ExecutePressed,
    CommandSelected(CommandFlat),
    LedRChanged(String),
    LedGChanged(String),
    LedBChanged(String),
    BaseSourceSelected(BaseSource),
    BaseValueChanged(String),
    BaseMeasureToggled(bool),
    CollectorSourceSelected(CollectorSource),
    CollectorValueChanged(String),
    CollectorMeasureToggled(bool),
    EmitterSourceSelected(EmitterSource),
}

struct State {
    led_r: String,
    led_g: String,
    led_b: String,
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
                if let Err(e) = self
                    .selected_command
                    .into_command(&self)
                    .inspect(|c| eprintln!("Executing: {:?}", c))
                    .map(|c| serial::execute(c))
                    .flatten()
                    .inspect(|r| eprintln!("Response: {:?}", r))
                {
                    DialogBuilder::message()
                        .set_level(MessageLevel::Error)
                        .set_title("QPoint Executor")
                        .set_text(format!("Error: {}", e))
                        .alert()
                        .show()
                        .unwrap();
                }
            }
            Message::CommandSelected(command) => self.selected_command = command,
            Message::LedRChanged(r) => self.led_r = r,
            Message::LedGChanged(g) => self.led_g = g,
            Message::LedBChanged(b) => self.led_b = b,
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

        let radio_led_set = radio(
            "LED set",
            CommandFlat::LedSet,
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

        let led_set = row![
            radio_led_set,
            space().width(63),
            text("R:"),
            text_input("R", &self.led_r)
                .on_input(Message::LedRChanged)
                .width(Length::Fixed(48.0)),
            text("G:"),
            text_input("G", &self.led_g)
                .on_input(Message::LedGChanged)
                .width(Length::Fixed(48.0)),
            text("B:"),
            text_input("B", &self.led_b)
                .on_input(Message::LedBChanged)
                .width(Length::Fixed(48.0))
        ]
        .spacing(12)
        .align_y(Alignment::Center);

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
            space().height(3),
            led_set,
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
            led_r: "0".to_string(),
            led_g: "0".to_string(),
            led_b: "0".to_string(),
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
