use crate::core::unit::ConvertibleUnit;

pub struct Rem;

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
