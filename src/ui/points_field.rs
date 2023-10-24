/*Copyright (c) 2023 Collin Ogren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

/*
use iced::overlay::Element;
use iced::widget::{row, text, text_input};
use iced::alignment::Vertical;
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct PointsField {
    pub(crate) index: usize,
    pub(crate) value: String,
}

impl PointsField {
    pub fn new(index: usize, value: String) -> Self {
        PointsField {
            index,
            value,
        }
    }

    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("{i}"))
    }

    pub fn view(&self, index: usize) -> Element<PointsForEachPlacement> {
        let points_field = text_input(
            format!("Points for position {}", index + 1).as_str(),
            &self.value,
        ).id(Self::text_input_id(index)).on_input(PointsForEachPlacement::Edited);

       row![text(if index < 9 {format!("  {}: ", index + 1)} else {format!("{}: ", index + 1)}).vertical_alignment(Vertical::Center).height(30), points_field].into()
    }
}
*/