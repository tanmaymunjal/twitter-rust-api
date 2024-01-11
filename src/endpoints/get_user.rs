use crate::models::user::User;
use crate::postgres_connect;
use crate::schema::users::{dsl::users, user_email};
use axum::{extract::{Path,State}, response::Json};
use diesel::prelude::*;
use serde_json::{json, Value};

pub async fn get_user(
    Path(user_query_email): Path<String>,
) -> Json<Value> {
    let connection = &mut postgres_connect::establish_connection();
    let result = users
        .filter(user_email.eq(&user_query_email))
        .limit(1)
        .select(User::as_select())
        .load(connection)
        .expect("Error finding user");
    let user = result.into_iter().nth(0).expect("No user found");
    Json(json!({
       "user_email": user.user_email,
       "user_name":user.user_name,
       "user_profile_pic":user.user_profile_pic,
       "user_twitter_banner":user.user_twitter_banner,
       "created_at":user.created_at,
       "updated_at":user.updated_at
    }))
}