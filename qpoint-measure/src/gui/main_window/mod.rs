use iced::widget::{button, column, radio, row, rule, space, text};
use iced::{Element, Length, Task};
use native_dialog::MessageLevel;

use crate::gui;
use crate::measure::PlotData;

mod hybrid_parameters;
mod input_characteristics;
mod output_characteristics;

#[derive(Clone, Debug)]
pub struct MainWindow {
    transistor: Option<TransistorType>,
    measurement: Option<MeasurementType>,
    output_config: output_characteristics::Config,
    input_config: input_characteristics::Config,
    hybrid_config: hybrid_parameters::Config,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TransistorType {
    NPN,
    PNP,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MeasurementType {
    OutputCharacteristics,
    InputCharacteristics,
    HybridParameters,
}

#[allow(private_interfaces)]
#[derive(Clone, Debug)]
pub enum Message {
    TransistorSelected(TransistorType),
    MeasurementSelected(MeasurementType),
    LoadDefaults,
    StartMeasurement,
    DisplayAlert(MessageLevel, String),
    OpenPlot(PlotData),
    OutputConfig(output_characteristics::Message),
    InputConfig(input_characteristics::Message),
    HybridConfig(hybrid_parameters::Message),
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            transistor: None,
            measurement: None,
            output_config: output_characteristics::Config::new(),
            input_config: input_characteristics::Config::new(),
            hybrid_config: hybrid_parameters::Config::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TransistorSelected(transistor_type) => {
                self.transistor = Some(transistor_type);
                Task::none()
            }
            Message::MeasurementSelected(measurement_type) => {
                self.measurement = Some(measurement_type);
                Task::none()
            }
            Message::LoadDefaults => {
                self.output_config = output_characteristics::Config::default();
                self.input_config = input_characteristics::Config::default();
                self.hybrid_config = hybrid_parameters::Config::default();
                Task::none()
            }
            Message::StartMeasurement => {
                if self.transistor.is_none() {
                    Task::done(Message::DisplayAlert(
                        MessageLevel::Warning,
                        "Select a transistor type!".into(),
                    ))
                } else if self.measurement.is_none() {
                    Task::done(Message::DisplayAlert(
                        MessageLevel::Warning,
                        "Select a measurement!".into(),
                    ))
                } else {
                    match self
                        .prepare_measurement(self.measurement.unwrap(), self.transistor.unwrap())
                    {
                        Ok(task) => task,
                        Err(e) => Task::done(Message::DisplayAlert(MessageLevel::Error, e)),
                    }
                }
            }
            Message::DisplayAlert(level, message) => {
                gui::alert(level, message);
                Task::none()
            }
            // NOTE: handled outside via a special case
            Message::OpenPlot(_) => unreachable!(),
            Message::OutputConfig(m) => {
                self.output_config.update(m);
                Task::none()
            }
            Message::InputConfig(m) => {
                self.input_config.update(m);
                Task::none()
            }
            Message::HybridConfig(m) => {
                self.hybrid_config.update(m);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = text("QPoint Measure")
            .size(28)
            .width(Length::Fill)
            .height(Length::Shrink)
            .center();

        let transistor_selection = row![
            radio(
                "NPN",
                TransistorType::NPN,
                self.transistor,
                Message::TransistorSelected
            ),
            radio(
                "PNP",
                TransistorType::PNP,
                self.transistor,
                Message::TransistorSelected
            ),
        ]
        .width(Length::Fill)
        .height(Length::Shrink)
        .spacing(8)
        .padding([4, 0]);

        let output_selector = radio(
            "Output Characteristics",
            MeasurementType::OutputCharacteristics,
            self.measurement,
            Message::MeasurementSelected,
        );

        let input_selector = radio(
            "Input Characteristics",
            MeasurementType::InputCharacteristics,
            self.measurement,
            Message::MeasurementSelected,
        );

        let hybrid_selector = radio(
            "Hybrid Parameters (Common Emitter)",
            MeasurementType::HybridParameters,
            self.measurement,
            Message::MeasurementSelected,
        );

        let control_buttons = row![
            button(text("Load defaults").width(Length::Fill).center())
                .width(Length::FillPortion(1))
                .on_press(Message::LoadDefaults),
            button(text("Start measurement").width(Length::Fill).center())
                .width(Length::FillPortion(1))
                .style(button::success)
                .on_press(Message::StartMeasurement),
        ]
        .width(Length::Fill)
        .spacing(8)
        .padding(4);

        column![
            title,
            space().height(4),
            rule::horizontal(1),
            rule::horizontal(1),
            text("Transistor Type").style(text::primary),
            transistor_selection,
            rule::horizontal(1),
            output_selector,
            self.output_config.view().map(Message::OutputConfig),
            rule::horizontal(1),
            input_selector,
            self.input_config.view().map(Message::InputConfig),
            rule::horizontal(1),
            hybrid_selector,
            self.hybrid_config.view().map(Message::HybridConfig),
            rule::horizontal(1),
            rule::horizontal(1),
            control_buttons,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(4)
        .padding(4)
        .into()
    }

    fn prepare_measurement(
        &self,
        measurement: MeasurementType,
        transistor: TransistorType,
    ) -> Result<Task<Message>, String> {
        match measurement {
            MeasurementType::OutputCharacteristics => {
                let params = self.output_config.parse()?;
                eprintln!("Starting measurement ({:?}): {:#?}", transistor, params);

                let task = match transistor {
                    TransistorType::NPN => Task::perform(params.measure_npn(), |r| match r {
                        Ok(plot) => Message::OpenPlot(plot),
                        Err(e) => Message::DisplayAlert(
                            MessageLevel::Error,
                            format!("Failed to measure: {}", e),
                        ),
                    }),
                    TransistorType::PNP => Task::perform(params.measure_pnp(), |r| match r {
                        Ok(plot) => Message::OpenPlot(plot),
                        Err(e) => Message::DisplayAlert(
                            MessageLevel::Error,
                            format!("Failed to measure: {}", e),
                        ),
                    }),
                };

                Ok(task)
            }
            MeasurementType::InputCharacteristics => {
                let params = self.input_config.parse()?;
                println!("{:#?}", params);
                Ok(Task::none())
            }
            MeasurementType::HybridParameters => {
                let params = self.hybrid_config.parse()?;
                println!("{:#?}", params);
                Ok(Task::none())
            }
        }
    }
}
