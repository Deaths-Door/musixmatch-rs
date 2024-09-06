use super::{Artist, ArtistListItem, Track, TrackListItem};
use crate::{
    client::MusixAbgleich,
    prelude::{Country, FilterHasLyrics, Page, PageSize, Parameters},
};
use api_request_utils::{serde_json::Value, RequestError};
use strum::Display;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Display)]
pub enum Chart {
    #[strum(serialize = "top")]
    TopEditorialChart,
    #[strum(serialize = "hot")]
    HotMostViewedLyricsLast2Hours,
    #[strum(serialize = "mxmweekly")]
    MxmWeeklyMostViewedLyricsLast7Days,
    #[strum(serialize = "mxmweekly_new")]
    MxmWeeklyNewMostViewedLyricsLast7DaysNewReleasesOnly,
}

impl From<Chart> for Value {
    fn from(format: Chart) -> Self {
        Value::from(format.to_string())
    }
}

impl Chart {
    pub(crate) fn add_to_hashmap(self, parameters: &mut Parameters<'_>) {
        parameters.insert("chart", self.into());
    }
}

#[bon::bon]
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
        #[builder(into, default)] country: Country<'a>,
        #[builder(into)] page: Page,
        page_size: PageSize,
    ) -> Option<Vec<Artist>> {
        let mut parmeters = Parameters::default();

        country.add_to_hashmap(&mut parmeters);
        page.add_to_hashmap(&mut parmeters);
        page_size.add_to_hashmap(&mut parmeters);

        let response: Option<Vec<ArtistListItem>> = self
            .request("chart.artists.get", &parmeters, "artist_list")
            .await;

        response.map(|v| v.into_iter().map(|v| v.artist).collect())
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
        #[builder(into, default)] country: Country<'a>,
        #[builder(into, default)] has_lyrics: FilterHasLyrics,
        #[builder(into)] page: Page,
        page_size: PageSize,
        chart_name: Chart,
    ) -> Option<Vec<Track>> {
        let mut parmeters = Parameters::default();

        country.add_to_hashmap(&mut parmeters);
        has_lyrics.add_to_hashmap(&mut parmeters);
        chart_name.add_to_hashmap(&mut parmeters);
        page.add_to_hashmap(&mut parmeters);
        page_size.add_to_hashmap(&mut parmeters);

        let response: Option<Vec<TrackListItem>> = self
            .request("chart.tracks.get", &parmeters, "track_list")
            .await;

        response.map(|v| v.into_iter().map(|v| v.track).collect())
    }
}

#[cfg(test)]
mod test {
    use crate::{api::test_utils, prelude::PageSize};

    #[tokio::test]
    async fn test_top_artists_by_country() {
        let client = test_utils::initialize_client();
        test_utils::assert_client_test(
            client
                .top_artists_by_country()
                .page(1u8)
                .page_size(PageSize::try_from(1u8).unwrap())
                .call(),
        )
        .await
    }

    #[tokio::test]
    async fn test_top_tracks_by_country() {
        let client = test_utils::initialize_client();
        test_utils::assert_client_test(
            client
                .top_tracks_by_country()
                .chart_name(crate::api::charts::Chart::TopEditorialChart)
                .page(1u8)
                .page_size(PageSize::try_from(1u8).unwrap())
                .call(),
        )
        .await
    }
}
