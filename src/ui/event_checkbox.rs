use iced::{Alignment, Element};
use iced::widget::{checkbox, row, text};
use crate::io::html::event::Event;

#[derive(Debug, Clone)]
pub enum EventToInclude {
    Edited(bool),
}

#[derive(Debug, Clone)]
pub struct EventCheckbox {
    pub(crate) index: usize,
    pub(crate) event: Event,
}

impl EventCheckbox {
    pub fn new(index: usize, event: Event) -> Self {
        EventCheckbox {
            index,
            event,
        }
    }

    pub fn view(&self) -> Element<EventToInclude> {
        let checkbox = checkbox("", self.event.active).on_toggle(EventToInclude::Edited);

        row![checkbox, text(&self.event.event_name).align_x(Alignment::Center).height(30)].align_y(Alignment::Center).into()
    }
}