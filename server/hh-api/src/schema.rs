table! {
    users (username) {
        email -> Text,
        username -> Varchar,
        password -> Text,
        about -> Nullable<Text>,
        admin -> Nullable<Integer>,
    }
}
