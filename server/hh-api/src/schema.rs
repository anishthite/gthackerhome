table! {
    invite_tokens (token) {
        token -> Varchar,
        creator -> Varchar,
    }
}

table! {
    items (id) {
        id -> Varchar,
        author -> Varchar,
        time -> Bigint,
        itemtype -> Varchar,
        title -> Nullable<Text>,
        url -> Nullable<Text>,
        text -> Nullable<Text>,
        parentid -> Nullable<Varchar>,
        descendents -> Nullable<Integer>,
        score -> Nullable<Integer>,
    }
}

table! {
    items_relationships (parent) {
        parent -> Varchar,
        child -> Varchar,
    }
}

table! {
    users (username) {
        email -> Text,
        username -> Varchar,
        password -> Text,
        about -> Nullable<Text>,
        admin -> Nullable<Integer>,
        timecreated -> Bigint,
        parent -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    invite_tokens,
    items,
    items_relationships,
    users,
);
