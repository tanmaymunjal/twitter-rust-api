use crate::models::post::NewPost;
use crate::schema::posts::dsl::posts;
use crate::{AppState, Arc};
use axum::{
    extract::{Json, State},
    response,
};
use chrono::Utc;
use diesel::RunQueryDsl;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct CreatePostPayload {
    pub id: i32,
    pub user_email: String,
    pub title: String,
    pub body: String,
    pub publish_now: bool,
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(create_post): Json<CreatePostPayload>,
) -> Json<Value> {
    let current_time = Utc::now().naive_utc();
    let mut new_post = NewPost {
        user_email: &create_post.user_email,
        title: &create_post.title,
        body: &create_post.body,
        created_time: current_time,
        published_at: None,
    };

    if create_post.publish_now {
        new_post.published_at = Some(current_time);
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(
            &mut state
                .db_pool
                .get()
                .expect("Failed to access db pool worker"),
        )
        .expect("Error saving new post");

    response::Json(json!({
        "updated":true
    }))
}
