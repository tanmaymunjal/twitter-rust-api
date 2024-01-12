use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub user_email: String,
    pub title: String,
    pub body: String,
    pub created_time: NaiveDateTime,
    pub published_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost<'a> {
    pub user_email: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub created_time: NaiveDateTime,
    pub published_at: Option<NaiveDateTime>,
}
