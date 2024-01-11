use diesel::prelude::*;

// `Path` gives you the path parameters and deserializes them.
async fn path(Path(user_id): Path<u32>) {}
