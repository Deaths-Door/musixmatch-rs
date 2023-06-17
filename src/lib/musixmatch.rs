use reqwest::{Client,StatusCode};
use crate::lib::utils::RequestHandler;

pub struct MusicAbgleich<'a> {
    client : Client,
    api_key : &'a str,
}

impl<'a> RequestHandler<'a> for MusicAbgleich<'a> {
    const BASE_URL : &'a str = "https://api.musixmatch.com/ws/1.1";

    fn client(&self) -> &Client {
        &self.client
    }

    fn api_key(&self) -> &str {
        self.api_key
    }

    //TODO finish this
    fn on_error(status_code: StatusCode) {}
}

impl<'a> MusicAbgleich<'a> { 
    pub fn new(api_key : &'a str) -> Self {
        MusicAbgleich {
            client : Client::new(),
            api_key : api_key,
        }
    }
}