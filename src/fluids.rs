use rfluids::prelude::*;
use strum::EnumIter;

#[allow(non_camel_case_types)]
#[derive(Debug, EnumIter)]
pub enum AvailableFluids {
    OneButene,
    Acetone,
    Air,
    Ammonia,
    Argon,
    Benzene,
    CarbonDioxide,
    CarbonMonoxide,
    CarbonylSulfide,
    Chlorine,
    Cyclohexane,
    Cyclopropane,
    Cyclopentane,
    D4,
    D5,
    D6,
    Deuterium,
    Dichloroethane,
    DiethylEther,
    DimethylCarbonate,
    Ethane,
    Ethanol,
    EthylBenzene,
    Ethylene,
    EthyleneOxide,
    Fluorine,
    HFE143m,
    HeavyWater,
    Helium,
    Hydrogen,
    HydrogenChloride,
    HydrogenSulfide,
    IsoButane,
    IsoButene,
    Isohexane,
    Isopentane,
    Krypton,
    MD2M,
    MD3M,
    MD4M,
    MM,
    Methane,
    Methanol,
    MethylLinoleate,
    MethylLinolenate,
    MethylOleate,
    MethylPalmitate,
    MethylStearate,
    Neon,
    Neopentane,
    Nitrogen,
    NitrousOxide,
    Novec649,
    OrthoDeuterium,
    OrthoHydrogen,
    Oxygen,
    ParaDeuterium,
    ParaHydrogen,
    Propylene,
    PropyleneGlycol,
    Propyne,
    R11,
    R1123,
    R113,
    R1130E,
    R1132E,
    R114,
    R115,
    R116,
    R12,
    R1224YDZ,
    R123,
    R1233zdE,
    R1234yf,
    R1234zeE,
    R1234zeZ,
    R124,
    R1243zf,
    R125,
    R13,
    R1336mzzE,
    R1336mzzZ,
    R134a,
    R13I1,
    R14,
    R141b,
    R142b,
    R143a,
    R152a,
    R161,
    R21,
    R218,
    R22,
    R227ea,
    R23,
    R236ea,
    R236fa,
    R245ca,
    R245fa,
    R32,
    R365mfc,
    R40,
    R404A,
    R407C,
    R41,
    R410A,
    R507A,
    RC318,
    SES36,
    SulfurDioxide,
    SulfurHexafluoride,
    Tetrahydrofuran,
    Toluene,
    VinylChloride,
    Water,
    Xenon,
    cis2Butene,
    mXylene,
    nButane,
    nDecane,
    nDodecane,
    nHeptane,
    nHexane,
    nNonane,
    nOctane,
    nPentane,
    nPerfluorobutane,
    nPerfluorohexane,
    nPerfluoropentane,
    nPropane,
    nUndecane,
    oXylene,
    pXylene,
    trans2Butene,
}

