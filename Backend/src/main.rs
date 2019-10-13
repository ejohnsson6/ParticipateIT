#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod fdf_converter;

use rocket_contrib::json::Json;
use serde_derive::{Serialize,Deserialize};
// use FdfConverter::FdfConverter;


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

   return String::from("Test");
}
fn main() {
    let _fdf_con = fdf_converter::FdfConverter::new().add_data("content1", "heh");

    rocket::ignite().mount("/", routes![part_list]).launch();
}