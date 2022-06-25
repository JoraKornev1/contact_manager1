use std::{collections::HashMap, fs, env};

#[derive(Default, Debug)]
pub struct Contacts {
    id: Option<usize>,
    name: Option<String>,
    email: Option<String>,
}

pub fn to_hash(vector_content: Vec<&str>) -> HashMap<usize, Contacts> {
    let mut cont = HashMap::new();
    let mut id = 1;
    for i in vector_content {
            if i.contains(" ") {
            let x: Vec<&str> = i.split(",").collect();
            let y: Contacts;
            if x.len() == 3 {
                y = Contacts {
                    id: Some(id),
                    name: Some(x[1].to_owned()),
                    email: Some(x[2].to_owned()),
                };
                
            } else {
                y = Contacts::default();
            }
            cont.insert(id, y);
            id += 1;
            
        }
    }
    cont
}

pub fn add_contacts(cont: &mut HashMap<usize, Contacts>, name: Option<String>, email: Option<String>) {
    let key = cont.len() + 1; 
    let new_contact = Contacts {
        id: Some(key),
        name,
        email
    };
    cont.insert(key, new_contact);
    
}

pub fn edit_contacts(cont: &mut HashMap<usize, Contacts>, id: usize, name: Option<String>, email: Option<String>) {
    let key = id;
    if let Some(x) = cont.get_mut(&id) {
        *x = Contacts {
            id: Some(id),
            name,
            email
        }
    }   
}

pub fn view_contact (cont: HashMap<usize, Contacts>, id: usize) {
    println!("{:?}", cont.get(&id))
}


pub fn write_file(cont: HashMap<usize, Contacts>, file_name: &str) {
    let mut contacts_to_file = String::from("id,name,email\n");
    let mut key = 1;
    let x: &String = &"".to_owned();

    while cont.contains_key(&key) {
        let i = cont.get(&key).unwrap_or(&Contacts{ id: None, name: None, email: None});
        let id = i.id.unwrap_or(0).to_string();
        let name = i.name.as_ref().unwrap_or(x);
        let email = i.email.as_ref().unwrap_or(x);

        let line = format!("{},{},{}\n", id, name, email);
        contacts_to_file.push_str(&line);
        key += 1;

    }
    fs::write(file_name, contacts_to_file).expect("Something go wrong");
}