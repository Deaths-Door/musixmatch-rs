#![cfg_attr(not(test), forbid(unused_mut))]

mod charts;
mod genres;
mod structs;
mod tracking;

pub use structs::*;

#[cfg(test)]
mod test_utils {
    use std::future::Future;

    use api_request_utils::{serde_json::Value, RequestError};

    use crate::client::MusixAbgleich;

    pub(in crate::api) fn initialize_client(
    ) -> MusixAbgleich<String, impl Fn(RequestError<Value>) -> ()> {
        use dotenvy::dotenv;

        dotenv().expect("Failure");

        let api_key = std::env::var("API_KEY").expect("No API_KEY provided");

        MusixAbgleich::builder()
            .api_key(api_key)
            .error_resolver(|value| eprintln!("{}", value.to_string()))
            .build()
    }

    pub(in crate::api) async fn assert_client_test<O>(future: impl Future<Output = Option<O>>) {
        assert!(future.await.is_some())
    }
}
