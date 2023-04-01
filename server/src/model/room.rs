use crate::model::message::Message;
use crate::model::user::User;

#[derive(Default)]
pub struct Room {
    id: Uuid,
    messages: Vec<Message>,
    users: Vec<User>,
}

impl Room {
    pub fn new(id: Uuid) -> Self {
        Room {
            id,
            ..Default::default()
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.messages.sort_by_key(|message| message.created_at)
    }

    pub fn messages_iter(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn user_iter(&self) -> impl Iterator<Item = &User> {
        self.users.iter()
    }
}