use std::collections::HashMap;

use api_request_utils_rs::{
    RequestInfo,
    RequestModifiers,
    RequestDefaults,
    RequestHandler,
    reqwest::{
        Client,
        RequestBuilder
    },
};

struct MusixAbgleich<'a>(Client,&'a str);

impl RequestInfo for MusixAbgleich<'_> {
    const BASE_URL : &'static str = "https://api.musixmatch.com/ws/1.1";
}

impl RequestModifiers for MusixAbgleich<'_> {}

impl RequestDefaults for MusixAbgleich<'_> { 
    fn client(&self) -> &Client {
        &self.0
    }
    fn default_parameters(&self,request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.query(&[("apikey", self.1)])
    }
}

impl RequestHandler for MusixAbgleich<'_> {}

impl<'a> MusixAbgleich<'a> {
    pub fn new(api_key : &'a str) -> Self {
        MusixAbgleich(Client::new(),api_key)
    }

    pub async fn top_artists_by_country(&self,country : Option<&str>,page : Option<u16>,page_size : Option<u8>) {
        let parameters = HashMap::new();//HashMap::from([("country",country),("page",page),("page_size",page_size)]);
        let request = self.default_get_requestor("chart.artists.get",parameters);
      //  println!("Formed URL: {}", request.send().await.unwrap().url());
        let response = Self::request::<api_request_utils_rs::serde_json::Value,api_request_utils_rs::serde_json::Value>(request).await;
        if let Ok(ok) = response {
            println!("SUCCESS = {}",ok);
        }
        else {
            println!("FAILURE");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test] 
    async fn test(){
        let musicmatch = MusixAbgleich::new("96868e3713c6d6ced65dd79a1196771c");
        musicmatch.top_artists_by_country(None,None,None).await;
    }
}