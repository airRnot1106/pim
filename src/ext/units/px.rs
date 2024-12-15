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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name() {
        let px = Px;
        assert_eq!(px.get_name(), "px");
    }

    #[test]
    fn test_to_px() {
        let px = Px;
        // `to_px` should return the same value for `px`
        assert_eq!(px.to_px(10.0, 16.0), 10.0);
        assert_eq!(px.to_px(0.0, 16.0), 0.0);
        assert_eq!(px.to_px(1.0, 1.0), 1.0);
        assert_eq!(px.to_px(1.0, 20.0), 1.0);
    }

    #[test]
    fn test_from_px() {
        let px = Px;
        // `from_px` should return the same value for `px`
        assert_eq!(px.to_px(10.0, 16.0), 10.0);
        assert_eq!(px.to_px(0.0, 16.0), 0.0);
        assert_eq!(px.to_px(1.0, 1.0), 1.0);
        assert_eq!(px.to_px(1.0, 20.0), 1.0);
    }
}
