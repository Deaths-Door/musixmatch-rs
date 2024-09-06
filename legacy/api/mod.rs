mod charts;
mod enums;
mod structs;
//mod track;
mod utils;

pub use charts::*;
pub use enums::*;
pub use structs::*;
//pub use track::*;
pub use utils::*;

#[cfg(test)]
mod test_utils {
    use std::future::Future;

    use api_request_utils::{serde_json::Value, RequestError};

    use crate::client::MusixAbgleich;

    pub(in crate::api) fn initialize_client(
    ) -> MusixAbgleich<String, impl Fn(RequestError<Value>) -> ()> {
        use dotenv::dotenv;

        dotenv().expect("Failure");

        let api_key = std::env::var("API_KEY").expect("No API_KEY provided");

        MusixAbgleich::builder()
            .api_key(api_key)
            .error_resolver(|_| ())
            .build()
    }

    pub(in crate::api) async fn assert_client_test<O>(future: impl Future<Output = Option<O>>) {
        assert!(future.await.is_some())
    }

    /*
    #[tokio::test]
            async fn $name() {
                let client = crate::api::test_utils::initialize_client();
                crate::api::test_utils::assert_client_test(async { $apply }).await;
            }
             */
}