impl AvailableFluids {
    pub fn get_fluid(&self) -> Pure {
        match self {
            AvailableFluids::OneButene => Pure::Butene,
            AvailableFluids::Acetone => Pure::Acetone,
            AvailableFluids::Air => Pure::Air,
            AvailableFluids::Ammonia => Pure::Ammonia,
            AvailableFluids::Argon => Pure::Argon,
            AvailableFluids::Benzene => Pure::Benzene,
            AvailableFluids::CarbonDioxide => Pure::CarbonDioxide,
            AvailableFluids::CarbonMonoxide => Pure::CarbonMonoxide,
            AvailableFluids::CarbonylSulfide => Pure::CarbonylSulfide,
            AvailableFluids::Chlorine => Pure::Chlorine,
            AvailableFluids::Cyclohexane => Pure::Cyclohexane,
            AvailableFluids::Cyclopropane => Pure::Cyclopropane,
            AvailableFluids::Cyclopentane => Pure::Cyclopentane,
            AvailableFluids::D4 => Pure::D4,
            AvailableFluids::D5 => Pure::D5,
            AvailableFluids::D6 => Pure::D6,
            AvailableFluids::Deuterium => Pure::Deuterium,
            AvailableFluids::Dichloroethane => Pure::Dichloroethane,
            AvailableFluids::DiethylEther => Pure::DiethylEther,
            AvailableFluids::DimethylCarbonate => Pure::DimethylCarbonate,
            AvailableFluids::Ethane => Pure::Ethane,
            AvailableFluids::Ethanol => Pure::Ethanol,
            AvailableFluids::EthylBenzene => Pure::Ethylbenzene,
            AvailableFluids::Ethylene => Pure::Ethylene,
            AvailableFluids::EthyleneOxide => Pure::EthyleneOxide,
            AvailableFluids::Fluorine => Pure::Fluorine,
            AvailableFluids::HFE143m => Pure::HFE143m,
            AvailableFluids::HeavyWater => Pure::HeavyWater,
            AvailableFluids::Helium => Pure::Helium,
            AvailableFluids::Hydrogen => Pure::Hydrogen,
            AvailableFluids::HydrogenChloride => Pure::HydrogenChloride,
            AvailableFluids::HydrogenSulfide => Pure::HydrogenSulfide,
            AvailableFluids::IsoButane => Pure::Isobutane,
            AvailableFluids::IsoButene => Pure::Isobutene,
            AvailableFluids::Isohexane => Pure::Isohexane,
            AvailableFluids::Isopentane => Pure::Isopentane,
            AvailableFluids::Krypton => Pure::Krypton,
            AvailableFluids::MD2M => Pure::MD2M,
            AvailableFluids::MD3M => Pure::MD3M,
            AvailableFluids::MD4M => Pure::MD4M,
            AvailableFluids::MM => Pure::MM,
            AvailableFluids::Methane => Pure::Methane,
            AvailableFluids::Methanol => Pure::Methanol,
            AvailableFluids::MethylLinoleate => Pure::MethylLinoleate,
            AvailableFluids::MethylLinolenate => Pure::MethylLinolenate,
            AvailableFluids::MethylOleate => Pure::MethylOleate,
            AvailableFluids::MethylPalmitate => Pure::MethylPalmitate,
            AvailableFluids::MethylStearate => Pure::MethylStearate,
            AvailableFluids::Neon => Pure::Neon,
            AvailableFluids::Neopentane => Pure::Neopentane,
            AvailableFluids::Nitrogen => Pure::Nitrogen,
            AvailableFluids::NitrousOxide => Pure::NitrousOxide,
            AvailableFluids::Novec649 => Pure::Novec649,
            AvailableFluids::OrthoDeuterium => Pure::Orthodeuterium,
            AvailableFluids::OrthoHydrogen => Pure::Orthohydrogen,
            AvailableFluids::Oxygen => Pure::Oxygen,
            AvailableFluids::ParaDeuterium => Pure::Paradeuterium,
            AvailableFluids::ParaHydrogen => Pure::Parahydrogen,
            AvailableFluids::Propylene => Pure::Propylene,
            AvailableFluids::PropyleneGlycol => Pure::PropyleneGlycol,
            AvailableFluids::Propyne => Pure::Propyne,
            AvailableFluids::R11 => Pure::R11,
            AvailableFluids::R1123 => Pure::R1123,
            AvailableFluids::R113 => Pure::R113,
            AvailableFluids::R1130E => Pure::R1130E,
            AvailableFluids::R1132E => Pure::R1132E,
            AvailableFluids::R114 => Pure::R114,
            AvailableFluids::R115 => Pure::R115,
            AvailableFluids::R116 => Pure::R116,
            AvailableFluids::R12 => Pure::R12,
            AvailableFluids::R1224YDZ => Pure::R1224ydZ,
            AvailableFluids::R123 => Pure::R123,
            AvailableFluids::R1233zdE => Pure::R1233zdE,
            AvailableFluids::R1234yf => Pure::R1234yf,
            AvailableFluids::R1234zeE => Pure::R1234zeE,
            AvailableFluids::R1234zeZ => Pure::R1234zeZ,
            AvailableFluids::R124 => Pure::R124,
            AvailableFluids::R1243zf => Pure::R1243zf,
            AvailableFluids::R125 => Pure::R125,
            AvailableFluids::R13 => Pure::R13,
            AvailableFluids::R1336mzzE => Pure::R1336mzzE,
            AvailableFluids::R1336mzzZ => Pure::R1336mzzZ,
            AvailableFluids::R134a => Pure::R134a,
            AvailableFluids::R13I1 => Pure::R13I1,
            AvailableFluids::R14 => Pure::R14,
            AvailableFluids::R141b => Pure::R141b,
            AvailableFluids::R142b => Pure::R142b,
            AvailableFluids::R143a => Pure::R143a,
            AvailableFluids::R152a => Pure::R152a,
            AvailableFluids::R161 => Pure::R161,
            AvailableFluids::R21 => Pure::R21,
            AvailableFluids::R218 => Pure::R218,
            AvailableFluids::R22 => Pure::R22,
            AvailableFluids::R227ea => Pure::R227ea,
            AvailableFluids::R23 => Pure::R23,
            AvailableFluids::R236ea => Pure::R236ea,
            AvailableFluids::R236fa => Pure::R236fa,
            AvailableFluids::R245ca => Pure::R245ca,
            AvailableFluids::R245fa => Pure::R245fa,
            AvailableFluids::R32 => Pure::R32,
            AvailableFluids::R365mfc => Pure::R365mfc,
            AvailableFluids::R40 => Pure::R40,
            AvailableFluids::R404A => Pure::R404A,
            AvailableFluids::R407C => Pure::R407C,
            AvailableFluids::R41 => Pure::R41,
            AvailableFluids::R410A => Pure::R410A,
            AvailableFluids::R507A => Pure::R507A,
            AvailableFluids::RC318 => Pure::RC318,
            AvailableFluids::SES36 => Pure::SES36,
            AvailableFluids::SulfurDioxide => Pure::SulfurDioxide,
            AvailableFluids::SulfurHexafluoride => Pure::SulfurHexafluoride,
            AvailableFluids::Tetrahydrofuran => Pure::Tetrahydrofuran,
            AvailableFluids::Toluene => Pure::Toluene,
            AvailableFluids::VinylChloride => Pure::VinylChloride,
            AvailableFluids::Water => Pure::Water,
            AvailableFluids::Xenon => Pure::Xenon,
            AvailableFluids::cis2Butene => Pure::cis2Butene,
            AvailableFluids::mXylene => Pure::mXylene,
            AvailableFluids::nButane => Pure::nButane,
            AvailableFluids::nDecane => Pure::nDecane,
            AvailableFluids::nDodecane => Pure::nDodecane,
            AvailableFluids::nHeptane => Pure::nHeptane,
            AvailableFluids::nHexane => Pure::nHexane,
            AvailableFluids::nNonane => Pure::nNonane,
            AvailableFluids::nOctane => Pure::nOctane,
            AvailableFluids::nPentane => Pure::nPentane,
            AvailableFluids::nPerfluorobutane => Pure::nPerfluorobutane,
            AvailableFluids::nPerfluorohexane => Pure::nPerfluorohexane,
            AvailableFluids::nPerfluoropentane => Pure::nPerfluoropentane,
            AvailableFluids::nPropane => Pure::nPropane,
            AvailableFluids::nUndecane => Pure::nUndecane,
            AvailableFluids::oXylene => Pure::oXylene,
            AvailableFluids::pXylene => Pure::pXylene,
            AvailableFluids::trans2Butene => Pure::trans2Butene,
        }
    }

