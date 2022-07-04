table! {
    logins (id) {
        id -> Integer,
        username -> Text,
        token -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        hashed_password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(logins, users,);
