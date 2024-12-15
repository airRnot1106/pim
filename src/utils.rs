extern crate num;

/// Format a float to an integer if the value is an integer, otherwise format it as is into a string representation.
pub fn format_number(num: f64) -> String {
    if num::rational::Ratio::from_float(num).unwrap().is_integer() {
        (num as i64).to_string()
    } else {
        num.to_string()
    }
}
