table! {
    book (isbn) {
        isbn -> Text,
        title -> Text,
        author -> Text,
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
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
    }
}

joinable!(owned_books -> book (isbn));
joinable!(owned_books -> user (user_id));

allow_tables_to_appear_in_same_query!(
    book,
    owned_books,
    user,
);
