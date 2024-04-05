use clap::{Args, Parser, Subcommand, ValueEnum};

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
    #[arg(
        short, 
        long, 
        long_help = "Path to .txt file with coordinates. The format is X Y Color(optional).
Awailable colors are: red, orange, yellow, green. blue, darkblue, purple"
    )]
    pub source: String,
    #[arg(short, long, help = "Path to .tga file where the result will be saved.")]
    pub target: String
}


#[derive(Args)]
pub struct LinesArgs {
    #[arg(
        short, 
        long, 
        long_help = "Path to .txt file with coordinates. The format is X1 Y1 X2 Y2 Color(optional).
Awailable colors are: red, orange, yellow, green. blue, darkblue, purple"
    )]
    pub source: String,
    #[arg(short, long, help = "Path to .tga file where the result will be saved.")]
    pub target: String
}

#[derive(Args)]
pub struct TrianglesArgs {
    #[arg(
        short, 
        long, 
        long_help = "Path to .txt file with coordinates. The format is X1 Y1 X2 Y2 X3 Y3 Color(optional).
Awailable colors are: red, orange, yellow, green. blue, darkblue, purple"
    )]
    pub source: String,
    #[arg(short, long, help = "Path to .tga file where the result will be saved.")]
    pub target: String,
    #[arg(
        short, 
        long, 
        long_help = "Fill triangle with a color"
    )]
    pub filled: bool
}

#[derive(Args)]
pub struct SkeletonArgs {
    #[arg(short, long, long_help = "Path to .obj file with a model.")]
    pub source: String,
    #[arg(short, long, help = "Path to .tga file where the result will be saved.")]
    pub target: String,
    #[arg(value_enum)]
    pub color: Color
}


#[derive(Args)]
pub struct ModelArgs {
    #[arg(short, long, long_help = "Path to .obj file with a model.")]
    pub source: String,
    #[arg(short, long, help = "Path to .tga file where the result will be saved.")]
    pub target: String,
    #[arg(long = "texture", long_help = "Path to .tga file with a texture.")]
    pub texture: Option<String>
}

#[derive(ValueEnum, Clone)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    DarkBlue,
    Purple
}