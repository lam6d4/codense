use clap::Parser;

#[derive(Parser)]
#[command(name = "codense")]
#[command(about = "Generate an XML representation of a directory structure")]
pub struct Cli {
    #[arg(help = "Path to the input directory")]
    pub input: String,
}
