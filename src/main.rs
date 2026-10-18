use clap::{arg, command};
use dialoguer::{FuzzySelect, Input};
use strum::IntoEnumIterator;
use thermocalc::fluids::{AvailableFluids, create_pure_state};
use thermocalc::thermo::fluid_state;

fn main() {
    let mut available_fluids = Vec::new();
    for fluid in AvailableFluids::iter() {
        available_fluids.push(fluid.get_name());
    }

    let selection = FuzzySelect::new()
        .with_prompt("Select a fluid (type to filter)")
        .items(&available_fluids)
        .interact()
        .unwrap();

    let pressure: f64 = Input::new()
        .with_prompt("Enter prerssure (Pa)")
        .allow_empty(false)
        .interact_text()
        .unwrap();

    let temperature: f64 = Input::new()
        .with_prompt("Enter temperature (K)")
        .allow_empty(false)
        .interact_text()
        .unwrap();

    let mut fluid = create_pure_state(&AvailableFluids::Nitrogen, pressure, temperature).unwrap();
    let state = fluid_state(&mut fluid);

    println!();
    println!("Thermo State");
    println!("-------------");
    println!("{:#?}", state);
}
