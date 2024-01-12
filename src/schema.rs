// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 320]
        user_email -> Varchar,
        #[max_length = 70]
        title -> Varchar,
        #[max_length = 280]
        body -> Varchar,
        created_time -> Timestamptz,
        published_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (user_email) {
        #[max_length = 320]
        user_email -> Varchar,
        #[max_length = 64]
        user_hashed_password -> Bpchar,
        #[max_length = 100]
        user_name -> Varchar,
        user_profile_pic -> Nullable<Bytea>,
        user_twitter_banner -> Nullable<Bytea>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(posts -> users (user_email));

diesel::allow_tables_to_appear_in_same_query!(posts, users,);
