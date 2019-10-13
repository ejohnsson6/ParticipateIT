#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
use serde_derive::{Serialize,Deserialize};
use pdf_form::Form;
use std::path::Path;


#[derive(Serialize, Deserialize, Debug)]
pub struct PartList {
    arrangement: String,
    date: String,
    part_list: Vec<Participant>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Participant {
    name: String,
    member: String,
}

#[post("/jsontopdf", data = "<part_list>")]
fn part_list(part_list: Json<PartList>) -> String {
    let json_data = part_list.into_inner();
    
    println!("Received List with {0} items", json_data.part_list.len());
    let form = Form::load(Path::new("./resources/testa.pdf")).unwrap();
    println!("{:?}", form.get_all_types());

   return String::from("Test");
}

fn generate_fdf(_part_list: PartList) -> String {

    let 


    
}

fn main() {
    rocket::ignite().mount("/", routes![part_list]).launch();
}