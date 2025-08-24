pub struct Gpt4AllClient {
    client: reqwest::Client,
}

impl Gpt4AllClient {
    pub fn new() -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: reqwest::ClientBuilder::new().build()?,
        })
    }
}
