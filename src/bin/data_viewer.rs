use core::data;

use std::fs;

fn main() {
    println!("Identified Source files:");

    let paths = fs::read_dir("./chummer_data").unwrap();

    for path in paths {
        println!("\t{}", path.unwrap().path().display())
    }

    let chummer_path = String::from("./chummer_data/");

    println!("Importing Actions");
    let action_list =
        data::action::Action::init_action_list(&chummer_path).expect("Could not import actions");
    println!("Imported {} actions.", action_list.len());

    println!("Importing Programs.");
    let program_list = data::program::Program::init_program_list(&chummer_path)
        .expect("Could not import programs");
    println!("Imported {} programs.", program_list.len());
}
