#![feature(proc_macro_hygiene, decl_macro)]



#[macro_use] extern crate rocket;

use mysql::*;
use mysql::prelude::*;
mod users;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}



fn main() -> Result<(), >{

    let url = "mysql://root:password@localhost:3306/users";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    let users = vec![
        users::User{hash : 1, email: String::from("anishthite@gatech.edu"), username: String::from("aquajet"), password: String::from("dnec"), admin : true},
        users::User{hash : 2, email: String::from("anishthite@gmail.com"), username: String::from("aquajet2"), password: String::from("dnec"), admin : false},
    
    ];

// Now let's insert payments to the database
    conn.exec_batch(
        r"INSERT INTO payment (customer_id, amount, account_name)
        VALUES (:customer_id, :amount, :account_name)",
        users.iter().map(|&u| params! {
            "hash" => &u.hash,
            "email" => &u.email,
            "username" => &u.username,
            "password" => &u.password,
            "admin" => &u.admin
        })
    )?;

// Let's select payments from database. Type inference should do the trick here.
    let selected_payments = conn
        .query_map(
            "SELECT hash, email, username, password, admin from payment",
            |(hash, email, username, password, admin)| {
                users::User{ hash, email, username, password, admin}
            },
        )?;

// Let's make sure, that `payments` equals to `selected_payments`.
// Mysql gives no guaranties on order of returned rows
// without `ORDER BY`, so assume we are lucky.
    println!("{:?}", selected_payments);
    println!("Yay!");

    Ok(())
}





    //rocket::ignite().mount("/", routes![index]).launch();
