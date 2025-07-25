use std::collections::HashMap;
use colored::*;
use prettytable::{Table, Row, Cell};
use dialoguer::Input;

#[derive(Debug)]
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
