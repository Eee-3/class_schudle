// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id) {
        id -> Text,
        name -> Text,
        teacher -> Nullable<Text>,
        location -> Nullable<Text>,
        weekday -> Integer,
        start_time -> Text,
        end_time -> Text,
        weeks -> Text,
        color -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
