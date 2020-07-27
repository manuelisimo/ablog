table! {
    image (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
        created_at -> Timestamp,
    }
}

table! {
    post (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    image,
    post,
);
