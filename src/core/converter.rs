use crate::core::unit::ConvertibleUnit;
use indexmap::IndexMap;

/// A converter that manages units of measurement and performs conversions between them.
///
/// This struct is responsible for:
/// - Storing registered units.
/// - Handling conversions between units.
/// - Providing a list of available units.
pub struct UnitConverter {
    converters: IndexMap<String, Box<dyn ConvertibleUnit>>,
}

impl UnitConverter {
    /// Creates a new `UnitConverter` instance with the provided units.
    ///
    /// # Parameters
    /// - `units`: A list of units implementing the `ConvertibleUnit` trait.
    ///
    /// # Returns
    /// A `UnitConverter` instance with the given units registered.
    pub fn new(units: Vec<Box<dyn ConvertibleUnit>>) -> Self {
        let converters: IndexMap<_, _> = units
            .into_iter()
            .map(|unit| (unit.get_name().to_string(), unit))
            .collect();

        Self { converters }
    }

    /// Converts a value from one unit to another.
    ///
    /// # Parameters
    /// - `value`: The value to be converted.
    /// - `from_unit`: The name of the unit to convert from.
    /// - `to_unit`: The name of the unit to convert to.
    /// - `root_font_size_px`: The root font size in pixels (used for relative units like `rem`).
    ///
    /// # Returns
    /// - `Ok(f64)`: The converted value if the conversion was successful.
    /// - `Err(String)`: An error message if either the `from_unit` or `to_unit` is not supported.
    ///
    /// # Errors
    /// - Returns an error if either `from_unit` or `to_unit` is not found in the registered units.
    pub fn convert(
        &self,
        value: f64,
        from_unit: &str,
        to_unit: &str,
        root_font_size_px: f64,
    ) -> Result<f64, String> {
        let from_converter = self.converters.get(from_unit).unwrap();

        let to_converter = self.converters.get(to_unit).unwrap();

        let value_in_px = from_converter.to_px(value, root_font_size_px);
        Ok(to_converter.from_px(value_in_px, root_font_size_px))
    }

    /// Returns a list of all available unit names.
    ///
    /// # Returns
    /// A vector of unit names.
    pub fn list_units(&self) -> Vec<String> {
        self.converters.keys().cloned().collect()
    }
}
