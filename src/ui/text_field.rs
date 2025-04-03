use iced::alignment::Vertical;
use iced::{Element, Length};
use iced::widget::{row, text, text_input};

#[derive(Debug, Clone)]
pub enum PointsForEachPlacement {
    Edited(String),
}

pub trait TextField<T, E> {
    fn new(id: T, value: String, requested_width: Option<Length>) -> Self;
    fn text_input_id(id: T) -> text_input::Id;

    fn view(&self, id: T) -> Element<E>;
}

#[derive(Debug, Clone)]
pub struct PointsField {
    pub(crate) index: usize,
    pub(crate) value: String,
    pub(crate) requested_width: Option<Length>,
}

impl TextField<usize, PointsForEachPlacement> for PointsField {
    fn new(index: usize, value: String, requested_width: Option<Length>) -> Self {
        PointsField {
            index,
            value,
            requested_width
        }
    }

    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("{i}"))
    }

    fn view(&self, index: usize) -> Element<PointsForEachPlacement> {
        let mut points_field = text_input(
            format!("Points for position {}", index + 1).as_str(),
            &self.value,
        ).id(Self::text_input_id(index)).on_input(PointsForEachPlacement::Edited);

        points_field = match &self.requested_width {
            Some(width) => {
                points_field = points_field.width(*width);
                points_field
            }
            None => {
                points_field
            }
        };

        row![text(if index < 9 {format!("  {}: ", index + 1)} else {format!("{}: ", index + 1)}).align_y(Vertical::Center).height(30), points_field].into()
    }
}

#[derive(Debug, Clone)]
pub struct ClubPointsField {
    pub(crate) index: usize,
    pub(crate) value: String,
    pub(crate) requested_width: Option<Length>,
}

#[derive(Debug, Clone)]
pub enum ClubPointsEdit {
    Edited(String),
    Submitted,
}

impl TextField<usize, ClubPointsEdit> for ClubPointsField {
    fn new(index: usize, value: String, requested_width: Option<Length>) -> Self {
        ClubPointsField {
            index,
            value,
            requested_width
        }
    }

    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("{}", i))
    }

    fn view(&self, index: usize) -> Element<ClubPointsEdit> {
        let mut points_field = text_input(
            "",
            &self.value,
        ).id(Self::text_input_id(index)).on_input(ClubPointsEdit::Edited).on_submit(ClubPointsEdit::Submitted);

        points_field = match &self.requested_width {
            Some(width) => {
                points_field = points_field.width(*width);
                points_field
            }
            None => {
                points_field
            }
        };

        points_field.into()
    }
}