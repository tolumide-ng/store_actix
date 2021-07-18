table! {
    user_info (id) {
        id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        email -> Varchar,
        hash -> Varchar,
        user_type -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_role (id) {
        id -> Uuid,
        auth_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        active -> Bool,
    }
}

joinable!(user_info -> user_role (user_type));

allow_tables_to_appear_in_same_query!(user_info, user_role,);
