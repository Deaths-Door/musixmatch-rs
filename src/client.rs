use api_request_utils::{
    reqwest::{Client, RequestBuilder},
    serde::de::DeserializeOwned,
    serde_json::{from_value, Value},
    RequestDefaults, RequestError, RequestHandler, RequestInfo, RequestModifiers,
};

/// Represents a client for accessing the MusicMatch API.
///
/// The `MusixAbgleich` struct provides the necessary functionality to interact with the
/// MusicMatch API, including sending requests and handling errors.
#[bon::builder]
pub struct MusixAbgleich<A, F>
where
    A: AsRef<str> + Send + Sync,
    F: Fn(RequestError<Value>) + Sync + Send,
{
    #[builder(default = Client::new())]
    client: Client,

    /// A reference to a string representing the API key used for authentication.
    api_key: A,
    /// This is responsible for handling errors that occur during API requests.
    pub(crate) error_resolver: F,
}

impl<A, F> MusixAbgleich<A, F>
where
    A: AsRef<str> + Send + Sync,
    F: Fn(RequestError<Value>) + Sync + Send,
{
    pub(crate) fn create_map<O: DeserializeOwned>(
        value_mapper: impl FnOnce(&Value) -> &Value + Send + Sync,
    ) -> impl FnOnce(Value) -> O + Send + Sync {
        |response: Value| {
            from_value::<O>(value_mapper(response.get("body").unwrap()).clone()).unwrap()
        }
    }
}

impl<A, F> RequestInfo for MusixAbgleich<A, F>
where
    A: AsRef<str> + Send + Sync,
    F: Fn(RequestError<Value>) + Sync + Send,
{
    const BASE_URL: &'static str = "https://api.musixmatch.com/ws/1.1";

    fn client(&self) -> &Client {
        &self.client
    }
}

impl<A, F> RequestModifiers for MusixAbgleich<A, F>
where
    A: AsRef<str> + Send + Sync,
    F: Fn(RequestError<Value>) + Sync + Send,
{
}

impl<A, F> RequestDefaults for MusixAbgleich<A, F>
where
    A: AsRef<str> + Send + Sync,
    F: Fn(RequestError<Value>) + Sync + Send,
{
    fn default_parameters(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.query(&[("apikey", &self.api_key.as_ref())])
    }
}

impl<A, T, O, E, F> RequestHandler<T, O, E> for MusixAbgleich<A, F>
where
    A: AsRef<str> + Send + Sync,
    F: Fn(RequestError<Value>) + Sync + Send,
    T: DeserializeOwned,
    O: DeserializeOwned,
    E: DeserializeOwned,
{
}
