use grammers_client::Client;

//TODO: This just logs chats, I need to return them.
pub async fn get_all_chats(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching all chats...");

    let mut iter = client.iter_dialogs();

    //TODO: Unwrap
    let iter_count = iter.total().await.unwrap();

    for _ in 0..iter_count {
        let next_dialog_result = iter.next().await;
        match next_dialog_result {
            Ok(next_dialog_option) => {
                match next_dialog_option {
                    Some(next_dialog) => {
                        println!("{}",next_dialog.chat.id());
                        println!("{}",next_dialog.chat.name());
                    },
                    None => println!("Dialog is None."),
                }
            },
            Err(_) => {
                println!("Dialog invalid.");
                return Ok(());
            }
        }
    }

    Ok(())
}