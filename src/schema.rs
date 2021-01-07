use std::iter::Iterator;

table! {
    chatrooms (chatroom_id) {
        chatroom_id -> Int4,
        chatroom_name -> Varchar,
    }
}

table! {
    messages (id) {
        id -> Int4,
        body -> Varchar,
        chatroom_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    chatrooms,
    messages,
);
