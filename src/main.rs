extern crate dotenv;
use dotenv::dotenv;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
type WhereIsStarknetDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveWebsite,
    ReceiveExactUrl {
        website: String,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting Where is Starknet bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(dptree::case![State::ReceiveWebsite].endpoint(receive_website))
            .branch(dptree::case![State::ReceiveExactUrl { website }].endpoint(receive_exact_url)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn start(bot: Bot, dialogue: WhereIsStarknetDialogue, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "What is the website where Starknet is missing ?",
    )
    .await?;
    dialogue.update(State::ReceiveWebsite).await?;
    Ok(())
}

async fn receive_website(
    bot: Bot,
    dialogue: WhereIsStarknetDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "What's the exact url ? ?")
                .await?;
            dialogue
                .update(State::ReceiveExactUrl {
                    website: text.into(),
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

async fn receive_exact_url(
    bot: Bot,
    dialogue: WhereIsStarknetDialogue,
    website: String, // Available from `State::ReceiveAge`.
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(exact_url) => {
            let report = format!("Website: {website}\nExact URL: {exact_url}");
            bot.send_message(msg.chat.id, report).await?;
            dialogue.exit().await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
