#![allow(dead_code)]
// use ::std::fmt;
use rfluids::{
    fluid::{FluidOutputError, OutputResult},
    prelude::*,
};
// use std::fmt::write;

#[derive(Debug)]
pub struct ThermoState {
    acentric_factor: OutputResult<f64>,
    compressibility: OutputResult<f64>,
    conductivity: OutputResult<f64>,
    critical_density: OutputResult<f64>,
    critical_pressure: OutputResult<f64>,
    critical_temperature: OutputResult<f64>,
    density: OutputResult<f64>,
    dynamic_viscosity: OutputResult<f64>,
    enthalpy: OutputResult<f64>,
    entropy: OutputResult<f64>,
    fundamental_derivative_of_gas_dynamics: OutputResult<f64>,
    gibbs_energy: OutputResult<f64>,
    helmholtz_energy: OutputResult<f64>,
    ideal_gas_enthalpy: OutputResult<f64>,
    ideal_gas_entropy: OutputResult<f64>,
    ideal_gas_internal_energy: OutputResult<f64>,
    ideal_gas_molar_enthalpy: OutputResult<f64>,
    ideal_gas_molar_entropy: OutputResult<f64>,
    ideal_gas_molar_internal_energy: OutputResult<f64>,
    ideal_gas_molar_specific_heat_constant_pressure: OutputResult<f64>,
    ideal_gas_specific_heat_constant_pressure: OutputResult<f64>,
    internal_energy: OutputResult<f64>,
    isentropic_expansion_coefficient: OutputResult<f64>,
    isothermal_compressibility: OutputResult<f64>,
    kinematic_viscosity: OutputResult<f64>,
    molar_density: OutputResult<f64>,
    molar_enthalpy: OutputResult<f64>,
    molar_entropy: OutputResult<f64>,
    molar_internal_energy: OutputResult<f64>,
    molar_quality: OutputResult<f64>,
    molar_specific_heat_constant_pressure: OutputResult<f64>,
    molar_specific_heat_constant_volume: OutputResult<f64>,
    molar_gibbs_energy: OutputResult<f64>,
    molar_helmholtz_energy: OutputResult<f64>,
    molar_mass: OutputResult<f64>,
    ozone_depletion_potential: OutputResult<f64>,
    phase: Phase,
    phase_id_param: OutputResult<f64>,
    prandtl_number: OutputResult<f64>,
    pressure: OutputResult<f64>,
    quality: OutputResult<f64>,
    ratio_of_specific_heats: OutputResult<f64>,
    residual_molar_enthalpy: OutputResult<f64>,
    residual_molar_entropy: OutputResult<f64>,
    residual_molar_gibbs_energy: OutputResult<f64>,
    specific_heat_constant_pressure: OutputResult<f64>,
    specific_heat_constant_volume: OutputResult<f64>,
    speed_of_sound: OutputResult<f64>,
    specific_volume: OutputResult<f64>,
    surface_tension: OutputResult<f64>,
    tau: OutputResult<f64>,
    temperature: OutputResult<f64>,
    triple_point_pressure: OutputResult<f64>,
    triple_point_temperature: OutputResult<f64>,
}

// impl fmt::Display for ThermoState {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let acentric_factor = match &self.acentric_factor {
//             Ok(value) => value.to_string(),
//             Err(_) => "ERROR".to_string(),
//         };
//         let compressibility = self.compressibility.clone().unwrap();
//         let conductivity = self.conductivity.clone().unwrap();
//         // Use the write! macro to format exactly how you want it to look
//         write!(
//             f,
//             r#"
//                 Thermodynamic State
// ====================================================
// Property            Value                   Unit
// ====================================================
// Acentric Factor:    {acentric_factor:.3}    []
// Compressibility:    {compressibility:.3}    []
// Conductivity:       {conductivity:.3}       W/m-K
// Critical_density: fluid.critical_density(),
// Critical_pressure: fluid.critical_pressure(),
// Critical_temperature: fluid.critical_temperature(),
// Density: fluid.density(),
// Dynamic_viscosity: fluid.dynamic_viscosity(),
// Ethalpy: fluid.enthalpy(),
// Entropy: fluid.entropy(),
// Fndamental_derivative_of_gas_dynamics: fluid.fundamental_derivative_of_gas_dynamics(),
// Gibbs_energy: fluid.gibbs_energy(),
// Helmholtz_energy: fluid.helmholtz_energy(),
// Ideal_gas_enthalpy: fluid.ideal_gas_enthalpy(),
// Ideal_gas_entropy: fluid.ideal_gas_entropy(),
// Ideal_gas_internal_energy: fluid.ideal_gas_internal_energy(),
// Ideal_gas_molar_enthalpy: fluid.ideal_gas_molar_enthalpy(),
// Ideal_gas_molar_entropy: fluid.ideal_gas_molar_entropy(),
// Ideal_gas_molar_internal_energy: fluid.ideal_gas_molar_internal_energy(),
// Ideal_gas_molar_specific_heat_constant_pressure: fluid.ideal_gas_molar_specific_heat(),
// Ideal_gas_specific_heat_constant_pressure: fluid.ideal_gas_specific_heat(),
// Internal_energy: fluid.internal_energy(),
// Isentropic_expansion_coefficient: fluid.isentropic_expansion_coefficient(),
// Isothermal_compressibility: fluid.isothermal_compressibility(),
// Kinematic_viscosity: fluid.kinematic_viscosity(),
// Molar_density: fluid.molar_density(),
// Molar_enthalpy: fluid.molar_enthalpy(),
// Molar_entropy: fluid.molar_entropy(),
// Molar_gibbs_energy: fluid.molar_gibbs_energy(),
// Molar_helmholtz_energy: fluid.molar_helmholtz_energy(),
// molar_internal_energy: fluid.molar_internal_energy(),
// Molar_quality: fluid.molar_quality(),
// Molar_specific_heat_constant_pressure: fluid.molar_specific_heat(),
// Molar_specific_heat_constant_volume: fluid.molar_specific_heat_const_volume(),
// Molar_mass: fluid.molar_mass(),
// Ozone_depletion_potential: fluid.odp(),
// Phase: fluid.phase(),
// Phase_id_param: fluid.phase_id_param(),
// Prandtl_number: fluid.prandtl(),
// Pressure: fluid.pressure(),
// Quality: fluid.quality(),
// Ratio_of_specific_heats: ratio_specific_heats(
//     fluid.specific_heat(),
//     fluid.specific_heat_const_volume(),
// ),
// Residual_molar_enthalpy: fluid.residual_molar_enthalpy(),
// Residual_molar_entropy: fluid.residual_molar_entropy(),
// Residual_molar_gibbs_energy: fluid.residual_molar_gibbs_energy(),
// Specific_heat_constant_pressure: fluid.specific_heat(),
// Specific_heat_constant_volume: fluid.specific_heat_const_volume(),
// Speed_of_sound: fluid.sound_speed(),
// Specific_volume: fluid.specific_volume(),
// Surface_tension: fluid.surface_tension(),
// Tau: fluid.tau(),
// Temperature: fluid.temperature(),
// Triple_point_pressure: fluid.triple_pressure(),
// Triple_point_temperature: fluid.triple_temperature(),
// "#
//         )
//     }
// }

