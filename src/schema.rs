table! {
    image (id) {
        id -> Integer,
        file_name -> Nullable<Text>,
        web_name -> Text,
        path -> Text,
        created_at -> Timestamp,
    }
}

table! {
    post (id) {
        id -> Integer,
        title -> Text,
        intro -> Text,
        web_name -> Text,
        banner -> Nullable<Integer>,
        body -> Text,
        created_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
        published -> Bool,
    }
}

joinable!(post -> image (banner));

allow_tables_to_appear_in_same_query!(
    image,
    post,
);
