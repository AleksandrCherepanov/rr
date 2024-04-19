use clap::{Args, Parser, Subcommand, ValueEnum};

pub mod dot;
pub mod line;
pub mod triangle;
pub mod skeleton;
pub mod model;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Dots(DotsArgs),
    Lines(LinesArgs),
    Triangles(TrianglesArgs),
    Skeleton(SkeletonArgs),
    Model(ModelArgs),
}

#[derive(Args)]
pub struct DotsArgs {
    #[arg(long, help = "Width of result image in pixels. Default: 800")]
    pub width: Option<usize>,
    #[arg(long, help = "Height of result image in pixels. Default: 800")]
    pub height: Option<usize>,
    #[arg(
    short,
    long,
    long_help = "Path to .txt file with coordinates. The format is X Y Color(optional). Default: white.
Awailable colors are: red, orange, yellow, green. blue, darkblue, purple. white."
    )]
    pub source: String,
    #[arg(short, long, help = "Path to .tga file where the result will be saved.")]
    pub target: String,
}


#[derive(Args)]
pub struct LinesArgs {
    #[arg(long, help = "Width of result image in pixels. Default: 800")]
    pub width: Option<usize>,
    #[arg(long, help = "Height of result image in pixels. Default: 800")]
    pub height: Option<usize>,
    #[arg(
    short,
    long,
    long_help = "Path to .txt file with coordinates. The format is X1 Y1 X2 Y2 Color(optional). Default: white.
Awailable colors are: red, orange, yellow, green. blue, darkblue, purple, white."
    )]
    pub source: String,
    #[arg(short, long, help = "Path to .tga file where the result will be saved.")]
    pub target: String,
}

#[derive(Args)]
pub struct TrianglesArgs {
    #[arg(long, help = "Width of result image in pixels. Default: 800")]
    pub width: Option<usize>,
    #[arg(long, help = "Height of result image in pixels. Default: 800")]
    pub height: Option<usize>,
    #[arg(
    short,
    long,
    long_help = "Path to .txt file with coordinates. The format is X1 Y1 X2 Y2 X3 Y3 Color(optional). Default: white.
Awailable colors are: red, orange, yellow, green. blue, darkblue, purple, white."
    )]
    pub source: String,
    #[arg(short, long, help = "Path to .tga file where the result will be saved.")]
    pub target: String,
    #[arg(
    short,
    long,
    long_help = "Fill triangle with a color"
    )]
    pub filled: bool,
}

#[derive(Args)]
pub struct SkeletonArgs {
    #[arg(long, help = "Width of result image in pixels. Default: 800")]
    pub width: Option<usize>,
    #[arg(long, help = "Height of result image in pixels. Default: 800")]
    pub height: Option<usize>,
    #[arg(short, long, long_help = "Path to .obj file with a model.")]
    pub source: String,
    #[arg(short, long, help = "Path to .tga file where the result will be saved.")]
    pub target: String,
    #[arg(value_enum)]
    pub color: Color,
}


#[derive(Args)]
pub struct ModelArgs {
    #[arg(long, help = "Width of result image in pixels. Default: 800")]
    pub width: Option<usize>,
    #[arg(long, help = "Height of result image in pixels. Default: 800")]
    pub height: Option<usize>,
    #[arg(short, long, long_help = "Path to .obj file with a model.")]
    pub source: String,
    #[arg(short, long, help = "Path to .tga file where the result will be saved.")]
    pub target: String,
    #[arg(long = "texture", long_help = "Path to .tga file with a texture. Default: white color is used.")]
    pub texture: Option<String>,
}

#[derive(ValueEnum, Clone)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    DarkBlue,
    Purple,
    White,
}
