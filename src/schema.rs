table! {
    chatrooms (chatroom_id) {
        chatroom_id -> Int4,
        chatroom_name -> Varchar,
    }
}

table! {
    messages (id) {
        id -> Int4,
        creation -> Timestamp,
        body -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    chatrooms,
    messages,
);
