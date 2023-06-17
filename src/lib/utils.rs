use std::collections::HashMap;

use reqwest::{Client,StatusCode,RequestBuilder};

#[async_trait::async_trait]
pub trait RequestHandler<'a> {
    const BASE_URL : &'a str;
    const API_KEY : &'a str = "apiKey";

    fn client(&self) -> &Client;
    fn api_key(&self) -> &str;

    fn on_error(status_code: StatusCode);
    
    fn concentrate_endpoint(endpoint : &str) -> String {
        format!("{}/{}?",Self::BASE_URL,endpoint)
    }

    //TODO add get,push variants from https://dtantsur.github.io/rust-openstack/reqwest/struct.Method.html
    fn build_request(&self,endpoint : &str,parameters : &HashMap<&str,&str>) -> RequestBuilder {
        self.client().get(Self::concentrate_endpoint(endpoint)).query(&parameters)
    }

    //TODO enable this again
    /*fn build_parameters<Function : FnOnce(&mut HashMap<&str, &str>)>(&self,function: Function) -> HashMap<&'a str, &'a str> {
        let mut parameters = HashMap::new();
        parameters.insert(Self::API_KEY,self.api_key());
        function(&mut parameters);
        parameters
    }*/

    async fn request<T>(&self,endpoint: &str,parameters : &HashMap<&str,&str>) -> Result<T, ()> where T : for<'de> serde::Deserialize<'de> {
        let response = self.build_request(endpoint,parameters)
            .send()
            .await
            .expect("Error in sending Https Request");

        let status = response.status();

        if !status.is_success() {
            //TODO enable this againc
           // self.on_error();
            return Err(())
        }

        let body = response
            .text()
            .await
            .expect("Error reading response body");

        let result : T = serde_json::from_str(&body).expect("Error deserializing response body");
        Ok(result)
    }
}