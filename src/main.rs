use clap::Parser;
use indexmap::IndexMap;
extern crate num;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    value: f64,
    unit: String,

    #[arg(short, long, default_value_t = 16.0)]
    root_font_size_px: f64,
}

trait ConvertibleUnit {
    fn get_name(&self) -> &'static str;
    fn to_px(&self, value: f64, root_font_size_px: f64) -> f64;
    fn from_px(&self, value_in_px: f64, root_font_size_px: f64) -> f64;
}

struct Px;

impl ConvertibleUnit for Px {
    fn get_name(&self) -> &'static str {
        "px"
    }

    fn to_px(&self, value: f64, _: f64) -> f64 {
        value
    }

    fn from_px(&self, value_in_px: f64, _: f64) -> f64 {
        value_in_px
    }
}

struct Em;

impl ConvertibleUnit for Em {
    fn get_name(&self) -> &'static str {
        "em"
    }

    fn to_px(&self, value: f64, root_font_size_px: f64) -> f64 {
        value * root_font_size_px
    }

    fn from_px(&self, value_in_px: f64, root_font_size_px: f64) -> f64 {
        value_in_px / root_font_size_px
    }
}

struct Rem;

impl ConvertibleUnit for Rem {
    fn get_name(&self) -> &'static str {
        "rem"
    }

    fn to_px(&self, value: f64, root_font_size_px: f64) -> f64 {
        value * root_font_size_px
    }

    fn from_px(&self, value_in_px: f64, root_font_size_px: f64) -> f64 {
        value_in_px / root_font_size_px
    }
}

struct UnitConverter {
    converters: IndexMap<String, Box<dyn ConvertibleUnit>>,
}

impl UnitConverter {
    fn new() -> Self {
        let units: Vec<Box<dyn ConvertibleUnit>> = vec![Box::new(Px), Box::new(Em), Box::new(Rem)];

        let mut converters: IndexMap<String, Box<dyn ConvertibleUnit>> = IndexMap::new();

        for unit in units {
            converters.insert(unit.get_name().to_string(), unit);
        }

        Self { converters }
    }

    fn convert(
        &self,
        value: f64,
        from_unit: &str,
        to_unit: &str,
        root_font_size_px: f64,
    ) -> Result<f64, String> {
        let from_converter = self
            .converters
            .get(from_unit)
            .ok_or_else(|| format!("Unsupported unit: '{}'.", from_unit))?;

        let to_converter = self
            .converters
            .get(to_unit)
            .ok_or_else(|| format!("Unsupported unit: '{}'.", to_unit))?;

        let value_in_px = from_converter.to_px(value, root_font_size_px);
        Ok(to_converter.from_px(value_in_px, root_font_size_px))
    }

    fn list_units(&self) -> Vec<String> {
        self.converters.keys().cloned().collect()
    }
}

fn format_number(num: f64) -> String {
    if num::rational::Ratio::from_float(num).unwrap().is_integer() {
        (num as i64).to_string()
    } else {
        num.to_string()
    }
}

fn main() {
    let args = Args::parse();
    let converter = UnitConverter::new();

    for target_unit in converter.list_units() {
        match converter.convert(args.value, &args.unit, &target_unit, args.root_font_size_px) {
            Ok(result) => println!("{}: {}{}", target_unit, format_number(result), target_unit),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
