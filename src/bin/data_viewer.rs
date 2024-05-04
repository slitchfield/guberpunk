
use core::data;

use core::data::program::{ChummerProgram, Program};
use std::fs;
use std::io;

use xml::reader::{EventReader, XmlEvent};

fn main() -> () {
    println!("Identified Source files:");

    let paths = fs::read_dir("./chummer_data").unwrap();

    for path in paths {
        println!("\t{}", path.unwrap().path().display())
    }

    println!("Reading through programs.xml");

    /* 
    let program: Program = serde_xml_rs::from_str(r##"<program>
      <id>bed1f6eb-ead9-49aa-bb24-266c21d7aeb4</id>
      <name>Browse</name>
      <category>Common Programs</category>
      <avail>0</avail>
      <cost>80</cost>
      <source>SR5</source>
      <page>245</page>
    </program>"##).unwrap();
    println!("{:#?}", program);

    let document: ChummerProgram = serde_xml_rs::from_str(r##"<chummer xmlns="" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
     xsi:schemaLocation="http://www.w3.org/2001/XMLSchema programs.xsd">

  <categories>
    <category>Autosofts</category>
    <category>Advanced Programs</category>
    <category>Common Programs</category>
    <category>Hacking Programs</category>
    <category>Software</category>
  </categories>
  <programs>
    <program>
    <id>bed1f6eb-ead9-49aa-bb24-266c21d7aeb4</id>
    <name>Browse</name>
    <category>Common Programs</category>
    <avail>0</avail>
    <cost>80</cost>
    <source>SR5</source>
    <page>245</page>
    </program>
  </programs>
  </chummer>"##).unwrap();
    println!("{:#?}", document);

    let raw_file = fs::File::open("./chummer_data/programs.xml").expect("Could not open the right file");
    let file_reader = io::BufReader::new(raw_file);

    let container: ChummerProgram = serde_xml_rs::from_reader(file_reader).unwrap();

    for program in container.programs.programs {
        println!("{:#?}", program);
    }
    */
    println!("Cleaner now");
    let program_list = data::program::Program::init_program_list(String::from("./chummer_data/")).expect("");
    for program in program_list {
        println!("{:#?}", program);
    }
    

}