use serenity::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().unwrap();
    let token = std::env::var("DISCORD_BOT_TOKEN").unwrap();
    let mut client = Client::builder(token, juixe_bot::intents())
        .event_handler(juixe_bot::MainHandler)
        .await
        .expect("Error while creating client");
    client.start().await.expect("Error on event loop");
}
