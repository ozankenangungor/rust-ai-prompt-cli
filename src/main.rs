use prompts::{Prompt, text::TextPrompt};

#[tokio::main]
async fn main() {
    let mut prompt = TextPrompt::new("$ ");

    match prompt.run().await {
        Ok(Some(input)) => {
            println!("{input}")
        }
        Err(error) => {
            println!("Error: {error}");
        }
        _ => {}
    }
}
