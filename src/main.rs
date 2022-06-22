use std::{fs, collections::HashMap};
#[derive(Default, Debug)]
struct Contacts {
    id: Option<u32>,
    name: Option<String>,
    email: Option<String>
}

impl Contacts {
    fn from_vec(raw: &str) -> Self {
        let x:Vec<&str> = raw.split(",").collect();
        let mut y: Contacts; 
        if x.len() == 3 {
                y = Contacts {
                id: Some(x[0].parse().unwrap_or_default()),
                name: Some(x[1].to_owned()),
                email: Some(x[2].to_owned())
            }; 
        } else {
            y = Contacts::default(); 

        }
        y   
        
    }
}




fn main() {
    let content = fs::read_to_string("p2_data.csv").expect("Some thing wrong with file");
    let vector_content:Vec<&str> = content.split("\n").collect();

    println!("{}",vector_content.len());
    let mut cont: Vec<Contacts> = Vec::new();
    let mut hash_cont: HashMap<String, Contacts> = HashMap::new();


    for i in vector_content {
        if i != "" {
            cont.push(Contacts::from_vec(i));
        }
        
    }
    for i in cont {
        println!("{:?}", &i)
    }

    

    




    
}
