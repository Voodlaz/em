table! {
    chatroom (chatroom_id) {
        chatroom_id -> Int4,
        chatroom_name -> Varchar,
    }
}

table! {
    message (id) {
        id -> Int4,
        body -> Varchar,
        chatroom_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    chatroom,
    message,
);