    pub fn name_to_fluid(name: impl AsRef<str>) -> Result<AvailableFluids, &'static str> {
        match name {
            _ => Err("Invalid fluid name"),
        }
    }

    pub fn get_name(&self) -> String {
        self.get_fluid().as_ref().to_string()
    }
}

pub fn create_pure_state(
    fluid: &AvailableFluids,
    pressure_pa: f64,
    temp_k: f64,
) -> Result<Fluid, rfluids::Error> {
    let p = FluidInput::pressure(pressure_pa);
    let t = FluidInput::temperature(temp_k);
    let new_fluid = Fluid::from(fluid.get_fluid()).in_state(p, t)?;
    Ok(new_fluid)
}

#[cfg(test)]
mod tests {

    use super::*;
    use rfluids::prelude::Pure;
    use strum::IntoEnumIterator;

    #[test]
    fn fluid_names_valid_for_coolprop() {
        for fluid in AvailableFluids::iter() {
            let gas_name = fluid.get_name();
            let initialization = Pure::try_from(gas_name.as_str());
            assert!(
                initialization.is_ok(),
                "Error: {:?} has invalid gas_name",
                fluid
            );
        }
    }

    use std::collections::HashSet;
    #[test]
    fn check_for_duplicate_fluids() {
        let mut items: Vec<Pure> = Vec::new();
        for item in AvailableFluids::iter() {
            let cur_fluid = item.get_fluid();
            items.push(cur_fluid.clone());

            let mut seen = HashSet::new();

            let duplicate_list: Vec<_> = items
                .iter()
                .filter(|item| !seen.insert(*item))
                .cloned() // Clones the references to give you owned data values if needed
                .collect();

            for duplicate in &duplicate_list {
                println!("WARNING: Duplicate fluid found! --> {:?}", duplicate)
            }

            assert!(
                duplicate_list.len() == 0,
                "Duplicate fluids found in fluids.rs"
            )
        }
    }

    #[test]
    fn create_all_pure_fluids_at() {
        let p_stp_pa = 100_000.0;
        let t_stp_k = 300.00;
        for cur_fluid in AvailableFluids::iter() {
            let test_state = create_pure_state(&cur_fluid, p_stp_pa, t_stp_k);
            assert!(
                test_state.is_ok(),
                "Error: CoolProp was unable to initalize {:?} at STP\n
                CoolProp Error: {:?}",
                &cur_fluid,
                test_state
            )
        }
    }
}
