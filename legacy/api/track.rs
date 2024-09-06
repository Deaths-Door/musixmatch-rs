use super::{utils::*, Track};
use crate::client::MusixAbgleich;
use api_request_utils::{serde_json::Value, RequestError, RequestHandler};
use chrono::Datelike;

//#[bon::bon]
impl<A, F> MusixAbgleich<A, F>
where
    A: AsRef<str> + Send + Sync,
    F: Fn(RequestError<Value>) + Sync + Send,
{
    /// Get track information by Musixmatch Track ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The Musixmatch Track ID.
    pub async fn track_with_id<'a>(&self, id: impl Into<TrackId<'a>>) -> Option<Track> {
        let mut parmeters = parmeters::default();
        id.into().add_to_hashmap(&mut parmeters);

        self.get_request_handler(
            "track.get",
            &parmeters,
            Self::create_map(|value| value),
            &self.error_resolver,
        )
        .await
    }

    /// Search for track in our database.
    ///
    /// # parmeters
    ///
    /// `q_track` : The song title
    /// `q_artist` : The song artist
    /// `q_lyrics` : Any word in the lyrics
    /// `q_track_artist` : Any word in the song title or artist name
    /// `q_writer` : Search among writers
    /// `q` : Any word in the song title or artist name or lyrics
    /// `f_artist_id` : When set, filter by this artist id
    /// `f_music_genre_i`d : When set, filter by this music category id
    /// `f_lyrics_language` : Filter by the lyrics language (en,it,..)
    /// `f_has_lyrics` : When set, filter only contents with lyrics
    /// `f_track_release_group_first_release_date_min` : When set, filter the tracks with release date newer than value, format is YYYYMMDD
    /// `f_track_release_group_first_release_date_max` : When set, filter the tracks with release date older than value, format is YYYYMMDD
    /// `s_artist_rating` : Sort by our popularity index for artists (asc|desc)
    /// `s_track_rating` : Sort by our popularity index for tracks (asc|desc)
    /// `quorum_factor` : Search only a part of the given query string.Allowed range is (0.1 – 0.9)
    /// `page` : Define the page number for paginated results
    /// `page_size` : Define the page size for paginated results. Range is 1 to 100.
    #[builder]
    pub async fn search_track<'l, L, U>(
        &'l self,
        #[builder(into, default)] query_track: QueryTrack<'l>,
        query_artist: QueryArtist<'l>,
        query_lyrics: QueryLyrics<'l>,
        query_track_artist: QueryTrackArtist<'l>,
        query_writer: QueryWriter<'l>,
        query_any: QueryAny<'l>,
        filter_artist_id: FilterArtistId,
        filter_music_genre: FilterMusicGenre,
        filter_lyrics_language: FilterLyricsLanguage<'l>,
        filter_has_lyrics: FilterHasLyrics,
        filter_track_release_group_first_release_date_min: FilterReleaseGroupFirstReleaseDateMin<L>,
        filter_track_release_group_first_release_date_max: FilterReleaseGroupFirstReleaseDateMax<U>,
        sort_artist_rating: SortArtistRating,
        sort_track_rating: SortTrackRating,
        quorom_factor: QueryQuoromFactor,
        page: Page,
        page_size: PageSize,
    ) -> Option<Track>
    where
        L: Datelike,
        U: Datelike,
    {
        let mut parmeters = parmeters::default();

        self.get_request_handler(
            "track.search",
            &parmeters,
            Self::create_map(|value| value),
            &self.error_resolver,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn top_artists() {
        let client = crate::api::test_utils::initialize_client();
        crate::api::test_utils::assert_client_test(client.track_with_id(5920049u32)).await;
    }

    #[tokio::test]
    async fn top_tracks() {
        let client = crate::api::test_utils::initialize_client();
        crate::api::test_utils::assert_client_test(client.track_with_id("AA-6Q7-21-00847")).await;
    }
}
