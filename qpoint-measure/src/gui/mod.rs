use iced::widget::{button, column, container, radio, row, rule, text, toggler};
use iced::{Alignment, Element, Length};

mod hparam_config;
mod ib_config;
mod ic_config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Measurement {
    /// Ic = f(Uce) @ Ib
    CollectorCurrent,
    /// Ib = f(Ube) @ Uce
    BaseCurrent,
    /// Self explanatory
    HParameters,
}

#[derive(Debug, Clone)]
pub enum Message {
    TransistorTypeChanged(bool),
    MeasurementSelected(Measurement),
    StartPressed,
    IcConfig(ic_config::Message),
    IbConfig(ib_config::Message),
}

#[derive(Default, Debug, Clone)]
pub struct State {
    /// Transistor type selection: false - NPN, true - PNP
    type_selection: bool,
    /// Measurement selection
    measurement_selection: Option<Measurement>,
    /// [`Measurement::CollectorCurrent`] config
    ic_config: ic_config::State,
    /// [`Measurement::BaseCurrent`] config
    ib_config: ib_config::State,
}

impl State {
    pub fn update(&mut self, m: Message) {
        match m {
            Message::TransistorTypeChanged(v) => self.type_selection = v,
            Message::MeasurementSelected(v) => self.measurement_selection = Some(v),
            Message::StartPressed => {
                if self.measurement_selection.is_none() {
                    // TODO: add a dialog box for this
                    println!("Select a measurement first");
                } else {
                    // TODO: implement measurements
                    println!("Starting measurement: {:#?}", &self);
                }
            }
            Message::IcConfig(m) => self.ic_config.update(m),
            Message::IbConfig(m) => self.ib_config.update(m),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = container(text("QPoint Measure").size(28))
            .padding(4)
            .center_x(Length::Fill);

        let type_selection = row![
            text("NPN").height(Length::Fill).align_y(Alignment::Center),
            toggler(self.type_selection).on_toggle(Message::TransistorTypeChanged),
            text("PNP").height(Length::Fill).align_y(Alignment::Center)
        ]
        .width(Length::Fill)
        .height(Length::Shrink)
        .spacing(4)
        .padding(8);

        let ic_selector = radio(
            "Collector Current Graph",
            Measurement::CollectorCurrent,
            self.measurement_selection,
            Message::MeasurementSelected,
        );

        let ib_selector = radio(
            "Base Current Graph",
            Measurement::BaseCurrent,
            self.measurement_selection,
            Message::MeasurementSelected,
        );

        let hparam_selector = radio(
            "H Parameters",
            Measurement::HParameters,
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

        column![
            title,
            rule::horizontal(1),
            rule::horizontal(1),
            text("Transistor Type").style(text::primary),
            type_selection,
            rule::horizontal(1),
            ic_selector,
            self.ic_config.view().map(Message::IcConfig),
            rule::horizontal(1),
            ib_selector,
            self.ib_config.view().map(Message::IbConfig),
            rule::horizontal(1),
            hparam_selector,
            rule::horizontal(1),
            rule::horizontal(1),
            start_button,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(4)
        .padding(4)
        .into()
    }
}
