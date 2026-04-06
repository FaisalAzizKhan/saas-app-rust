// @generated automatically by Diesel CLI.

diesel::table! {
    users (users_id) {
        users_id -> Uuid,
        name -> Text,
        email -> Text,
        password -> Text,
        is_admin -> Bool,
        is_deleted -> Bool,
    }
}
