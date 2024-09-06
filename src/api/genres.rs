use super::{Genre, MusicGenreItem};
use crate::{client::MusixAbgleich, prelude::Parameters};
use api_request_utils::{serde_json::Value, RequestError};

impl<A, F> MusixAbgleich<A, F>
where
    A: AsRef<str> + Send + Sync,
    F: Fn(RequestError<Value>) + Sync + Send,
{
    /// Get the list of music genres in the catalogue.
    pub async fn genres(&self) -> Option<Vec<Genre>> {
        let parmeters = Parameters::default();
        let response: Option<Vec<MusicGenreItem>> = self
            .request("music.genres.get", &parmeters, "music_genre_list")
            .await;

        response.map(|v| v.into_iter().map(|v| v.music_genre).collect())
    }
}

#[cfg(test)]
mod test {
    use crate::api::test_utils;

    #[tokio::test]
    async fn test_genres() {
        let client = test_utils::initialize_client();
        test_utils::assert_client_test(client.genres()).await
    }
}
