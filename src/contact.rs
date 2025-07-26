use std::collections::HashMap;
use prettytable::{Table, Row, Cell};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io;
use std::io::BufReader;


#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub phone_numbers: Vec<String>,

}


pub fn add_contact(contacts: &mut HashMap<String, Contact>, name: &str, cntct: Contact) {
    contacts.insert(name.to_string(), cntct);
}

pub fn rem_contact(contacts: &mut HashMap<String, Contact>, name: String) {
    contacts.remove(&name);
}

pub fn print_contacts(contacts: &HashMap<String, Contact>) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Number(s)"),
    ]));

    for (key, value) in contacts.iter() {
        let numbers_str = value.phone_numbers.join(", ");
        table.add_row(Row::new(vec![
            Cell::new(key),
            Cell::new(&numbers_str),
                                                                    ]));
    }

    table.printstd();

    // println!("{}", "Raw Debug View:".bright_black());
    // println!("{:#?}", contacts);
}

pub fn save_contacts(path: &str, contacts: &HashMap<String, Contact>) -> io::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, &contacts)?;
    Ok(())
}

pub fn load_contacts(path: &str) -> io::Result<HashMap<String, Contact>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let contacts: HashMap<String, Contact> = serde_json::from_reader(reader)?;
    Ok(contacts)
}
