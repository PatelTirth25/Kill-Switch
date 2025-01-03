use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::rooms)]
#[derive(Debug, Serialize)]
pub struct Room {
    pub id: i32,
    pub room: String,
    pub url: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::rooms)]
pub struct NewRoom<'a> {
    pub room: &'a str,
    pub url: &'a str,
}
