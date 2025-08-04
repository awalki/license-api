// @generated automatically by Diesel CLI.

diesel::table! {
    license_keys (id) {
        id -> Int4,
        key -> Text,
        expires -> Timestamp,
        is_activated -> Bool,
        hwid -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}
