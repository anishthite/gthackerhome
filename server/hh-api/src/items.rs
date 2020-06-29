use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde::{ Serialize, Deserialize };
use diesel::{ QueryId, Queryable, Insertable, AsChangeset};
use trees::{tr,Tree,Forest};
use crate::schema::{items, items_relationships};

#[table_name = "items"]
#[derive(Serialize, Deserialize, QueryId, Queryable, Insertable, AsChangeset, Debug, PartialEq, Eq)]
pub struct Item {
    pub id: String,
    pub author: String,
    pub time: i64,
    pub itemtype: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub text: Option<String>,
    pub parentid: Option<String>,
    pub descendents: Option<i32>,
    pub score: Option<i32>
}    
    
#[table_name = "items_relationships"]
#[derive(Serialize, Deserialize, QueryId, Queryable, Insertable,AsChangeset, Debug, PartialEq, Eq)]
pub struct ItemRelationship {
    pub parent: String,
    pub child: String
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ItemNode {
    pub item: Item,
    pub descendents: Option<Vec<ItemNode>>
}



impl Item{
    pub fn create(item: Item, connection: &MysqlConnection) -> Item {
        diesel::insert_into(items::table)
            .values(&item)
            .execute(connection)
            .expect("Error creating new item");


       if item.itemtype == "comment" && item.parentid.is_some() { 
            let pc = ItemRelationship{parent: item.parentid.unwrap(), child: item.id};   
            diesel::insert_into(items_relationships::table)
                .values(&pc)
                .execute(connection)
                .expect("Error adding item to pc table");
        }
       items::table.order(items::id.desc()).first(connection).unwrap()
    }
    //TODO: Maybe make this more efficent by only taking top x
    //
    //General TODO:
    //create post/comment
    //read: returns list of everything
    //read_posts: returns list of posts only in chron order
    //read_single: gets item data by id
    //render: given item id itll render everything below it
    //update post comment
    //delete post/comment
    pub fn read_posts(connection: &MysqlConnection) -> Vec<Item> {
        items::table.filter(items::itemtype.eq("post")).order(items::id.desc()).load::<Item>(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Item> {
        items::table.order(items::id.desc()).load::<Item>(connection).unwrap()
    }
    
    pub fn read_single(id: String, connection: &MysqlConnection) -> Result<Item, diesel::result::Error> {
        items::table.find(id).first(connection)
    }

    //pub fn render_single(rootitem: Item, connection: &MysqlConnection) -> ItemNode {
    //    let root_node = ItemNode {item: rootitem, descendents: Some(Vec::new())};  
    //}
//    pub fn update(id: String, user: User, connection: &MysqlConnection) -> bool {
//        diesel::update(users::table.find(username)).set(&user).execute(connection).is_ok()
//    }

}
