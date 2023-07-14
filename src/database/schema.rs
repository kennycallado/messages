// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        title -> Varchar,
        body -> Varchar,
        message_type -> Varchar,
        content -> Array<Nullable<Text>>,
        data -> Jsonb,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int4,
        user_id -> Int4,
        fcm_token -> Nullable<Varchar>,
        web_token -> Nullable<Jsonb>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    tokens,
);
