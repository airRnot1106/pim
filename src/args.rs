use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub value: f64,
    pub unit: String,

    #[arg(short, long, default_value_t = 16.0)]
    pub root_font_size_px: f64,
}
