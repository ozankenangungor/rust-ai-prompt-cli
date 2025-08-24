use prompts::{Prompt, text::TextPrompt};

#[tokio::main]
async fn main() {
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
}
