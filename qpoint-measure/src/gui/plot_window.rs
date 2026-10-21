use iced::Element;
use iced::widget::space;

use crate::measure::PlotData;

#[derive(Clone, Debug)]
pub struct PlotWindow {
    data: PlotData,
}

#[derive(Clone, Debug)]
pub enum Message {}

impl PlotWindow {
    pub fn new(plot_data: PlotData) -> Self {
        Self { data: plot_data }
    }

    pub fn update(&mut self, message: Message) {
        match message {}
    }

    pub fn view(&self) -> Element<'_, Message> {
        space().into()
    }
}
