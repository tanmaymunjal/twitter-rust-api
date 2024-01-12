-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_email VARCHAR(320) NOT NULL,
    title VARCHAR(70) NOT NULL,
    body VARCHAR(280) NOT NULL,
    created_time timestamptz NOT NULL DEFAULT NOW(),
    published_at  timestamptz DEFAULT NULL,
    CONSTRAINT post_belong_to_user
      FOREIGN KEY(user_email) 
	  REFERENCES users(user_email)
	  ON DELETE CASCADE
)