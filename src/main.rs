use std::path::Path;
use clap::Parser;
use quick_xml::se::to_string;
use crate::directory::read_directory;
use crate::cli::Cli;

mod directory;
mod cli;


fn main() {
    let args = Cli::parse();
    let dir_path = Path::new(&args.input);

    let project_structure = read_directory(dir_path);

    let xml_output = to_string(&project_structure).unwrap();
    println!("{}", xml_output);
}
