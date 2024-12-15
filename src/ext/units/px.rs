use crate::core::unit::ConvertibleUnit;

pub struct Px;

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
