mod constants;
mod extract_token_address;
mod get_dialogs;
mod handle_incoming_message;
mod download_profile_photo;

use dotenv::dotenv;
use download_profile_photo::download_profile_photo;
use get_dialogs::get_all_chats;
use grammers_client::{Client, Config, SignInError, Update};
use grammers_session::Session;
use handle_incoming_message::handle_message;
use std::env;
use tokio::fs;
use tokio::task;

async fn start_message_listener(client: Client) -> tokio::task::JoinHandle<()> {
    task::spawn(async move {
        loop {
            match client.next_update().await {
                Ok(update) => {
                    if let Update::NewMessage(message) = update {
                        handle_message(&message);
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving update: {:?}", e);
                }
            }
        }
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_id: i32 = env::var("API_ID")?.parse()?;
    let api_hash: String = env::var("API_HASH")?;
    let phone_number: String = env::var("PHONE_NUMBER")?;

    let session_file = "session.session";
    let session = if let Ok(data) = fs::read(session_file).await {
        Session::load(&data)?
    } else {
        Session::new()
    };

    let client = Client::connect(Config {
        session,
        api_id,
        api_hash: api_hash.clone(),
        params: Default::default(),
    })
    .await?;

    if !client.is_authorized().await? {
        let token = client.request_login_code(&phone_number).await?;

        println!("Enter the OTP code:");
        let mut code = String::new();
        std::io::stdin().read_line(&mut code)?;
        let code = code.trim();

        match client.sign_in(&token, code).await {
            Ok(_) => println!("Logged in successfully!"),
            Err(SignInError::PasswordRequired(password_token)) => {
                let password = env::var("PASSWORD")?;
                client.check_password(password_token, password).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    let session_data = client.session().save();
    fs::write(session_file, session_data).await?;

    println!("Connected to Telegram!");

    if let Err(e) = download_profile_photo(&client, "diloytte", "./photos/taranda.jpg").await {
        eprintln!("Error downloading profile photo: {}", e);
    }
    // get_all_chats(&client).await?;

    let message_listener = start_message_listener(client).await;

    message_listener.await?;

    Ok(())
}
