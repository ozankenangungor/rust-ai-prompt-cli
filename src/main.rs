use std::any;

use prompts::{Prompt, text::TextPrompt};

use crate::ai_client::Gpt4AllClient;
mod ai_client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Gpt4AllClient::new("http://localhost:4891/v1/chat/completions")?;
    let mut prompt = TextPrompt::new("");

    loop {
        match prompt.run().await {
            Ok(Some(input)) => {
                if (input == "q") {
                    break;
                }
                println!("{input}")
            }
            Err(error) => {
                println!("Error: {error}");
            }
            _ => {}
        }
        prompt = TextPrompt::new("");
    }

    Ok(())
}
