table! {
    nodes (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        content -> Nullable<Text>,
        node_id -> Nullable<Int4>,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    trees (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        node_id -> Int4,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    user_trees (id) {
        id -> Int4,
        user_id -> Int4,
        tree_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        nickname -> Nullable<Varchar>,
        email -> Varchar,
        created -> Timestamp,
        last_login -> Nullable<Timestamp>,
    }
}

joinable!(trees -> nodes (node_id));
joinable!(user_trees -> trees (tree_id));
joinable!(user_trees -> users (user_id));

allow_tables_to_appear_in_same_query!(
    nodes,
    trees,
    user_trees,
    users,
);
