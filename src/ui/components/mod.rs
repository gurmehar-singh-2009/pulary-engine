use gpui::{Render, div};

pub mod button;
pub mod colors;
pub mod text_area;
pub mod text_input;

pub struct Ui {}

impl Render for Ui {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut gpui::prelude::Context<Self>) -> impl gpui::prelude::IntoElement {
        div()
    }
}

impl Ui {
    pub fn new() -> Self {
        Self {}
    }
}
