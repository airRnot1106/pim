mod args;
mod consts;
mod core;
mod ext;
mod utils;

use args::Args;
use clap::Parser;
use core::{converter::UnitConverter, unit::ConvertibleUnit};
use ext::units::{em::Em, pt::Pt, px::Px, rem::Rem};

fn main() {
    // parse CLI arguments
    let args = Args::parse();

    // register units and create the converter
    let units: Vec<Box<dyn ConvertibleUnit>> =
        vec![Box::new(Px), Box::new(Pt), Box::new(Em), Box::new(Rem)];
    let converter = UnitConverter::new(units);

    if !converter.list_units().contains(&args.unit) {
        eprintln!("Error: Unsupported unit: '{}'.", args.unit);
        return;
    }

    // process unit conversions
    for target_unit in converter.list_units() {
        match converter.convert(args.value, &args.unit, &target_unit, args.root_font_size_px) {
            Ok(result) => println!(
                "{}: {}{}",
                target_unit,
                utils::format_number(result),
                target_unit
            ),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
