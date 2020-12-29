use crate::schema::chatroom;
use crate::schema::message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[table_name="chatroom"]
pub struct NewChatroom {
    pub chatroom_id: i32,
    pub chatroom_name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[table_name="message"]
pub struct NewMessage {
    pub id: i32,
    pub body: String,
    pub chatroom_id: i32,
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Chatroom {
    pub chatroom_id: i32,
    pub chatroom_name: String,
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Message {
    pub id: i32,
    pub body: String,
    pub chatroom_id: i32,
}
