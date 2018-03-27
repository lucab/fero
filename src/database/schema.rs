/// Generated with `diesel print-schema`

table! {
    secrets (id) {
        id -> Integer,
        key_id -> BigInt,
        threshold -> Integer,
    }
}

table! {
    user_secret_weights (id) {
        id -> Integer,
        secret_id -> Integer,
        user_id -> Integer,
        weight -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        key_id -> BigInt,
    }
}

joinable!(user_secret_weights -> secrets (secret_id));
joinable!(user_secret_weights -> users (user_id));

//allow_tables_to_appear_in_same_query!(
//    secrets,
//    user_secret_weights,
//    users,
//);