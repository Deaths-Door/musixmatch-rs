use default_args::default_args;

use crate::{
    SubtitleFormat, 
    Chart,

    Artist,
    Track,
    Lyrics,
    LyricMood,
    Snippet,
    Genre,
    Subtitle,
    Album
};

default_args! { 
    /// Note: Refer to the original documentation of the `top_artists_by_country`
    /// method in the `MusixAbgleich` struct for more details and authentication requirements.
    pub async fn top_artists_by_country(musicabgleich : MusixAbgleich,country : Option<&str> = None,page : Option<u16>  = None,page_size : Option<u8>  = None) -> Option<Vec<Artist>> {
        musicabgleich.top_artists_by_country(country,page,page_size)
    }
    /// Note: Refer to the original documentation of the `top_artists_by_country`
    /// method in the `MusixAbgleich` struct for more details and authentication requirements.
    pub async fn top_tracks_by_country(&self,country : Option<&str> = None,chart_name : Option<Chart> = None,has_lyrics : Option<bool> = None,page : Option<u16> = None,page_size : Option<u8> = None) -> Option<Vec<Track>> {
        musicabgleich.top_tracks_by_country(country,page,page_size)
    }
}