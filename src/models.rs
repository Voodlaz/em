use super::schema::{chatrooms, messages};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="chatrooms"]
pub struct NewChatroom {
    pub chatroom_name: String
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Chatroom {
    pub chatroom_id: i32,
    pub chatroom_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="messages"]
pub struct NewMessage {
    pub body: String,
    pub chatroom_id: i32,
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Associations)]
//#[belongs_to(Chatroom)]
pub struct Message {
    pub id: i32,
    pub body: String,
    pub chatroom_id: i32,
}
