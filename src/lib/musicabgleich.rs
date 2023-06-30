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
        request_builder.query(&[("apiKey", self.1)])
    }
}

impl RequestHandler for MusixAbgleich<'_> {}

impl<'a> MusixAbgleich<'a> {
    pub fn new(api_key : &'a str) -> Self {
        MusixAbgleich(Client::new(),api_key)
    }   
    
}