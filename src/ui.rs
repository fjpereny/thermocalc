use crate::fluids::AvailableFluids;
use iced::Element;
use iced::widget::{container, grid, pick_list};
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct Ui {
    selected_fluid: Option<AvailableFluids>,
}

impl Ui {}

#[derive(Clone, Copy)]
pub enum Message {
    FluidSelected(AvailableFluids),
}

impl Ui {
    pub fn new() -> Self {
        Self {
            selected_fluid: Option::Some(AvailableFluids::Air),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            _ => (),
        }
    }

    pub fn view(&self) -> Element<'_, Message, iced::Theme> {
        let mut layout_grid = grid::Grid::new().columns(3).spacing(15);

        let available_fluids: Vec<AvailableFluids> = AvailableFluids::iter().collect();
        let fluid_dropdown: iced::widget::PickList<
            '_,
            AvailableFluids,
            _,
            _,
            Message,
            iced::Theme,
        > = pick_list(
            available_fluids,
            self.selected_fluid,
            Message::FluidSelected,
        );
        layout_grid = layout_grid.push(container(fluid_dropdown).padding(5));

        container(layout_grid).padding(20.0).into()
    }
}
