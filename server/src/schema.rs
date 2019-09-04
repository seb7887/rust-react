table! {
    books (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        author -> Varchar,
        cover -> Nullable<Varchar>,
        page_count -> Nullable<Int4>,
        publisher -> Nullable<Varchar>,
        synopsis -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        token -> Varchar,
    }
}

joinable!(books -> users (user_id));

allow_tables_to_appear_in_same_query!(
    books,
    users,
);
