// @generated automatically by Diesel CLI.

diesel::table! {
    license_keys (id) {
        id -> Int4,
        key -> Text,
        expires -> Timestamp,
        is_activated -> Bool,
        banned -> Bool,
        hwid -> Nullable<Text>,
    }
}

diesel::table! {
    user_info (license_id) {
        license_id -> Int4,
        username -> Text,
        first_login -> Text,
        last_login -> Text,
        last_session_time -> Nullable<Timestamp>,
        last_ip -> Inet,
        os_name -> Nullable<Text>,
        os_version -> Nullable<Text>,
        cpu_info -> Nullable<Text>,
        cpu_cores -> Nullable<Int4>,
        created_at -> Timestamp,
        notes -> Nullable<Text>,
    }
}

diesel::joinable!(user_info -> license_keys (license_id));

diesel::allow_tables_to_appear_in_same_query!(
    license_keys,
    user_info,
);
