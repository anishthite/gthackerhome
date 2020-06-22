use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde::{ Serialize, Deserialize };
use diesel::{ Queryable, Insertable, AsChangeset};

use crate::schema::users;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable,AsChangeset, Debug, PartialEq, Eq)]
pub struct User {
    pub email: String, 
    pub username: String,
    pub password: String,
    pub about: Option<String>,
    pub admin: Option<i32>
}

impl User{
    pub fn create(user: User, connection: &MysqlConnection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");

        users::table.order(users::username.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<User> {
        users::table.order(users::username.asc()).load::<User>(connection).unwrap()
    }

    pub fn update(username: String, user: User, connection: &MysqlConnection) -> bool {
        diesel::update(users::table.find(username)).set(&user).execute(connection).is_ok()
    }

}
