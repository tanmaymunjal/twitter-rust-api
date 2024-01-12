use crate::models::user::NewUser;
use crate::schema::users::dsl::users;
use crate::utils::sha256_hash;
use crate::{AppState, Arc};
use axum::{
    extract::{Json, State},
    response,
};
use chrono::NaiveDateTime;
use diesel::RunQueryDsl;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub user_email: String,
    pub user_password: String,
    pub user_name: String,
    pub user_profile_pic: Option<Vec<u8>>,
    pub user_twitter_banner: Option<Vec<u8>>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(create_user): Json<CreateUserPayload>,
) -> Json<Value> {
    let user_hashed_password = &sha256_hash(&create_user.user_password);

    let new_user = NewUser {
        user_email: &create_user.user_email,
        user_hashed_password: user_hashed_password,
        user_name: &create_user.user_name,
        user_profile_pic: create_user.user_profile_pic,
        user_twitter_banner: create_user.user_twitter_banner,
        created_at: create_user.created_at,
        updated_at: create_user.updated_at
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(
            &mut state
                .db_pool
                .get()
                .expect("Failed to access db pool worker"),
        )
        .expect("Error saving new user");

    response::Json(json!({
     "updated":true
    }))
}
