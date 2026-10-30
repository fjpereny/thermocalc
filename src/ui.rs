use crate::fluids::{self, AvailableFluids};
use crate::thermo::{ThermoState, ThermoStateView, fluid_state};
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{
    Scrollable, Space, button, column, container, grid, pick_list, row, text, text_input,
};
use iced::{Element, Font, Length};
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct Ui {
    thermo_state: ThermoState,
    thermo_state_view: ThermoStateView,

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
    MenuClicked(&'static str),
    FluidSelected(AvailableFluids),
    InputPressureChanged(String),
    InputTemperatureChanged(String),
}

impl Ui {
    pub fn new() -> Self {
        Self {
            thermo_state: ThermoState::default(),
            thermo_state_view: ThermoStateView::default(),
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
            if state.is_ok() {
                self.thermo_state = fluid_state(&mut state.unwrap());
                self.thermo_state_view = ThermoStateView::from(&self.thermo_state);
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
                    }
                }
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message, iced::Theme> {
        let menu_items: iced::widget::Row<'_, Message, iced::Theme, iced::Renderer> = row![
            button(text("File").size(14))
                .style(button::text)
                .on_press(Message::MenuClicked("File")),
            button(text("Edit").size(14))
                .style(button::text)
                .on_press(Message::MenuClicked("Edit")),
            button(text("View").size(14))
                .style(button::text)
                .on_press(Message::MenuClicked("View")),
            button(text("Help").size(14))
                .style(button::text)
                .on_press(Message::MenuClicked("Help")),
        ]
        .spacing(15)
        .padding(8);

        let menu_bar = container(menu_items)
            .width(Length::Fill)
            .style(|theme: &iced::Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(palette.background.weak.color.into()),
                    border: iced::Border {
                        color: palette.background.strong.color,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                }
            });

        let mut layout_grid = grid::Grid::new()
            .columns(3)
            .spacing(5)
            .height(Length::Shrink);
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
        layout_grid = layout_grid
            .push(container(text("Fluid").size(16).font(Font::DEFAULT)).height(Length::Shrink));
        layout_grid = layout_grid.push(
            container(fluid_dropdown)
                .padding(5)
                .max_width(100.0)
                .height(Length::Shrink),
        );
        layout_grid = layout_grid.push(container(Space::new()).center_y(Length::Shrink));

        // Row 2
        layout_grid = layout_grid
            .push(container(text("Pressure").size(16).font(Font::DEFAULT)).height(Length::Shrink));
        layout_grid = layout_grid.push(
            container(
                text_input("Pressure", &self.input_pressure_text)
                    .on_input(Message::InputPressureChanged),
            )
            .max_width(100.0)
            .height(Length::Shrink),
        );
        layout_grid = layout_grid.push(container(Space::new()).center_y(Length::Shrink));

        // Row 3
        layout_grid = layout_grid.push(
            container(text("Temperature").size(16).font(Font::DEFAULT)).height(Length::Shrink),
        );
        layout_grid = layout_grid.push(
            container(
                text_input("Temperature", &self.input_temperature_text)
                    .on_input(Message::InputTemperatureChanged),
            )
            .max_width(100.0)
            .height(Length::Shrink),
        );
        layout_grid = layout_grid.push(container(Space::new()).center_y(Length::Shrink));

        let view_data = &self.thermo_state_view;
        let fields: &[(&str, &str)] = &[
            ("Acentric Factor", &view_data.acentric_factor),
            ("Compressibility", &view_data.compressibility),
            ("Conductivity", &view_data.conductivity),
            ("Critical Density", &view_data.critical_density),
            ("Critical Pressure", &view_data.critical_pressure),
            ("Critical Temperature", &view_data.critical_temperature),
            ("Density", &view_data.density),
            ("Dynamic Viscosity", &view_data.dynamic_viscosity),
            ("Enthalpy", &view_data.enthalpy),
            ("Entropy", &view_data.entropy),
            (
                "Gas Dynamics Fund. Deriv.",
                &view_data.fundamental_derivative_of_gas_dynamics,
            ),
            ("Gibbs Energy", &view_data.gibbs_energy),
            ("Helmholtz Energy", &view_data.helmholtz_energy),
            ("Ideal Gas Enthalpy", &view_data.ideal_gas_enthalpy),
            ("Ideal Gas Entropy", &view_data.ideal_gas_entropy),
            (
                "Ideal Gas Internal Energy",
                &view_data.ideal_gas_internal_energy,
            ),
            (
                "Ideal Gas Molar Enthalpy",
                &view_data.ideal_gas_molar_enthalpy,
            ),
            (
                "Ideal Gas Molar Entropy",
                &view_data.ideal_gas_molar_entropy,
            ),
            (
                "Ideal Gas Molar Int. Energy",
                &view_data.ideal_gas_molar_internal_energy,
            ),
            (
                "Ideal Gas Molar Cp Constant",
                &view_data.ideal_gas_molar_specific_heat_constant_pressure,
            ),
            (
                "Ideal Gas Cp Constant",
                &view_data.ideal_gas_specific_heat_constant_pressure,
            ),
            ("Internal Energy", &view_data.internal_energy),
            (
                "Isentropic Expansion Coeff.",
                &view_data.isentropic_expansion_coefficient,
            ),
            (
                "Isothermal Compressibility",
                &view_data.isothermal_compressibility,
            ),
            ("Kinematic Viscosity", &view_data.kinematic_viscosity),
            ("Molar Density", &view_data.molar_density),
            ("Molar Enthalpy", &view_data.molar_enthalpy),
            ("Molar Entropy", &view_data.molar_entropy),
            ("Molar Internal Energy", &view_data.molar_internal_energy),
            ("Molar Quality", &view_data.molar_quality),
            (
                "Molar Cp Constant",
                &view_data.molar_specific_heat_constant_pressure,
            ),
            (
                "Molar Cv Constant",
                &view_data.molar_specific_heat_constant_volume,
            ),
            ("Molar Gibbs Energy", &view_data.molar_gibbs_energy),
            ("Molar Helmholtz Energy", &view_data.molar_helmholtz_energy),
            ("Molar Mass", &view_data.molar_mass),
            (
                "Ozone Depletion Potential",
                &view_data.ozone_depletion_potential,
            ),
            ("Phase", &view_data.phase),
            ("Phase ID Param", &view_data.phase_id_param),
            ("Prandtl Number", &view_data.prandtl_number),
            ("Pressure", &view_data.pressure),
            ("Quality", &view_data.quality),
            (
                "Ratio of Specific Heats",
                &view_data.ratio_of_specific_heats,
            ),
            (
                "Residual Molar Enthalpy",
                &view_data.residual_molar_enthalpy,
            ),
            ("Residual Molar Entropy", &view_data.residual_molar_entropy),
            (
                "Residual Molar Gibbs Energy",
                &view_data.residual_molar_gibbs_energy,
            ),
            (
                "Specific Heat Constant Cp",
                &view_data.specific_heat_constant_pressure,
            ),
            (
                "Specific Heat Constant Cv",
                &view_data.specific_heat_constant_volume,
            ),
            ("Speed of Sound", &view_data.speed_of_sound),
            ("Specific Volume", &view_data.specific_volume),
            ("Surface Tension", &view_data.surface_tension),
            ("Tau", &view_data.tau),
            ("Temperature", &view_data.temperature),
            ("Triple Point Pressure", &view_data.triple_point_pressure),
            (
                "Triple Point Temperature",
                &view_data.triple_point_temperature,
            ),
        ];

        let mut row_alternate_background = true;
        for (label, value) in fields {
            // Define the row's dynamic color matching strategy
            let current_row_is_alt = row_alternate_background;
            let row_style = move |theme: &iced::Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(if current_row_is_alt {
                        palette.background.weak.color.into() // Lighter color
                    } else {
                        palette.background.base.color.into() // Darker color
                    }),
                    ..Default::default()
                }
            };

            layout_grid = layout_grid
                .push(
                    container(text(*label))
                        .style(row_style.clone())
                        .padding(4)
                        .width(Length::Fill),
                )
                .push(
                    container(text(*value))
                        .style(row_style.clone())
                        .padding(4)
                        .width(Length::Fill),
                )
                .push(
                    container(Space::new())
                        .style(row_style)
                        .height(Length::Shrink)
                        .width(Length::Fill),
                );

            row_alternate_background = !row_alternate_background;

            layout_grid = layout_grid.push(text(*label)).push(text(*value)).push(
                container(Space::new())
                    .height(Length::Shrink)
                    .style(|theme: &iced::Theme| {
                        let palette = theme.extended_palette();
                        container::Style {
                            background: Some(palette.background.strongest.color.into()),
                            border: iced::Border {
                                color: palette.background.strong.color,
                                width: 1.0,
                                radius: 0.0.into(),
                            },
                            ..Default::default()
                        }
                    }),
            );
        }

        let grid_container = container(layout_grid);
        let scrollable_grid =
            Scrollable::with_direction(grid_container, Direction::Vertical(Scrollbar::default()));

        let main_content = container(scrollable_grid)
            .padding(20.0)
            .width(Length::Fill)
            .height(Length::Fill);

        // Put the menu_bar and scrollable content together here
        let root_layout: iced::widget::Column<'_, Message, iced::Theme, iced::Renderer> =
            column![menu_bar, main_content,]
                .width(Length::Fill)
                .height(Length::Fill);

        container(root_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
