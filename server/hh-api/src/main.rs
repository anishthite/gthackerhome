#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;



extern crate r2d2;
extern crate r2d2_diesel;
extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{ Serialize, Deserialize };
use diesel::{ QueryId, Queryable, Insertable, AsChangeset};
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::{ Status, Cookie };
use rocket::Response;
use rocket::response::status;
mod users;
mod schema;
mod db;
use users::{User};




#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]   
pub struct LoginForm {
    pub username: String,
    pub password: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCookie {
    pub username: String,
    pub display: Option<String>,
}




#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

//Create user, 
#[post("/create", data = "<user>")]
fn create(user: Json<User>, connection: db::Connection) -> Status {
    
    let mut insert = User {admin : Some(0), ..user.into_inner()};
    
    let hash_result  = hash(insert.password, 4);

    match hash_result {
        Ok(x) => insert.password = x,
        Err(_e) => return Status::NotAcceptable,
    }

    let curr_names = User::read(&connection);

    for name in curr_names.iter(){
        if name.username == insert.username {
            return Status::Conflict ;      
        }
    }
    User::create(insert, &connection);
    return Status::Created;
}

#[post("/login", data = "<form>")]
fn login(form: Json<LoginForm>, connection: db::Connection) -> Response<'static> {
    let my_user = LoginForm{..form.into_inner()};
    let my_user_copy = my_user.username.clone();
    let result = User::read_single(my_user.username, &connection);
    let mut response = Response::new();
    let mybool;
    match result {
        Ok(x) =>  mybool = verify(my_user.password,&x.password),
        Err(_e) => {response.set_raw_status(699, "Tripped a Wire"); return response;}
    }
    match mybool {
        Ok(x) => {
                    if x == true {  
                        let cookie = Cookie::build("username", my_user_copy.clone())
                            //.domain("www.rust-lang.org")
                            //.path("/")
                            .secure(true)
                            .http_only(true)
                            .finish();
                            response.set_header(cookie);
                            let mycookie = Cookie::build("username", my_user_copy).secure(true).finish();
                            response.adjoin_header(mycookie);
                            return response;
                    }
                    else {
                        response.set_raw_status(699, "Tripped a Wire"); return response;
                    }
                }
        Err(_e) => {response.set_raw_status(699, "Tripped a Wire"); return response;}
 
    }

}





//read user
#[get("/<username>")]
fn view(username: String) -> Response<'static> {
    format!("username: {}", username);

    let mut response = Response::new();
    //response.set_header(Cookie::new("hello", "world!"));
    
    let cookie = Cookie::build("name", "value")
    .domain("www.rust-lang.org")
    //.path("/")
    .secure(true)
    .http_only(true)
    .finish();
    response.set_header(cookie);
    return response
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/testy", routes![hello])
        .mount("/user", routes![view])
        .mount("/user_api", routes![create, login])
        .launch();
}
