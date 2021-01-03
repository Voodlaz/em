use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::RunQueryDsl;
use dotenv::dotenv;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use crate::models::{
    Chatroom, NewChatroom,
    Message, NewMessage
};

pub fn create_chatroom<'a>(conn: &PgConnection, chat_name: String) -> Chatroom {
    use crate::schema::chatrooms::dsl::*;

    let new_chatroom = NewChatroom {
        chatroom_name: chat_name
    };

    diesel::insert_into(chatrooms)
        .values(&new_chatroom)
        .get_result(conn)
        .expect("Chatroom fail")
}

/*pub fn create_message<'a>(conn: &PgConnection, text: String, chat_id: i32) -> SQLMessage {
    use crate::schema::messages::dsl::*;

    let new_message = NewMessage {
        body: text,
        chatroom_id: chat_id
    };

    diesel::insert_into(messages)
        .values(&new_message)
        .get_result(conn)
        .expect("Message fail")
}*/
