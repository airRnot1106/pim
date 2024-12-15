use crate::core::unit::ConvertibleUnit;

pub struct Em;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name() {
        let em = Em;
        assert_eq!(em.get_name(), "em");
    }

    #[test]
    fn test_to_px() {
        let em = Em;
        assert_eq!(em.to_px(1.0, 16.0), 16.0); // 1rem = 16px
        assert_eq!(em.to_px(1.0, 12.0), 12.0); // 1rem(root:12px) = 12px
        assert_eq!(em.to_px(2.0, 16.0), 32.0); // 2rem = 32px
    }

    #[test]
    fn test_from_px() {
        let em = Em;
        assert_eq!(em.from_px(16.0, 16.0), 1.0); // 16px = 1em
        assert_eq!(em.from_px(16.0, 12.0), 1.3333333333333333); // 16px(root:12px) = 1.3333333333333333rem
        assert_eq!(em.from_px(32.0, 16.0), 2.0); // 32px = 2em
    }
}
