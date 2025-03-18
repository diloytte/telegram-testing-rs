mod get_dialogs; // Import the get_dialogs module

use dotenv::dotenv;
use get_dialogs::get_all_chats;
use grammers_client::{Client, Config, SignInError};
use grammers_session::Session;
use std::env;
use tokio::fs;

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

    let carranza = &(client.resolve_username("diloytte").await?.unwrap());

    let me_downloadable_photo = &(carranza.photo_downloadable(false).unwrap());

    client.download_media(me_downloadable_photo,"./photos/mephoto.jpg" ).await?;

    dbg!(me_downloadable_photo);

    dbg!(Some(carranza));

    get_all_chats(&client).await?;

    Ok(())
}
