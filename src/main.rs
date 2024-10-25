mod client;
mod models;
mod utils;

use anyhow::Result;
use models::SendMessageRequest;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if let Err(e) = start().await {
        eprintln!("Error: {e}");
    }
}

async fn start() -> Result<()> {
    println!("Started processing...");

    loop {
        if !process_message().await? {
            println!("No messages found...");
        }
    }
}

async fn process_message() -> Result<bool> {
    let client = client::GreenApiClient::new();

    let message = match client.get_notification().await? {
        Some(m) => m,
        None => return Ok(false),
    };

    println!("Sender: {}", message.body.sender_data.sender_name);
    println!("Message: {:?}", message.body.message_data);

    if message.mentions_me() {
        let msg = SendMessageRequest {
            chat_id: message.body.sender_data.chat_id,
            quoted_message_id: message.body.id_message,
            message: "Fala que eu te esgurmo!".to_string(),
        };

        client.quote_answer_message(msg).await?;
    }

    let deletion = client.delete_notification(message.receipt_id).await?;

    if deletion.result {
        println!("Message {} deleted from queue", message.receipt_id);
    } else {
        println!("Error deleting message: {}", message.receipt_id)
    }

    Ok(true)
}
