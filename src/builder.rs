#![allow(unused_results)]

use std::collections::HashMap;
use api_request_utils::serde_json::Value;

use crate::SortBy;

/// A struct representing a query for searching track
pub struct TrackSearchQuery<'a>(pub(crate) HashMap<&'a str,Value>);

impl<'a> TrackSearchQuery<'a> {
    /// Creates a new `TrackSearchQuery` instance with an empty HashMap.
    pub fn new() -> Self {
        TrackSearchQuery(HashMap::new())
    }

    /// Set the song title to search for.
    pub fn song_title(mut self, q_track: Option<&'a str>) -> Self {
        self.0.insert("q_track", q_track.into());
        self
    }

    /// Set the song artist to search for.
    pub fn song_artist(mut self, q_artist: Option<&'a str>) -> Self {
        self.0.insert("q_artist", q_artist.into());
        self
    }

    /// Set any word to search for in the lyrics.
    pub fn lyrics_contain_word(mut self, q_lyrics: Option<&'a str>) -> Self {
        self.0.insert("q_lyrics", q_lyrics.into());
        self
    }

    /// Set any word to search for in the song title or artist name.
    pub fn song_title_or_artist_contains_word(mut self, q_track_artist: Option<&'a str>) -> Self {
        self.0.insert("q_track_artist", q_track_artist.into());
        self
    }

    /// Search among writers for a specific word.
    pub fn search_writers(mut self, q_writer: Option<&'a str>) -> Self {
        self.0.insert("q_writer", q_writer.into());
        self
    }

    /// Set any word to search for in the song title, artist name, or lyrics.
    pub fn search_all_contains_word(mut self, q: Option<&'a str>) -> Self {
        self.0.insert("q", q.into());
        self
    }

    /// Filter by the artist ID.
    pub fn artist_id(mut self, f_artist_id: Option<u32>) -> Self {
        self.0.insert("f_artist_id", f_artist_id.into());
        self
    }

    /// Filter by the music category ID.
    pub fn music_genre_id(mut self, f_music_genre_id: Option<u32>) -> Self {
        self.0.insert("f_music_genre_id", f_music_genre_id.into());
        self
    }

    /// Filter by the lyrics language (en, it, ...).
    pub fn lyrics_language(mut self, f_lyrics_language: Option<&'a str>) -> Self {
        self.0.insert("f_lyrics_language", f_lyrics_language.into());
        self
    }

    /// Filter only contents with lyrics when set to true.
    pub fn has_lyrics(mut self, f_has_lyrics: Option<bool>) -> Self {
        self.0.insert("f_has_lyrics", f_has_lyrics.into());
        self
    }

    /// Filter the tracks with a release date newer than the specified value (format: YYYYMMDD).
    pub fn track_release_group_first_release_date_min(mut self, f_track_release_group_first_release_date_min: Option<&'a str>) -> Self {
        self.0.insert("f_track_release_group_first_release_date_min", f_track_release_group_first_release_date_min.into());
        self
    }

    /// Filter the tracks with a release date older than the specified value (format: YYYYMMDD).
    pub fn track_release_group_first_release_date_max(mut self, f_track_release_group_first_release_date_max: Option<&'a str>) -> Self {
        self.0.insert("f_track_release_group_first_release_date_max", f_track_release_group_first_release_date_max.into());
        self
    }

    /// Sort by our popularity index for artists (asc|desc).
    pub fn artist_rating(mut self, s_artist_rating: Option<SortBy>) -> Self {
        self.0.insert("s_artist_rating", s_artist_rating.into());
        self
    }

    /// Sort by our popularity index for tracks (asc|desc).
    pub fn track_rating(mut self, s_track_rating: Option<SortBy>) -> Self {
        self.0.insert("s_track_rating", s_track_rating.into());
        self
    }

    /// Search only a part of the given query string. Allowed range is (0.1 - 0.9).
    pub fn quorum_factor(mut self, quorum_factor: Option<f32>) -> Self {
        self.0.insert("quorum_factor", quorum_factor.into());
        self
    }

    /// Define the page number for paginated results.
    pub fn page(mut self, page: Option<u8>) -> Self {
        self.0.insert("page", page.into());
        self
    }

    /// Define the page size for paginated results. Range is 1 to 100.
    pub fn page_size(mut self, page_size: Option<u32>) -> Self {
        self.0.insert("page_size", page_size.into());
        self
    }
}