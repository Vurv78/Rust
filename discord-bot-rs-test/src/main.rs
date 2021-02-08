// Basic Bot that listens for any messages, replies to the user's name, and tells them to Shut up.
// Fair warning: If you haven't read it somewher else already, this has no ratelimiting.
// Discord will probably show up to your house and personally take away your PC if you leave this unattended.

use discord_bot_rs::Bot;
use discord_bot_rs::Deserialize;

// Make some basic User and Message structs to turn the Gateway payload into using serde_json.
#[derive(Deserialize)]
struct User {
    id: String,
    username: String,
    bot: Option<bool>
}

#[derive(Deserialize)]
struct Message {
    content: String,
    author: User,
    channel_id: String,
}

fn main() {
    let tok = std::env::var("BOT_TOKEN").expect("Couldn't get discord bot token.");
    let mut test_bot = Bot::new( &tok );
    test_bot.on_event("MESSAGE_CREATE", |bot, data| {
        match serde_json::from_value::<Message>(data) {
            Ok(msg) => {
                // Make sure the message wasn't sent by a bot.
                // You should also check for if it was sent by system, see https://discord.com/developers/docs/resources/user#user-object
                let mut is_bot = false;
                if let Some(isbot) = msg.author.bot {
                    is_bot = isbot;
                }
                if !is_bot {
                    match bot.send_message(&msg.channel_id, &format!("Shut up {}", msg.author.username) ) {
                        Ok(_) => {
                            println!("Successfully sent message.");
                        }
                        Err(w) => {
                            println!("Failed to send message. Why: [{}]", w);
                        }
                    }
                }
            }
            Err(why) => {
                // Usually if you mess up in your struct declaration.
                println!("Couldn't deserialize Message. Why: [{}]", why);
            }
        }
    });
    test_bot.start_listening().expect("Couldn't startup bot.");
}

