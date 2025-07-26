use std::collections::HashMap;
use colored::*;
use prettytable::{Table, Row, Cell};
use dialoguer::Input;
use std::process::Command;

fn clear_screen() {
        Command::new("clear").status().unwrap();
}

mod contact;


fn main() {
    let mut contacts: HashMap<String, contact::Contact> = HashMap::new();

    match contact::load_contacts(&"ContactBook.json") {
        Ok(c) => {contacts = c;},
        Err(e) => {
            eprintln!("Error to load : {}", e);
        },
    }

    loop {
        println!("{}","Choose an option".red());
        println!("{}","1 .Print all exist contacts".blue());
        println!("{}","2 .Add contact".blue());
        println!("{}","3 .Remove contact".blue());
        println!("{}","4 .Exit.".blue());

        let Choice: String = Input::new()
            .with_prompt("Enter Choice: ")
            .interact_text()
            .unwrap();
        clear_screen();
        match Choice.trim() {
            "1" => {
                contact::print_contacts(&contacts);
            },
            "2" => {
                let name: String = Input::new()
                    .with_prompt("Name")
                    .interact_text()
                    .unwrap();

                let phone_num: String = Input::new()
                    .with_prompt("Phone Number")
                    .interact_text()
                    .unwrap();                    
                let mut cntct = contact::Contact {                             phone_numbers: Vec::new(),
                };
                cntct.phone_numbers.push(phone_num.to_string());
                contact::add_contact(&mut contacts, &name, cntct);
            },
            "3" => {
                let name: String = Input::new()
                    .with_prompt("Name")
                    .interact_text()
                    .unwrap();
                contact::rem_contact(&mut contacts, name);
            },
            "4" => break,
            _ => {println!("{}","Invalid Choice".red()); },
        }
        let err = contact::save_contacts(&"ContactBook.json", &contacts);
        // println!("save");
    }
}
