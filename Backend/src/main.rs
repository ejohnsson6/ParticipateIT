#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod fdf_converter;

use rocket_contrib::json::Json;
use serde_derive::{Serialize,Deserialize};
use std::fs;
use fdf_converter::FdfConverter;
use encoding::{Encoding, EncoderTrap};
use encoding::all::ISO_8859_1;
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

    let mut fdf_data = FdfConverter::new()
    .add_data("namn1", json_data.date.as_str())
    .add_data("namn2", json_data.arrangement.as_str())
    .add_data("namn3", json_data.part_list.len().to_string().as_str());

    for x in 0..(json_data.part_list.len()) {
        let mut field = String::from("art");
        field.push_str((x + 1).to_string().as_str());
        fdf_data = fdf_data.add_data(field.as_str(), json_data.part_list[x].name.as_str())
    }

    let data_string = fdf_data.finish();

    let bytes = ISO_8859_1.encode(data_string.as_str() ,EncoderTrap::Replace).unwrap();
    
    fs::write("fdfdata.fdf", bytes);



   return String::from("Test");
}
fn main() {
    rocket::ignite().mount("/", routes![part_list]).launch();
}