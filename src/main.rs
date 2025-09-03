use std::{path::PathBuf, str::FromStr};

use clap::{Parser, ValueEnum};

static DEFAULT_BAR_WIDTH: f64 = 16.0;
static DEFAULT_BAR_SPACING: f64 = 8.0;
static DEFAULT_BAR_HEIGHT_MIN: f64 = 16.0;
static DEFAULT_BAR_HEIGHT_MAX: f64 = 256.0;
static DEFAULT_BAR_RADIUS: f64 = 8.0;
static DEFAULT_CIRCLE_RADIUS: f64 = 64.0;
static DEFAULT_BAR_COLOR: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
static DEFAULT_VIDEO_FILE_STR: &str = "spectrum.mov";

#[derive(Debug, Parser)]
struct Cli {

    /// Path of the input audio file
    audio_file: PathBuf,

    /// Style of each frame of the spectrum plot
    #[arg(value_enum)]
    #[arg(default_value_t = Style::Linear)]
    #[arg(short = 's')]
    #[arg(long = "style")]
    style: Style,

    /// Width of bars in pixels
    #[arg(default_value_t = DEFAULT_BAR_WIDTH)]
    #[arg(long = "bar-width")]
    bar_width: f64,

    /// Spacing between bars in pixels (only for the linear style)
    #[arg(default_value_t = DEFAULT_BAR_SPACING)]
    #[arg(long = "bar-spacing")]
    bar_spacing: f64,

    /// Minimal height of bars in pixels
    #[arg(default_value_t = DEFAULT_BAR_HEIGHT_MIN)]
    #[arg(long = "bar-height-min")]
    bar_height_min: f64,

    /// Maximal height of bars in pixels
    #[arg(default_value_t = DEFAULT_BAR_HEIGHT_MAX)]
    #[arg(long = "bar-height-max")]
    bar_height_max: f64,

    /// Corner radius of bars in pixels
    #[arg(default_value_t = DEFAULT_BAR_RADIUS)]
    #[arg(long = "bar-radius")]
    bar_radius: f64,

    /// Radius of the inner circle in pixels (only for the circular style)
    #[arg(default_value_t = DEFAULT_CIRCLE_RADIUS)]
    #[arg(long = "circle-radius")]
    circle_radius: f64,

    /// Color of the bars in the format of rgba(red,green,blue,alpha)
    #[arg(value_parser)]
    #[arg(default_value_t = DEFAULT_BAR_COLOR.clone())]
    #[arg(long = "bar-color")]
    bar_color: Color,

    /// Path of the output video file
    #[arg(default_value = DEFAULT_VIDEO_FILE_STR)]
    #[arg(short = 'o')]
    #[arg(long = "video-file")]
    video_file: PathBuf,
}

#[derive(Clone, Debug, ValueEnum)]
enum Style {

    /// Linear style
    Linear,

    /// Circular style
    Circular,
}

#[derive(Clone, Debug)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        let r = (self.r * 255.0).round() as u8;
        let g = (self.g * 255.0).round() as u8;
        let b = (self.b * 255.0).round() as u8;
        let a = (self.a * 255.0).round() as u8;
        format!("rgba({r},{g},{b},{a})")
    }
}

fn main() {
    let _ = Cli::parse();
}
