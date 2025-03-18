use grammers_client::types::Message;

pub fn handle_message(message: &Message) {
    let chat = &message.chat();
    let message_text = message.text();
    match chat {
        grammers_client::types::Chat::User(user) => {
            println!(
                "New message from User {}: {} <<USERNAME: {:?}",
                user.id(),
                message_text,
                user.username().unwrap_or("Unknown username")
            );
        }
        grammers_client::types::Chat::Group(group) => {
            //TODO: Unwrap
            let sender = message.sender().unwrap();
            let sender_name = sender.name();
            let sender_username = sender.username().unwrap_or("no_username");
            let group_id = message.chat().id();

            if sender_username.contains("Phanes") || sender_username.contains("Rick") {
                println!("Message sent from Phanes or Rick. Skipping...");
                println!("-------------------------------------");
                return;
            }

            println!(
                "New message in Group {} with ID: {}\n <<USERNAME: {:?} <<NAME: {:?} \n Message: \n {}",
                group.title(),
                group_id,
                sender_username,
                sender_name,
                message_text,
            );
        }
        grammers_client::types::Chat::Channel(channel) => {
            let channel_id = channel.id();
            println!(
                "New message in Channel {} with ID: {}\n Messaage: \n {}",
                channel.title(),
                channel_id,
                message_text
            );
        }
    }
    println!("-------------------------------------------------------------------")
}
