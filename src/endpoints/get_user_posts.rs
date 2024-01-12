use crate::models::post::Post;
use crate::schema::posts::{dsl::posts, user_email};
use crate::{AppState, Arc};
use axum::{
    extract::{Path, State},
    response::Json,
};
use diesel::prelude::*;
use serde_json::{to_string, Value};

pub async fn get_user_posts(
    Path(user_query_email): Path<String>,
    Path(page_no): Path<i64>,
    Path(page_size): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    let result = posts
        .filter(user_email.eq(&user_query_email))
        .limit(page_size)
        .offset(page_no)
        .select(Post::as_select())
        .load(
            &mut state
                .db_pool
                .get()
                .expect("Failed to access db pool worker"),
        )
        .expect("Error finding posts");

    Json(Value::String(
        to_string(&result).expect("Error serializing data into json"),
    ))
}
