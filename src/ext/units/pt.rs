use crate::consts::DECIMAL_PLACES;
use crate::{core::unit::ConvertibleUnit, utils::round_to_places};

pub struct Pt;

impl ConvertibleUnit for Pt {
    fn get_name(&self) -> &'static str {
        "pt"
    }

    fn to_px(&self, value: f64, _: f64) -> f64 {
        round_to_places(value * 1.333333, DECIMAL_PLACES)
    }

    fn from_px(&self, value_in_px: f64, _: f64) -> f64 {
        round_to_places(value_in_px / 1.333333, DECIMAL_PLACES)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name() {
        let pt = Pt;
        assert_eq!(pt.get_name(), "pt");
    }

    #[test]
    fn test_to_px() {
        let pt = Pt;
        assert_eq!(pt.to_px(1.0, 16.0), 1.333333);
        assert_eq!(pt.to_px(0.0, 16.0), 0.0);
        assert_eq!(pt.to_px(1.0, 1.0), 1.333333);
        assert_eq!(pt.to_px(0.75, 20.0), 1.0);
    }

    #[test]
    fn test_from_px() {
        let pt = Pt;
        assert_eq!(pt.from_px(1.0, 16.0), 0.75);
        assert_eq!(pt.from_px(0.0, 16.0), 0.0);
        assert_eq!(pt.from_px(1.0, 1.0), 0.75);
        assert_eq!(pt.from_px(1.333333, 20.0), 1.0);
    }
}
