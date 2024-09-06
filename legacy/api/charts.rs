use api_request_utils::{serde_json::Value, RequestError, RequestHandler};

use crate::client::MusixAbgleich;

use super::{parmeters, Artist, Chart, Country, FilterHasLyrics, Page, PageSize, Track};
use bon::bon;

#[bon]
impl<A, F> MusixAbgleich<A, F>
where
    A: AsRef<str> + Send + Sync,
    F: Fn(RequestError<Value>) + Sync + Send,
{
    /// Retrieves the top artists by country.
    ///
    /// # Arguments
    ///
    /// * `country` - A valid country code (default: "US").
    /// * `page` - The page number for paginated results.
    /// * `page_size` - The page size for paginated results. Range is 1 to 100.
    #[builder]
    pub async fn top_artists_by_country<'a>(
        &self,
        #[builder(into)] country: Country<'a>,
        #[builder(into)] page: Page,
        #[builder(into)] page_size: PageSize,
    ) -> Option<Vec<Artist>> {
        let mut parmeters = parmeters::default();
        country.add_to_hashmap(&mut parmeters);
        page.add_to_hashmap(&mut parmeters);
        page_size.add_to_hashmap(&mut parmeters);

        self.get_request_handler(
            "chart.artists.get",
            &parmeters,
            Self::create_map(|value| value),
            &self.error_resolver,
        )
        .await
    }

    /// Retrieves the top tracks by country.
    ///
    /// # Arguments
    ///
    /// * `country` - A valid 2-letter country code (default: "US"). Set "XW" for worldwide.
    /// * `chart_name` - Select among available charts: "top" (editorial chart), "hot" (most viewed lyrics in the last 2 hours),
    ///   "mxmweekly" (most viewed lyrics in the last 7 days), "mxmweekly_new" (most viewed lyrics in the last 7 days limited to new releases only).
    /// * `has_lyrics` - When set, filter only contents with lyrics.
    /// * `page` - The page number for paginated results.
    /// * `page_size` - The page size for paginated results. Range is 1 to 100.
    #[builder]
    pub async fn top_tracks_by_country<'a>(
        &self,
        #[builder(into)] country: Country<'a>,
        #[builder(into)] chart_name: Chart,
        #[builder(into)] page: Page,
        #[builder(into)] page_size: PageSize,
        #[builder(into)] has_lyrics: FilterHasLyrics,
    ) -> Option<Vec<Track>> {
        let mut parmeters = parmeters::default();
        country.add_to_hashmap(&mut parmeters);
        chart_name.add_to_hashmap(&mut parmeters);
        has_lyrics.add_to_hashmap(&mut parmeters);
        page.add_to_hashmap(&mut parmeters);
        page_size.add_to_hashmap(&mut parmeters);

        self.get_request_handler(
            "chart.tracks.get",
            &parmeters,
            Self::create_map(|value| value),
            &self.error_resolver,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{Country, Page, PageSize};

    #[tokio::test]
    async fn top_artists() {
        let client = crate::api::test_utils::initialize_client();
        crate::api::test_utils::assert_client_test(
            client
                .top_artists_by_country()
                .page(Page(1))
                .page_size(PageSize(1))
                .country(Country(Some("US")))
                .call(),
        )
        .await;
    }
    /*
    #[tokio::test]
    async fn top_tracks() {
        let client = crate::api::test_utils::initialize_client();
        crate::api::test_utils::assert_client_test(
            client
                .top_tracks_by_country()
                .chart_name(Chart::HotMostViewedLyricsLast2Hours)
                .page(Page(1))
                .page_size(PageSize(1))
                .call(),
        )
        .await;
    }*/
}
