table! {
    book (isbn) {
        isbn -> Text,
        title -> Text,
        author -> Text,
        thumbnail -> Nullable<Text>,
        thumbnail_small -> Nullable<Text>,
    }
}

table! {
    owned_books (isbn, user_id) {
        isbn -> Text,
        user_id -> Integer,
    }
}

table! {
    user (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

table! {
    user_sessions (session_id) {
        session_id -> Text,
        user_id -> Integer,
    }
}

joinable!(owned_books -> book (isbn));
joinable!(owned_books -> user (user_id));
joinable!(user_sessions -> user (user_id));

allow_tables_to_appear_in_same_query!(book, owned_books, user, user_sessions,);
