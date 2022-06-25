use contact_manager::{Contacts, add_contacts, edit_contacts, to_hash, write_file, view_contact};
use std::{collections::HashMap, fs, env};

fn main() {
    let content = fs::read_to_string("p2_data.csv").expect("Some thing wrong with file");
    let vector_content: Vec<&str> = content.split("\n").collect();
    let mut cont = to_hash(vector_content);
    // println!("{}", cont.len());


    add_contacts(&mut cont, Some("Andrii Anto".to_owned()), Some("Andr@gmail.com".to_owned()));
    edit_contacts(&mut cont, 500, Some("Jack".to_owned()), None);
    view_contact(cont, 501);
    // write_file(cont, "new1.txt");

    // let args: Vec<String> = env::args().collect(); 
    // for i in args {
    //     println!("{i}");

    // }
 
    
}
