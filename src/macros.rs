use default_args::default_args;

use crate::{
    MusixAbgleich,


    SubtitleFormat, 
    Chart,
    SortBy,

    Artist,
    Track,
    Lyrics,
    Subtitle,
    Album
};


default_args! { 
    export pub async fn top_artists_by_country(musicabgleich : &MusixAbgleich<'_>,country : Option<&str> = None,page : Option<u16>  = None,page_size : Option<u8>  = None) -> Option<Vec<Artist>> {
        musicabgleich.top_artists_by_country(country,page,page_size).await
    }
}

default_args! { 
    export pub async fn top_tracks_by_country(musicabgleich : &MusixAbgleich<'_>,country : Option<&str> = None,chart_name : Option<Chart> = None,has_lyrics : Option<bool> = None,page : Option<u16> = None,page_size : Option<u8> = None) -> Option<Vec<Track>> {
        musicabgleich.top_tracks_by_country(country,chart_name,has_lyrics,page,page_size).await
    }
}

default_args! { 
    export pub async fn track(musicabgleich : &MusixAbgleich<'_>,title : Option<&str> = None,artist : Option<&str> = None,album : Option<&str> = None) -> Option<Track> {
        musicabgleich.track(title,artist,album).await
    }
}

default_args! { 
    export pub async fn track_lyrics(musicabgleich : &MusixAbgleich<'_>,title : Option<&str> = None,artist : Option<&str> = None) -> Option<Lyrics> {
        musicabgleich.track_lyrics(title,artist).await
    }
}

default_args! { 
    export pub async fn track_subtitle(musicabgleich : &MusixAbgleich<'_>,commontrack_id : u32 = None,subtitle_length/*seconds*/ : Option<u16> = None,max_deviation : Option<u8> /*seconds*/ = None,format : Option<SubtitleFormat> = None) -> Option<Subtitle> {
        musicabgleich.track_subtitle(commontrack_id,subtitle_length,max_deviation,format).await
    }
}

default_args! { 
    export pub async fn subtitle(musicabgleich : &MusixAbgleich<'_>,title : Option<&str> = None,artist : Option<&str> = None,album : Option<&str> = None,subtitle_length/*seconds*/ : Option<u16> = None,max_deviation : Option<u8> /*seconds*/ = None) -> Option<Track> {
        musicabgleich.subtitle(title,artist,album,subtitle_length,max_deviation).await
    }
}

default_args! { 
    export pub async fn track_lyrics_translations_with_track_irsc(musicabgleich : &MusixAbgleich<'_>, id: &str,min_completed : Option<f32> = None /*percent*/,selected_language : Option<&str> = None/* (ISO 639-1) */) -> Option<Lyrics> { 
        musicabgleich.track_lyrics_translations_with_track_irsc(id,min_completed,selected_language).await
    }
}

default_args! { 
    export pub async fn track_lyrics_translations_with_musixbrainx_id(musicabgleich : &MusixAbgleich<'_>, id: &str,min_completed : Option<f32> = None /*percent*/,selected_language : Option<&str> = None/* (ISO 639-1) */) -> Option<Lyrics> { 
        musicabgleich.track_lyrics_translations_with_musixbrainx_id(id,min_completed,selected_language).await
    }
}

default_args! { 
    export pub async fn track_lyrics_translations_with_track_id(musicabgleich : &MusixAbgleich<'_>, id: &str,min_completed : Option<f32> = None/*percent*/,selected_language : Option<&str> = None/* (ISO 639-1) */) -> Option<Lyrics> { 
        musicabgleich.track_lyrics_translations_with_track_id(id,min_completed,selected_language).await
    }
}

default_args! { 
    export pub async fn track_lyrics_translations_with_commontrack_id(musicabgleich : &MusixAbgleich<'_>, id: &str,min_completed : Option<f32> = None /*percent*/,selected_language : Option<&str> = None/* (ISO 639-1) */) -> Option<Lyrics> { 
        musicabgleich.track_lyrics_translations_with_commontrack_id(id,min_completed,selected_language).await
    }
}

