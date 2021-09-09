#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::from_reader;

#[derive(Debug, Deserialize)]
struct Item {
    pub name: String,
    pub source: String
}

#[derive(Debug, Deserialize)]
struct Project {
    pub name: String,

    #[serde(rename = "Item", default)]
    pub items: Vec<Item>
}

fn main() {
    let correct = r##"
        <Project name="my_project">
            <Item name="hello" source="world.rs" />
        </Project>
    "##;
    let project: Project = from_reader(correct.as_bytes()).unwrap();
    println!("{:#?}", project);

    let malformed = r##"
        <Project name="malformed">
            <malformed name="Hello" source="world.rs />
            <WeDontClose This>
        </Project>
    "##;
    let messedup: Project = match from_reader(malformed.as_bytes())
    {
        Ok(v) => v,
        Err(e) => println!("Error reading malformed xml {:?}", e),
    };

    println!("{:#?}", messedup);
}
