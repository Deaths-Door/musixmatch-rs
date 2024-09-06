use std::ops::{Deref, DerefMut};

use crate::{
    client::MusixAbgleich,
    prelude::{Domain, Parameters},
};
use api_request_utils::{serde_json::Value, RequestError};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TrackingUrl(String);

impl Deref for TrackingUrl {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl DerefMut for TrackingUrl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl<A, F> MusixAbgleich<A, F>
where
    A: AsRef<str> + Send + Sync,
    F: Fn(RequestError<Value>) + Sync + Send,
{
    /// Get the base url for the tracking script
    ///
    /// With this api you’ll be able to get the base url for the tracking script you need to insert in your page to legalize your existent lyrics library.
    /// Read more here: [rights-clearance-on-your-existing-catalog](https://developer.musixmatch.com/documentation/rights-clearance-on-your-existing-catalog)
    pub async fn tracking_url(&self, domain: impl Into<Domain<'_>>) -> Option<TrackingUrl> {
        let mut parameters = Parameters::default();
        domain.into().add_to_hashmap(&mut parameters);

        self.request("tracking.url.get", &parameters, "url")
            .await
            .map(|v| TrackingUrl(v))
    }
}

#[cfg(test)]
mod test {
    use crate::api::test_utils;

    #[tokio::test]
    async fn test_tracking_url() {
        let client = test_utils::initialize_client();

        test_utils::assert_client_test(client.tracking_url("www.mylyricswebsite.com")).await
    }
}
