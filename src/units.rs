#![allow(non_camel_case_types)]
pub struct UnitsState {}

pub enum UnitConductivity {
    W_m_K,
}

pub enum UnitDensity {
    kg_m3,
    lbm_ft3,
}

pub enum UnitPressure {
    atm,
    Pa,
    kPa,
    MPa,
    Bar,
    PSI,
    kg_cm2,
}

pub enum UnitTemperature {
    K,
    R,
    C,
    F,
}

pub enum UnitViscosity {
    Pa_s,
    kg_m_s,
    cP,
}

pub enum UnitSpecificEnergy {
    J_kg,
    kJ_kg,
}

pub enum UnitSpecificHeatCapacity {
    J_kg_K,
    kJ_kg_K,
}

pub enum UnitMolarEnthalpy {
    J_mol,
    kJ_mol,
}

pub enum UnitMoldarEntropy {
    J_mol_K,
    kJ_mol_K,
}

pub enum UnitVelocity {
    m_s,
    ft_s,
    km_hr,
    mi_hr,
}
