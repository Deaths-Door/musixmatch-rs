use std::collections::HashMap;

use api_request_utils_rs::{
    RequestInfo,
    RequestModifiers,
    RequestDefaults,
    RequestHandler,
    reqwest::{
        Client,
        RequestBuilder,
        StatusCode
    },
    serde_json::{
        Value,
        from_value
    }
};

use crate::Artist;

struct MusixAbgleich<'a> {
    client : Client,
    api_key : &'a str, 
    error_resolver : Box<&'a dyn Fn(&StatusCode)>
}

impl RequestInfo for MusixAbgleich<'_> {
    const BASE_URL : &'static str = "https://api.musixmatch.com/ws/1.1";
}

impl RequestModifiers for MusixAbgleich<'_> {}

impl RequestDefaults for MusixAbgleich<'_> { 
    fn client(&self) -> &Client {
        &self.client
    }
    fn default_parameters(&self,request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.query(&[("apikey", self.api_key)])
    }
}

impl RequestHandler for MusixAbgleich<'_> {}

impl<'a> MusixAbgleich<'a> {
    pub fn new(api_key : &'a str,error_resolver : &'a dyn Fn(&StatusCode)) -> Self {
        MusixAbgleich { client : Client::new(),api_key : api_key,error_resolver : Box::new(error_resolver) }
    }

    fn status_code_from_value(value : &Value) -> StatusCode {
        let status : Option<u64> = value.get("message").unwrap()
            .get("header").unwrap()
            .get("status_code").unwrap()
            .as_u64();
        StatusCode::from_u16(status.unwrap() as u16)
    }

    fn body_content<'de,T : api_request_utils_rs::serde::DeserializeOwned>(value : Value) -> T {
        let body = value.get("body").unwrap();
        from_value::<T>(body).unwrap()
    }

    pub async fn top_artists_by_country(&self,country : Option<&str>,page : Option<u16>,page_size : Option<u8>) -> Vec<Artist> {
        let parameters = HashMap::new();//HashMap::from([("country",country),("page",page),("page_size",page_size)]);
        let request = self.default_get_requestor("chart.artists.get",parameters);
        let response = Self::request::<Value,Value>(request).await;
     //   let result = Self::resolve_error(response,|value| {
       //     (self.error_resolver)(Self::status_code_from_value(&value))
       // });

        Self::body_content(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test] 
    async fn test(){
        let musicmatch = MusixAbgleich::new("96868e3713c6d6ced65dd79a1196771c",|_|{

        });
        musicmatch.top_artists_by_country(None,None,None).await;
    }
}