/// A trait representing a unit of measurement that can be converted to and from pixels (px).
/// This is the foundation for implementing custom units.
pub trait ConvertibleUnit {
    /// Returns the name of the unit
    fn get_name(&self) -> &'static str;

    /// Converts a value from this unit to pixels (px).
    ///
    /// # Parameters
    /// - `value`: The value in the current unit.
    /// - `root_font_size_px`: The root font size in pixels (used for relative units like `rem`).
    ///
    /// # Returns
    /// The equivalent value in pixels (px).
    fn to_px(&self, value: f64, root_font_size_px: f64) -> f64;

    /// Converts a value from pixels (px) to this unit.
    ///
    /// # Parameters
    /// - `value_in_px`: The value in pixels.
    /// - `root_font_size_px`: The root font size in pixels (used for relative units like `rem`).
    ///
    /// # Returns
    /// The equivalent value in the current unit.
    fn from_px(&self, value_in_px: f64, root_font_size_px: f64) -> f64;
}
