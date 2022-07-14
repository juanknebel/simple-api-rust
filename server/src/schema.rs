table! {
    logins (id) {
        id -> Integer,
        username -> Text,
        token -> Text,
    }
}

table! {
    messages (id) {
        id -> Integer,
        from -> Integer,
        to -> Integer,
        message -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        hashed_password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(logins, messages, users,);
