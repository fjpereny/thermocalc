use rfluids::prelude::*;
use strum::EnumIter;

#[allow(non_camel_case_types)]
#[derive(Debug, EnumIter, PartialEq)]
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
    DimethylEther,
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
    MDM,
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
            AvailableFluids::DimethylEther => Pure::DimethylEther,
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
            AvailableFluids::MDM => Pure::MDM,
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
        match name.as_ref() {
            // Complete CoolProp pure/pseudo-pure fluid name → AvailableFluids mapping (137 fluids)
            "1-Butene" => Ok(AvailableFluids::OneButene),
            "Acetone" => Ok(AvailableFluids::Acetone),
            "Air" => Ok(AvailableFluids::Air),
            "Ammonia" => Ok(AvailableFluids::Ammonia),
            "Argon" => Ok(AvailableFluids::Argon),
            "Benzene" => Ok(AvailableFluids::Benzene),
            "CarbonDioxide" => Ok(AvailableFluids::CarbonDioxide),
            "CarbonMonoxide" => Ok(AvailableFluids::CarbonMonoxide),
            "CarbonylSulfide" => Ok(AvailableFluids::CarbonylSulfide),
            "Chlorine" => Ok(AvailableFluids::Chlorine),
            "cis-2-Butene" => Ok(AvailableFluids::cis2Butene),
            "Cyclohexane" => Ok(AvailableFluids::Cyclohexane),
            "Cyclopropane" => Ok(AvailableFluids::Cyclopropane),
            "Cyclopentane" => Ok(AvailableFluids::Cyclopentane),
            "D4" => Ok(AvailableFluids::D4),
            "D5" => Ok(AvailableFluids::D5),
            "D6" => Ok(AvailableFluids::D6),
            "Deuterium" => Ok(AvailableFluids::Deuterium),
            "Dichloroethane" => Ok(AvailableFluids::Dichloroethane),
            "DiethylEther" => Ok(AvailableFluids::DiethylEther),
            "DimethylCarbonate" => Ok(AvailableFluids::DimethylCarbonate),
            "DimethylEther" => Ok(AvailableFluids::DimethylEther),
            "Ethane" => Ok(AvailableFluids::Ethane),
            "Ethanol" => Ok(AvailableFluids::Ethanol),
            "EthylBenzene" => Ok(AvailableFluids::EthylBenzene),
            "Ethylene" => Ok(AvailableFluids::Ethylene),
            "EthyleneOxide" => Ok(AvailableFluids::EthyleneOxide),
            "Fluorine" => Ok(AvailableFluids::Fluorine),
            "HFE143m" => Ok(AvailableFluids::HFE143m),
            "HeavyWater" => Ok(AvailableFluids::HeavyWater),
            "Helium" => Ok(AvailableFluids::Helium),
            "Hydrogen" => Ok(AvailableFluids::Hydrogen),
            "HydrogenChloride" => Ok(AvailableFluids::HydrogenChloride),
            "HydrogenSulfide" => Ok(AvailableFluids::HydrogenSulfide),
            "Isobutane" => Ok(AvailableFluids::IsoButane),
            "Isobutene" => Ok(AvailableFluids::IsoButene),
            "Isohexane" => Ok(AvailableFluids::Isohexane),
            "Isopentane" => Ok(AvailableFluids::Isopentane),
            "Krypton" => Ok(AvailableFluids::Krypton),
            "m-Xylene" => Ok(AvailableFluids::mXylene),
            "MD2M" => Ok(AvailableFluids::MD2M),
            "MD3M" => Ok(AvailableFluids::MD3M),
            "MD4M" => Ok(AvailableFluids::MD4M),
            "MDM" => Ok(AvailableFluids::MDM),
            "MM" => Ok(AvailableFluids::MM),
            "Methane" => Ok(AvailableFluids::Methane),
            "Methanol" => Ok(AvailableFluids::Methanol),
            "MethylLinoleate" => Ok(AvailableFluids::MethylLinoleate),
            "MethylLinolenate" => Ok(AvailableFluids::MethylLinolenate),
            "MethylOleate" => Ok(AvailableFluids::MethylOleate),
            "MethylPalmitate" => Ok(AvailableFluids::MethylPalmitate),
            "MethylStearate" => Ok(AvailableFluids::MethylStearate),
            "n-Butane" => Ok(AvailableFluids::nButane),
            "n-Decane" => Ok(AvailableFluids::nDecane),
            "n-Dodecane" => Ok(AvailableFluids::nDodecane),
            "n-Heptane" => Ok(AvailableFluids::nHeptane),
            "n-Hexane" => Ok(AvailableFluids::nHexane),
            "n-Nonane" => Ok(AvailableFluids::nNonane),
            "n-Octane" => Ok(AvailableFluids::nOctane),
            "n-Pentane" => Ok(AvailableFluids::nPentane),
            "n-Perfluorobutane" => Ok(AvailableFluids::nPerfluorobutane),
            "n-Perfluorohexane" => Ok(AvailableFluids::nPerfluorohexane),
            "n-Perfluoropentane" => Ok(AvailableFluids::nPerfluoropentane),
            "n-Propane" => Ok(AvailableFluids::nPropane),
            "n-Undecane" => Ok(AvailableFluids::nUndecane),
            "Neon" => Ok(AvailableFluids::Neon),
            "Neopentane" => Ok(AvailableFluids::Neopentane),
            "Nitrogen" => Ok(AvailableFluids::Nitrogen),
            "NitrousOxide" => Ok(AvailableFluids::NitrousOxide),
            "Novec649" => Ok(AvailableFluids::Novec649),
            "o-Xylene" => Ok(AvailableFluids::oXylene),
            "OrthoDeuterium" => Ok(AvailableFluids::OrthoDeuterium),
            "OrthoHydrogen" => Ok(AvailableFluids::OrthoHydrogen),
            "Oxygen" => Ok(AvailableFluids::Oxygen),
            "p-Xylene" => Ok(AvailableFluids::pXylene),
            "ParaDeuterium" => Ok(AvailableFluids::ParaDeuterium),
            "ParaHydrogen" => Ok(AvailableFluids::ParaHydrogen),
            "Propylene" => Ok(AvailableFluids::Propylene),
            "PropyleneGlycol" => Ok(AvailableFluids::PropyleneGlycol),
            "Propyne" => Ok(AvailableFluids::Propyne),
            "R11" => Ok(AvailableFluids::R11),
            "R1123" => Ok(AvailableFluids::R1123),
            "R113" => Ok(AvailableFluids::R113),
            "R1130(E)" => Ok(AvailableFluids::R1130E),
            "R1132(E)" => Ok(AvailableFluids::R1132E),
            "R1132a" => Ok(AvailableFluids::R1132E),
            "R114" => Ok(AvailableFluids::R114),
            "R115" => Ok(AvailableFluids::R115),
            "R116" => Ok(AvailableFluids::R116),
            "R12" => Ok(AvailableFluids::R12),
            "R1224YDZ" => Ok(AvailableFluids::R1224YDZ),
            "R123" => Ok(AvailableFluids::R123),
            "R1233zd(E)" => Ok(AvailableFluids::R1233zdE),
            "R1234yf" => Ok(AvailableFluids::R1234yf),
            "R1234ze(E)" => Ok(AvailableFluids::R1234zeE),
            "R1234ze(Z)" => Ok(AvailableFluids::R1234zeZ),
            "R124" => Ok(AvailableFluids::R124),
            "R1243zf" => Ok(AvailableFluids::R1243zf),
            "R125" => Ok(AvailableFluids::R125),
            "R13" => Ok(AvailableFluids::R13),
            "R1336mzz(E)" => Ok(AvailableFluids::R1336mzzE),
            "R1336mzz(Z)" => Ok(AvailableFluids::R1336mzzZ),
            "R134a" => Ok(AvailableFluids::R134a),
            "R13I1" => Ok(AvailableFluids::R13I1),
            "R14" => Ok(AvailableFluids::R14),
            "R141b" => Ok(AvailableFluids::R141b),
            "R142b" => Ok(AvailableFluids::R142b),
            "R143a" => Ok(AvailableFluids::R143a),
            "R152a" => Ok(AvailableFluids::R152a),
            "R161" => Ok(AvailableFluids::R161),
            "R21" => Ok(AvailableFluids::R21),
            "R218" => Ok(AvailableFluids::R218),
            "R22" => Ok(AvailableFluids::R22),
            "R227ea" => Ok(AvailableFluids::R227ea),
            "R23" => Ok(AvailableFluids::R23),
            "R236ea" => Ok(AvailableFluids::R236ea),
            "R236fa" => Ok(AvailableFluids::R236fa),
            "R245ca" => Ok(AvailableFluids::R245ca),
            "R245fa" => Ok(AvailableFluids::R245fa),
            "R32" => Ok(AvailableFluids::R32),
            "R365mfc" => Ok(AvailableFluids::R365mfc),
            "R40" => Ok(AvailableFluids::R40),
            "R404A" => Ok(AvailableFluids::R404A),
            "R407C" => Ok(AvailableFluids::R407C),
            "R41" => Ok(AvailableFluids::R41),
            "R410A" => Ok(AvailableFluids::R410A),
            "R507A" => Ok(AvailableFluids::R507A),
            "RC318" => Ok(AvailableFluids::RC318),
            "SES36" => Ok(AvailableFluids::SES36),
            "SulfurDioxide" => Ok(AvailableFluids::SulfurDioxide),
            "SulfurHexafluoride" => Ok(AvailableFluids::SulfurHexafluoride),
            "Tetrahydrofuran" => Ok(AvailableFluids::Tetrahydrofuran),
            "Toluene" => Ok(AvailableFluids::Toluene),
            "trans-2-Butene" => Ok(AvailableFluids::trans2Butene),
            "VinylChloride" => Ok(AvailableFluids::VinylChloride),
            "Water" => Ok(AvailableFluids::Water),
            "Xenon" => Ok(AvailableFluids::Xenon),
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

    #[test]
    fn name_to_fluid_match() {
        for fluid in AvailableFluids::iter() {
            let name = fluid.get_name();
            let found_fluid_result = AvailableFluids::name_to_fluid(&name);
            assert!(
                found_fluid_result.is_ok(),
                "Error: {} not found in name_to_fluid()",
                name
            );
            let found_fluid = found_fluid_result.unwrap();
            assert_eq!(
                fluid, found_fluid,
                "Error: Fluid {:?} -> Name {:?} -> Fluid {:?} does not match",
                fluid, name, found_fluid
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
                println!("WARNING: Duplicate fluid found! -=> {:?}", duplicate)
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
