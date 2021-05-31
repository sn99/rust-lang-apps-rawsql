#[macro_use]
extern crate diesel;
mod model;
mod schema;
mod structs;


fn main() {
    let r = model::list_users();
    println!("{:#?}", r)
}
