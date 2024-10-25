use crate::models::receipt::{MessageData, MessageReceipt};

impl MessageReceipt {
    pub fn mentions_me(&self) -> bool {
        match &self.body.message_data {
            Some(MessageData::ExtendedTextMessage {
                extended_text_message_data,
            }) => {
                let number = self
                    .body
                    .instance_data
                    .wid
                    .strip_suffix("@c.us")
                    .expect("wid was expected to contain @c.us suffix");

                extended_text_message_data.text.contains(number)
            }
            _ => false,
        }
    }
}
