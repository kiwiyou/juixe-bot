use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub fn intents() -> GatewayIntents {
    GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
}

pub struct MainHandler;

#[async_trait]
impl EventHandler for MainHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected to discord as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!echo") {
            let content = msg.content[5..].trim_start();
            if !content.is_empty() && !msg.author.bot {
                if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
                    eprintln!("Error sending message: {:?}", why);
                }
            }
        }
    }
}