default_args! { 
    export pub async fn track_subtitle_translations_with_track_isrc(musicabgleich : &MusixAbgleich<'_>, id: &str,min_completed : Option<f32> = None /*percent*/,selected_language : Option<&str> = None/* (ISO 639-1) */,subtitle_length/*seconds*/ : Option<u16> = None,max_deviation : Option<u8> = None /*seconds*/)-> Option<Subtitle> {
        musicabgleich.track_subtitle_translations_with_track_isrc(id,min_completed,selected_language,subtitle_length,max_deviation).await
    }
}

default_args! { 
    export pub async fn track_subtitle_translations_with_commontrack_id(musicabgleich : &MusixAbgleich<'_>, id: &str,min_completed : Option<f32> = None /*percent*/,selected_language : Option<&str> = None/* (ISO 639-1) */,subtitle_length/*seconds*/ : Option<u16> = None,max_deviation : Option<u8> = None /*seconds*/)-> Option<Subtitle> {
        musicabgleich.track_subtitle_translations_with_commontrack_id(id,min_completed,selected_language,subtitle_length,max_deviation).await
    }
}

default_args! { 
    export pub async fn search_artist(musicabgleich : &MusixAbgleich<'_>, artist_song: Option<&str> = None, artist_id: Option<u32> = None, artist_mbid: Option<&str> = None, page: Option<u32> = None, page_size: Option<u8> = None) -> Option<Artist> {
        musicabgleich.search_artist(artist_song,artist_id,artist_mbid,page,page_size).await
    }
}

default_args! { 
    export pub async fn artist_relating_albums_with_id(musicabgleich : &MusixAbgleich<'_>, id:u32 ,album_name: Option<bool> = None,release_date_sort: Option<SortBy> = None, page: Option<u32> = None, page_size: Option<u8> = None) -> Option<Vec<Album>> {
        musicabgleich.artist_relating_albums_with_id(id,album_name,release_date_sort,page,page_size).await
    }
}

default_args! { 
    export pub async fn artist_relating_albums_with_musixbrainz_id(musicabgleich : &MusixAbgleich<'_>, id:u32 ,album_name: Option<bool> = None,release_date_sort: Option<SortBy> = None, page: Option<u32> = None, page_size: Option<u8> = None) -> Option<Vec<Album>> {
        musicabgleich.artist_relating_albums_with_musixbrainz_id(id,album_name,release_date_sort,page,page_size).await
    }
}

default_args! { 
    export pub async fn artist_relating_artist_with_id(musicabgleich : &MusixAbgleich<'_>,id:u32,page: Option<u32> = None, page_size: Option<u8> = None) -> Option<Vec<Artist>> {
        musicabgleich.artist_relating_artist_with_id(id,page,page_size).await
    }
}

default_args! { 
    export pub async fn artist_relating_artist_with_musixbrainz_id(musicabgleich : &MusixAbgleich<'_>,id:u32,page: Option<u32> = None, page_size: Option<u8> = None) -> Option<Vec<Artist>> {
        musicabgleich.artist_relating_artist_with_musixbrainz_id(id,page,page_size).await
    }
}

default_args! { 
    export async fn album_tracks_with_id(musicabgleich : &MusixAbgleich<'_>, id: u32 , has_lyrics: Option<bool> = None, page: Option<u32> = None, page_size: Option<u8> = None) -> Option<Vec<Track>> {
        musicabgleich.album_tracks_with_id(id,has_lyrics,page,page_size).await
    } 
}

default_args! { 
    export async fn album_tracks_with_musixbrainz_id(musicabgleich : &MusixAbgleich<'_>, id: u32 , has_lyrics: Option<bool> = None, page: Option<u32> = None, page_size: Option<u8> = None) -> Option<Vec<Track>> {
        musicabgleich.album_tracks_with_musixbrainz_id(id,has_lyrics,page,page_size).await
    }     
}