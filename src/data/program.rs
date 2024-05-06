use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Program {
    id: String,
    name: String,
    category: Category,
    hide: Option<String>,
    ignoresourcedisabled: Option<String>,
    tags: Option<Vec<Tag>>,
    rating: Option<String>,
    minrating: Option<String>,
    complexform: Option<String>,
    avail: Option<String>,
    cost: Option<String>,
    #[serde(skip)]
    bonus: Option<String>,
    forbidden: Option<String>,
    #[serde(skip)]
    required: Option<String>,
    source: String,
    page: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Tag {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "$value")]
enum Category {
    Autosofts,
    #[serde(rename = "Advanced Programs")]
    AdvancedPrograms,
    #[serde(rename = "Common Programs")]
    CommonPrograms,
    #[serde(rename = "Hacking Programs")]
    HackingPrograms,
    Software,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Categories {
    #[serde(rename = "category")]
    categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Programs {
    #[serde(rename = "program")]
    pub programs: Vec<Program>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "chummer")]
pub struct ChummerProgram {
    categories: Categories,
    pub programs: Programs,
}

use std::fs;
use std::io;
impl Program {
    pub fn init_program_list(base_path: &String) -> Result<Vec<Program>, String> {
        let filepath = format!("{}/programs.xml", base_path);
        let raw_file = fs::File::open(filepath).expect("Could not open the right file");
        let file_reader = io::BufReader::new(raw_file);

        let container: ChummerProgram = serde_xml_rs::from_reader(file_reader).unwrap();

        Ok(container.programs.programs)
    }
}
