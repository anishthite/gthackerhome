#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;


use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;

mod users;
mod schema;
mod db;
use users::{User};






#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

//Create user, 
#[post("/create", data = "<user>")]
fn create(user: Json<User>, connection: db::Connection) -> Status {
    
    let insert = User {admin : Some(0), ..user.into_inner()};
     
    let curr_names = User::read(&connection);

    for name in curr_names.iter(){
        if name.username == insert.username {
            return Status::Conflict ;      
        }
    }
    User::create(insert, &connection);
    return Status::Created;
}


//read user
#[get("/<username>")]
fn view(username: String) -> String {
    format!("username: {}", username)
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/testy", routes![hello])
        .mount("/user", routes![view])
        .mount("/user_api", routes![create])
        .launch();
}
