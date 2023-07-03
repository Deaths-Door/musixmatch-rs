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

/// Note: Refer to the original documentation of the methods in the `MusixAbgleich` struct for more details and authentication requirements.
default_args! { 
    export pub async fn top_artists_by_country(musicabgleich : MusixAbgleich,country : Option<&str> = None,page : Option<u16>  = None,page_size : Option<u8>  = None) -> Option<Vec<Artist>> {
        musicabgleich.top_artists_by_country(country,page,page_size)
    }

    export pub async fn top_tracks_by_country(&self,country : Option<&str> = None,chart_name : Option<Chart> = None,has_lyrics : Option<bool> = None,page : Option<u16> = None,page_size : Option<u8> = None) -> Option<Vec<Track>> {
        musicabgleich.top_tracks_by_country(country,page,page_size)
    }

    export pub async fn track(&self,title : Option<&str> = None,artist : Option<&str> = None,album : Option<&str> = None) -> Option<Track> {
        musicabgleich.track(country,page,page_size)
    }

    export pub async fn track_lyrics(&self,title : Option<&str> = None,artist : Option<&str> = None) -> Option<Lyrics> {
        musicabgleich.track_lyrics(title,artist)
    }

    export pub async fn track_subtitle(&self,commontrack_id : u32 = None,subtitle_length/*seconds*/ : Option<u16> = None,max_deviation : Option<u8> /*seconds*/ = None,format : Option<SubtitleFormat> = None) -> Option<Subtitle> {
        musicabgleich.track_subtitle(commontrack_id,subtitle_length,max_deviation,format)
    }

    export pub async fn subtitle(&self,title : Option<&str> = None,artist : Option<&str> = None,album : Option<&str> = None,subtitle_length/*seconds*/ : Option<u16> = None,max_deviation : Option<u8> /*seconds*/ = None) -> Option<Track> {
        musicabgleich.track_subtitle(title,artist,max_deviation,album,subtitle_length,max_deviation)
    }

    export pub async fn track_lyrics_translations_with_track_irsc(&self, id: &str,min_completed : Option<f32> = None /*percent*/,selected_language : Option<&str> = None/* (ISO 639-1) */) -> Option<Lyrics> { 
        musicabgleich.track_lyrics_translations_with_track_irsc(id,min_completed,selected_language)
    }

    export pub async fn track_lyrics_translations_with_musixbrainx_id(&self, id: &str,min_completed : Option<f32> = None /*percent*/,selected_language : Option<&str> = None/* (ISO 639-1) */) -> Option<Lyrics> { 
        musicabgleich.track_lyrics_translations_with_musixbrainx_id(id,min_completed,selected_language)
    }

    export pub async fn track_lyrics_translations_with_track_id(&self, id: &str,min_completed : Option<f32> = None/*percent*/,selected_language : Option<&str> = None/* (ISO 639-1) */) -> Option<Lyrics> { 
        musicabgleich.track_lyrics_translations_with_track_id(id,min_completed,selected_language)
    }

    export pub async fn track_lyrics_translations_with_commontrack_id(&self, id: &str,min_completed : Option<f32> = None /*percent*/,selected_language : Option<&str> = None/* (ISO 639-1) */) -> Option<Lyrics> { 
        musicabgleich.track_lyrics_translations_with_commontrack_id(id,min_completed,selected_language)
    }
}