use grammers_client::Client;

pub async fn download_profile_photo(client: &Client, username: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Attempt to resolve the username
    let chat_option = client.resolve_username(username).await?;

    // If no chat is found, return an error
    if chat_option.is_none() {
        return Err(format!("Chat with username {} not found", username).into());
    }

    let chat = chat_option.unwrap();  // Safe to unwrap now since we've checked for None

    // Attempt to get the downloadable photo
    let chat_downloadable_photo = chat.photo_downloadable(false).unwrap();

    // Download the media (profile photo) to the specified path
    client.download_media(&chat_downloadable_photo, path).await?;

    Ok(())
}
