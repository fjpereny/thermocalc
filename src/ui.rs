use crate::fluids::{self, AvailableFluids};
use crate::thermo::{ThermoState, ThermoStateView, fluid_state};
use iced::Length::Fill;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Scrollable, Space, container, grid, pick_list, text, text_input};
use iced::{Color, Element, Font, Length};
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

        for (label, value) in fields {
            layout_grid = layout_grid
                .push(text(*label))
                .push(text(*value))
                .push(container(Space::new()).center_y(Length::Fill));
        }

        let grid_container = container(layout_grid);
        let scrollable_grid =
            Scrollable::with_direction(grid_container, Direction::Vertical(Scrollbar::default()));
        container(scrollable_grid).padding(20.0).into()
    }
}
