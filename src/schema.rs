table! {
    server_snapshots (id) {
        id -> Integer,
        server -> Text,
        players -> Integer,
        time -> Timestamp,
    }
}

table! {
    servers (id) {
        id -> Text,
        name -> Text,
        address -> Text,
        rank -> Integer,
        record -> Integer,
        versions -> Text,
    }
}

joinable!(server_snapshots -> servers (server));

allow_tables_to_appear_in_same_query!(
    server_snapshots,
    servers,
);