pub fn fluid_state(fluid: &mut Fluid) -> ThermoState {
    ThermoState {
        acentric_factor: fluid.acentric_factor(),
        compressibility: fluid.compressibility(),
        conductivity: fluid.conductivity(),
        critical_density: fluid.critical_density(),
        critical_pressure: fluid.critical_pressure(),
        critical_temperature: fluid.critical_temperature(),
        density: fluid.density(),
        dynamic_viscosity: fluid.dynamic_viscosity(),
        enthalpy: fluid.enthalpy(),
        entropy: fluid.entropy(),
        fundamental_derivative_of_gas_dynamics: fluid.fundamental_derivative_of_gas_dynamics(),
        gibbs_energy: fluid.gibbs_energy(),
        helmholtz_energy: fluid.helmholtz_energy(),
        ideal_gas_enthalpy: fluid.ideal_gas_enthalpy(),
        ideal_gas_entropy: fluid.ideal_gas_entropy(),
        ideal_gas_internal_energy: fluid.ideal_gas_internal_energy(),
        ideal_gas_molar_enthalpy: fluid.ideal_gas_molar_enthalpy(),
        ideal_gas_molar_entropy: fluid.ideal_gas_molar_entropy(),
        ideal_gas_molar_internal_energy: fluid.ideal_gas_molar_internal_energy(),
        ideal_gas_molar_specific_heat_constant_pressure: fluid.ideal_gas_molar_specific_heat(),
        ideal_gas_specific_heat_constant_pressure: fluid.ideal_gas_specific_heat(),
        internal_energy: fluid.internal_energy(),
        isentropic_expansion_coefficient: fluid.isentropic_expansion_coefficient(),
        isothermal_compressibility: fluid.isothermal_compressibility(),
        kinematic_viscosity: fluid.kinematic_viscosity(),
        molar_density: fluid.molar_density(),
        molar_enthalpy: fluid.molar_enthalpy(),
        molar_entropy: fluid.molar_entropy(),
        molar_gibbs_energy: fluid.molar_gibbs_energy(),
        molar_helmholtz_energy: fluid.molar_helmholtz_energy(),
        molar_internal_energy: fluid.molar_internal_energy(),
        molar_quality: fluid.molar_quality(),
        molar_specific_heat_constant_pressure: fluid.molar_specific_heat(),
        molar_specific_heat_constant_volume: fluid.molar_specific_heat_const_volume(),
        molar_mass: fluid.molar_mass(),
        ozone_depletion_potential: fluid.odp(),
        phase: fluid.phase(),
        phase_id_param: fluid.phase_id_param(),
        prandtl_number: fluid.prandtl(),
        pressure: fluid.pressure(),
        quality: fluid.quality(),
        ratio_of_specific_heats: ratio_specific_heats(
            fluid.specific_heat(),
            fluid.specific_heat_const_volume(),
        ),
        residual_molar_enthalpy: fluid.residual_molar_enthalpy(),
        residual_molar_entropy: fluid.residual_molar_entropy(),
        residual_molar_gibbs_energy: fluid.residual_molar_gibbs_energy(),
        specific_heat_constant_pressure: fluid.specific_heat(),
        specific_heat_constant_volume: fluid.specific_heat_const_volume(),
        speed_of_sound: fluid.sound_speed(),
        specific_volume: fluid.specific_volume(),
        surface_tension: fluid.surface_tension(),
        tau: fluid.tau(),
        temperature: fluid.temperature(),
        triple_point_pressure: fluid.triple_pressure(),
        triple_point_temperature: fluid.triple_temperature(),
    }
}

// Calculated thermodynamic properties
fn ratio_specific_heats(
    cp: OutputResult<f64>,
    cv: OutputResult<f64>,
) -> Result<f64, FluidOutputError> {
    let cp = cp?;
    let cv = cv?;
    Ok(cp / cv)
}
