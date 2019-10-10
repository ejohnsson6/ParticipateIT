#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
use serde_derive::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct PartList {
    arrangement: String,
    date: String,
    part_list: Vec<Participant>
}

#[derive(Serialize, Deserialize, Debug)]
struct Participant {
    name: String,
    member: String
}

#[post("/user", data = "<new_user>")]
fn new_user(new_user: Json<PartList>) -> &'static str {
    println!("{:?}", new_user);
   return "Test";
}

fn main() {
    rocket::ignite().mount("/hello", routes![new_user]).launch();
}