use crate::fluids::{self, AvailableFluids};
use crate::thermo::{ThermoState, fluid_state};
use iced::Length::Fill;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Scrollable, Space, container, grid, pick_list, text, text_input};
use iced::{Color, Element, Font, Length};
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct Ui {
    current_thermo_state: Option<ThermoState>,

    selected_fluid: AvailableFluids,

    input_pressure_text: String,
    input_pressure_value: Option<f64>,
    input_pressure_valid: bool,

    input_temperature_text: String,
    input_temperature_value: Option<f64>,
    input_temperature_valid: bool,
}

#[derive(Clone)]
pub enum Message {
    FluidSelected(AvailableFluids),
    InputPressureChanged(String),
    InputTemperatureChanged(String),
}

impl Ui {
    pub fn new() -> Self {
        Self {
            current_thermo_state: None,
            selected_fluid: AvailableFluids::Air,
            input_pressure_text: String::new(),
            input_pressure_value: Some(100_000.0),
            input_pressure_valid: true,
            input_temperature_text: String::new(),
            input_temperature_value: Some(273.15),
            input_temperature_valid: true,
        }
    }

    fn calculate_thermo_state(&mut self) {
        if self.input_pressure_valid && self.input_temperature_valid {
            let state = fluids::create_pure_state(
                &self.selected_fluid,
                self.input_pressure_value.unwrap(),
                self.input_temperature_value.unwrap(),
            );
            match state {
                Ok(mut s) => {
                    let new_state = fluid_state(&mut s);
                    self.current_thermo_state = Some(new_state);
                }
                Err(_) => self.current_thermo_state = None,
            }
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::FluidSelected(fluid) => self.selected_fluid = fluid,
            Message::InputPressureChanged(string) => {
                let input_result = string.parse::<f64>();
                match input_result {
                    Ok(value) => {
                        self.input_pressure_text = string;
                        self.input_pressure_value = Some(value);
                        self.input_pressure_valid = true;
                        self.calculate_thermo_state();
                    }
                    Err(_) => {
                        self.input_pressure_text = string;
                        self.input_pressure_value = None;
                        self.input_pressure_valid = false;
                        self.current_thermo_state = None;
                    }
                }
            }
            Message::InputTemperatureChanged(string) => {
                let input_result = string.parse::<f64>();
                match input_result {
                    Ok(value) => {
                        self.input_temperature_text = string;
                        self.input_temperature_value = Some(value);
                        self.input_temperature_valid = true;
                        self.calculate_thermo_state();
                    }
                    Err(_) => {
                        self.input_temperature_text = string;
                        self.input_temperature_value = None;
                        self.input_temperature_valid = false;
                        self.current_thermo_state = None;
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message, iced::Theme> {
        let mut layout_grid = grid::Grid::new().columns(3).spacing(5);

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
            Some(self.selected_fluid),
            Message::FluidSelected,
        );

        // Row 1
        layout_grid = layout_grid.push(
            container(
                text("Fluid")
                    .size(16)
                    .color(Color::WHITE)
                    .font(Font::DEFAULT),
            )
            .center_y(Length::Fill),
        );
        layout_grid = layout_grid.push(
            container(fluid_dropdown)
                .padding(5)
                .max_width(100.0)
                .center_y(Fill),
        );
        layout_grid = layout_grid.push(container(Space::new()).center_y(Length::Fill));

        // Row 2
        layout_grid = layout_grid.push(
            container(
                text("Pressure")
                    .size(16)
                    .color(Color::WHITE)
                    .font(Font::DEFAULT),
            )
            .center_y(Length::Fill),
        );
        layout_grid = layout_grid.push(
            container(
                text_input("Pressure", &self.input_pressure_text)
                    .on_input(Message::InputPressureChanged),
            )
            .max_width(100.0)
            .center_y(Length::Fill),
        );
        layout_grid = layout_grid.push(container(Space::new()).center_y(Length::Fill));

        // Row 3
        layout_grid = layout_grid.push(
            container(
                text("Temperature")
                    .size(16)
                    .color(Color::WHITE)
                    .font(Font::DEFAULT),
            )
            .center_y(Length::Fill),
        );
        layout_grid = layout_grid.push(
            container(
                text_input("Temperature", &self.input_temperature_text)
                    .on_input(Message::InputTemperatureChanged),
            )
            .max_width(100.0)
            .center_y(Length::Fill),
        );
        layout_grid = layout_grid.push(container(Space::new()).center_y(Length::Fill));

        // Row 4
        layout_grid = layout_grid.push(
            container(
                text("Acentric Factor")
                    .size(16)
                    .color(Color::WHITE)
                    .font(Font::DEFAULT),
            )
            .center_y(Length::Fill),
        );

        let t = match self.current_thermo_state.clone() {
            Some(state) => state,
            None => ThermoState::default(),
        };
        let a = t.acentric_factor;
        match a {
            Some(value) => {
                layout_grid = layout_grid.push(
                    container(text(value).size(16).color(Color::WHITE).font(Font::DEFAULT))
                        .center_y(Length::Fill),
                );
            }
            None => {
                layout_grid = layout_grid.push(
                    container(text("").size(16).color(Color::WHITE).font(Font::DEFAULT))
                        .center_y(Length::Fill),
                );
            }
        }
        layout_grid = layout_grid.push(
            container(text("[]").size(16).color(Color::WHITE).font(Font::DEFAULT))
                .center_y(Length::Fill),
        );

        layout_grid = layout_grid.push(container(Space::new()).center_y(Length::Fill));
        let grid_container = container(layout_grid);
        let scrollable_grid =
            Scrollable::with_direction(grid_container, Direction::Vertical(Scrollbar::default()));
        container(scrollable_grid).padding(20.0).into()
    }
}
