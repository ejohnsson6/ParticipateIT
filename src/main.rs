#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
use serde_derive::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyType {
    test: String,
    num: i32
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> Json<MyType> {
    return Json(MyType{test: "asdas".to_string(), num: 123});
}

#[post("/user", data = "<new_user>")]
fn new_user(new_user: Json<MyType>) -> &'static str {
    println!("{:?}", new_user);
   return "Test";
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello, new_user]).launch();
}