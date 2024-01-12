// @generated automatically by Diesel CLI.

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
