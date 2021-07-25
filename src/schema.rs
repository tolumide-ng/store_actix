table! {
    user_info (id) {
        id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        email -> Varchar,
        hash -> Varchar,
        user_type -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    user_role (id) {
        id -> Uuid,
        auth_type -> Varchar,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(user_info -> user_role (user_type));

allow_tables_to_appear_in_same_query!(
    user_info,
    user_role,
);
