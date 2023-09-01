use default_args::default_args;

use crate::{
    MusixAbgleich,

    TrackSearchQuery,

    SubtitleFormat, 
    Chart,
    SortBy,

    Artist,
    Track,
    Lyrics,
    Subtitle,
    Album
};


default_args!{
    export async fn search_track(
        musicabgleich : &MusixAbgleich<'_>,
        song_title: Option<&str>,
        song_artist: Option<&str>,
        lyrics_contain_word: Option<&str>,
        song_title_or_artist_contains_word: Option<&str>,
        search_writers: Option<&str>,
        search_all_contains_word: Option<&str>,
        artist_id: Option<u32>,
        music_genre_id: Option<u32>,
        lyrics_language: Option<&str>,
        has_lyrics: Option<bool>,
        track_release_group_first_release_date_min: Option<&str>,
        track_release_group_first_release_date_max: Option<&str>,
        artist_rating: Option<SortBy>,
        track_rating: Option<SortBy>,
        quorum_factor: Option<f32>,
        page: Option<u8>,
        page_size: Option<u32>
    ) -> Option<Track> {
        let mut query = TrackSearchQuery::new();

        if let Some(title) = song_title {
            query = query.song_title(title);
        }

        if let Some(title) = song_artist {
            query = query.song_title(title);
        }


        if let Some(title) = lyrics_contain_word {
            query = query.lyrics_contain_word(title);
        }

        if let Some(title) = song_title_or_artist_contains_word {
            query = query.song_title_or_artist_contains_word(title);
        }


        if let Some(title) = search_writers {
            query = query.search_writers(title);
        }


        if let Some(title) = search_all_contains_word {
            query = query.search_all_contains_word(title);
        }

        if let Some(title) = artist_id {
            query = query.artist_id(title);
        }

        if let Some(title) = music_genre_id {
            query = query.music_genre_id(title);
        }

        if let Some(title) = lyrics_language {
            query = query.lyrics_language(title);
        }

        if let Some(title) = has_lyrics {
            query = query.has_lyrics(title);
        }

        if let Some(title) = track_release_group_first_release_date_min {
            query = query.track_release_group_first_release_date_min(title);
        }

        if let Some(title) = track_release_group_first_release_date_max {
            query = query.track_release_group_first_release_date_max(title);
        }

        if let Some(title) = artist_rating {
            query = query.artist_rating(title);
        }

        if let Some(title) = track_rating {
            query = query.track_rating(title);
        }

        if let Some(title) = quorum_factor {
            query = query.quorum_factor(title);
        }

        if let Some(title) = page {
            query = query.page(title);
        }

        if let Some(title) = page_size {
            query = query.page_size(title);
        }

        musicabgleich.search_track(query).await
    }
}