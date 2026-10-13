use iced::widget::{button, column, container, radio, row, rule, text};
use iced::{Alignment, Element, Length};

mod hparam_config;
mod input_config;
mod output_config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Measurement {
    /// Ic = f(Uce) @ Ib
    OutputCharacteristics,
    /// Ib = f(Ube) @ Uce
    InputCharacteristics,
    /// Self explanatory
    HybridParameters,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TransistorType {
    #[default]
    NPN,
    PNP,
}

#[derive(Debug, Clone)]
pub enum Message {
    TransistorTypeChanged(TransistorType),
    MeasurementSelected(Measurement),
    LoadDefaultsPressed,
    StartPressed,
    OutputConfig(output_config::Message),
    InputConfig(input_config::Message),
    HParamConfig(hparam_config::Message),
}

#[derive(Default, Debug, Clone)]
pub struct State {
    /// Transistor type selection
    type_selection: TransistorType,
    /// Measurement selection
    measurement_selection: Option<Measurement>,
    /// [`Measurement::OutputCharacteristics`] config
    output_config: output_config::State,
    /// [`Measurement::InputCharacteristics`] config
    input_config: input_config::State,
    /// [`Measurement::HParameters`] config
    hparam_config: hparam_config::State,
}

impl State {
    pub fn update(&mut self, m: Message) {
        match m {
            Message::TransistorTypeChanged(v) => self.type_selection = v,
            Message::MeasurementSelected(v) => self.measurement_selection = Some(v),
            Message::LoadDefaultsPressed => {
                self.output_config.load_defaults();
                self.input_config.load_defaults();
                self.hparam_config.load_defaults();
            }
            Message::StartPressed => {
                if self.measurement_selection.is_none() {
                    // TODO: add a dialog box for this
                    println!("Select a measurement first");
                } else {
                    self.start_measurement(self.measurement_selection.unwrap());
                }
            }
            Message::OutputConfig(m) => self.output_config.update(m),
            Message::InputConfig(m) => self.input_config.update(m),
            Message::HParamConfig(m) => self.hparam_config.update(m),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = container(text("QPoint Measure").size(28))
            .padding(4)
            .center_x(Length::Fill);

        let type_selection = row![
            radio(
                "NPN",
                TransistorType::NPN,
                Some(self.type_selection),
                Message::TransistorTypeChanged
            ),
            radio(
                "PNP",
                TransistorType::PNP,
                Some(self.type_selection),
                Message::TransistorTypeChanged
            ),
        ]
        .width(Length::Fill)
        .height(Length::Shrink)
        .spacing(8)
        .padding([4, 0]);

        let output_selector = radio(
            "Output Characteristics",
            Measurement::OutputCharacteristics,
            self.measurement_selection,
            Message::MeasurementSelected,
        );

        let input_selector = radio(
            "Input Characteristics",
            Measurement::InputCharacteristics,
            self.measurement_selection,
            Message::MeasurementSelected,
        );

        let hparam_selector = radio(
            "Hybrid Parameters (Common Emitter)",
            Measurement::HybridParameters,
            self.measurement_selection,
            Message::MeasurementSelected,
        );

        let start_button = container(
            button(
                text("Start measurement")
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .style(button::success)
            .on_press(Message::StartPressed),
        )
        .padding(4)
        .center_x(Length::Fill);

        let defaults_button = container(
            button(
                text("Load defaults")
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .on_press(Message::LoadDefaultsPressed),
        )
        .padding(4)
        .center_x(Length::Fill);

        column![
            title,
            rule::horizontal(1),
            rule::horizontal(1),
            text("Transistor Type").style(text::primary),
            type_selection,
            rule::horizontal(1),
            output_selector,
            self.output_config.view().map(Message::OutputConfig),
            rule::horizontal(1),
            input_selector,
            self.input_config.view().map(Message::InputConfig),
            rule::horizontal(1),
            hparam_selector,
            self.hparam_config.view().map(Message::HParamConfig),
            rule::horizontal(1),
            rule::horizontal(1),
            row![defaults_button, start_button].width(Length::Fill)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(4)
        .padding(4)
        .into()
    }

    fn start_measurement(&self, measurement: Measurement) -> Result<(), String> {
        match measurement {
            Measurement::OutputCharacteristics => todo!(),
            Measurement::InputCharacteristics => todo!(),
            Measurement::HybridParameters => todo!(),
        }
    }
}
