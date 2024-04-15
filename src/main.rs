mod directory;

use std::path::Path;
use quick_xml::se::to_string;
use directory::read_directory;


fn main() {
    let dir_path = Path::new("./src");
    let project_structure = read_directory(dir_path);

    let xml_output = to_string(&project_structure).unwrap();
    println!("{}", xml_output);
}
