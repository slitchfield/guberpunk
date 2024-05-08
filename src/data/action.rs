use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Action {
    id: String,
    name: String,
    #[serde(rename = "type")]
    actiontype: String,
    test: Option<Test>,
    initiativecost: Option<String>,
    specname: Option<String>,
    boosts: Option<Vec<Boost>>,
    source: String,
    page: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Test {
    dice: String,
    limit: Option<String>,
    bonusstring: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Boost {
    name: Option<String>,
    duration: Option<String>,
    dicebonus: Option<String>,
    addlimit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Actions {
    #[serde(rename = "action")]
    pub actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "chummer")]
pub struct ChummerAction {
    version: String,
    actions: Actions,
}

use std::{fs, io};
impl Action {
    pub fn init_action_list(base_path: &String) -> Result<Vec<Action>, String> {
        let filepath = format!("{}/actions.xml", base_path);
        let raw_file = fs::File::open(filepath).expect("Could not open the right file");
        let file_reader = io::BufReader::new(raw_file);

        let container: ChummerAction = serde_xml_rs::from_reader(file_reader).unwrap();

        Ok(container.actions.actions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_test() {
        let action: Action = serde_xml_rs::from_str(
            r##"<action>
                <id>c649963a-b88f-473f-96d0-2f38cd054d86</id>
                <name>Melee Defense</name>
                <type>No</type>
                <test>
                    <dice>{REA} + {INT} + {Improvement Value: Dodge}</dice>
                </test>
                <source>SR5</source>
                <page>168</page>
            </action>"##,
        )
        .unwrap();
        println!("{:#?}", action);
    }

    #[test]
    fn context_test() {
        let document: ChummerAction = serde_xml_rs::from_str(r##"<chummer xmlns="" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.w3.org/2001/XMLSchema actions.xsd">
        <version>0</version>
        <actions>
        <!-- Region Core Rulebook -->
        <!-- Region Basic Actions  -->
          <action>
            <id>c649963a-b88f-473f-96d0-2f38cd054d86</id>
            <name>Melee Defense</name>
            <type>No</type>
            <test>
              <dice>{REA} + {INT} + {Improvement Value: Dodge}</dice>
            </test>
            <source>SR5</source>
            <page>168</page>
          </action>
        </actions>
        </chummer>""##)
            .unwrap();
        println!("{:#?}", document);
    }

    #[test]
    #[cfg_attr(feature = "ci_test_env", ignore)]
    fn file_read_test() {
        let action_list = Action::init_action_list(&String::from("./chummer_data/"))
            .expect("Could not parse action list from actions.xml");
        println!("Imported {} actions", action_list.len());
    }
}
