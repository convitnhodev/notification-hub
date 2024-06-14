table! {
    notifications (id) {
        id -> Varchar,
        from_user -> Varchar,
        to_user -> Varchar,
        content -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        viewed_at -> Nullable<Timestamp>,
    }
}