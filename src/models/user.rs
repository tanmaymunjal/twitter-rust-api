use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_email: String,
    pub user_hashed_password: String,
    pub user_name: String,
    pub user_profile_pic: Option<Vec<u8>>,
    pub user_twitter_banner: Option<Vec<u8>>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub user_email: &'a str,
    pub user_hashed_password: &'a str,
    pub user_name: &'a str,
    pub user_profile_pic: Option<Vec<u8>>,
    pub user_twitter_banner: Option<Vec<u8>>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
