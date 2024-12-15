use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The value to convert
    pub value: f64,
    /// The unit to convert to
    pub unit: String,

    /// The font size in pixels of the root element
    #[arg(short, long, default_value_t = 16.0)]
    pub root_font_size_px: f64,
}
