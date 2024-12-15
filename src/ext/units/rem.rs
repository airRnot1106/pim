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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name() {
        let rem = Rem;
        assert_eq!(rem.get_name(), "rem");
    }

    #[test]
    fn test_to_px() {
        let rem = Rem;
        assert_eq!(rem.to_px(1.0, 16.0), 16.0); // 1rem = 16px
        assert_eq!(rem.to_px(1.0, 12.0), 12.0); // 1rem(root:12px) = 12px
        assert_eq!(rem.to_px(2.0, 16.0), 32.0); // 2rem = 32px
    }

    #[test]
    fn test_from_px() {
        let rem = Rem;
        assert_eq!(rem.from_px(16.0, 16.0), 1.0); // 16px = 1rem
        assert_eq!(rem.from_px(16.0, 12.0), 1.3333333333333333); // 16px(root:12px) = 1.3333333333333333rem
        assert_eq!(rem.from_px(32.0, 16.0), 2.0); // 32px = 2rem
    }
}
