#![allow(dead_code)]
#![allow(missing_docs)]

use api_request_utils::serde::{Deserialize,Serialize};
use getset::{Getters, MutGetters, Setters};

#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq, PartialOrd,Clone)]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct Translation {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    language : String,
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    translation : String
}
#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd,Clone )]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct Artist {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="artist_id")]
    id : u16,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="artist_name")]
    name : String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="artist_name_translation_list")]
    name_translations : Vec<Translation>,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="artist_comment")]
    comment : String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="artist_country")]
    country : String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="artist_alias_list")]
    alias : Vec<String>,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="artist_rating")]
    rating : u8,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="restricted")]
    is_restricted : bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="begin_date_year")]
    begin_year : u16,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="begin_date")]
    begin_data : String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="end_date_year")]
    end_year : u16,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename="end_date")]
    end_data : String,
}

#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd ,Clone)]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct Track {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "track_id")]
    id: u32,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "track_name")]
    name: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "track_rating")]
    rating: u32,
    
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "num_favourite")]
    number_mal_added_to_favourite_by_music_match_users: u32,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "commontrack_id")]
    common_track_id: u32,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "instrumental")]
    is_instrumental: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "explicit")]
    is_explicit: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "has_lyrics")]
    has_lyrics: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "has_subtitles")]
    has_subtitles: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "has_richsync")]
    has_richsync: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "album_id")]
    album_id: u32,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "album_name")]
    album_name: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "artist_id")]
    artist_id: u32,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "artist_name")]
    artist_name: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "track_share_url")]
    share_url: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "track_edit_url")]
    edit_url: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "restricted")]
    is_restricted: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "updated_time")]
    updated_time: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "primary_genres")]
    genres: Vec<Genre>,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "track_name_translation_list")]
    translated_names: Vec<Translation>,
}

#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd ,Clone)]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct Genre {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "music_genre_id")]
    id: u32,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "music_genre_parent_id")]
    parent_id: u32,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "music_genre_name")]
    name: String,
    
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "music_genre_name_extended")]
    name_extended: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "music_genre_vanity")]
    vanity: String
}

#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd ,Clone)]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct Lyrics {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "lyrics_id")]
    id: u32,
    
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "restricted")]
    is_restricted: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "instrumental")]
    is_instrumental: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "explicit")]
    is_explicit: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "lyrics_body")]
    lyrics: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "lyrics_language")]
    language: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "script_tracking_url")]
    script_tracking_url: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "pixel_tracking_url")]
    pixel_tracking_url: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "lyrics_copyright")]
    copyright: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "backlink_url")]
    backlink_url: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "updated_time")]
    updated_time: String,
}

#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd ,Clone)]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct LyricMood {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    mood_list: Vec<Mood>,
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    raw_data: RawData,
}

#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd ,Clone)]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct Mood {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    label: String,
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    value: f32,
}

#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd,Clone )]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct RawData {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    valence: f32,
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    arousal: f32,
}

#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd,Clone )]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct Snippet {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "snippet_language")]
    language: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "snippet_id")]
    id: u32,
    
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "restricted")]
    is_restricted: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "instrumental")]
    is_instrumental: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "snippet_body")]
    snippet_body: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "script_tracking_url")]
    script_tracking_url: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "pixel_tracking_url")]
    pixel_tracking_url: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "html_tracking_url")]
    html_tracking_url: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "updated_time")]
    updated_time: String,
}

#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd ,Clone)]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct Subtitle {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "subtitle_id")]
    subtitle_id: u32,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "restricted")]
    is_restricted: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    subtitle_body: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    subtitle_language: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    script_tracking_url: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    pixel_tracking_url: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    html_tracking_url: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    lyrics_copyright: String,
}

#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd ,Clone)]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct Album {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "album_id")]
    id: u32,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "album_mbid")]
    music_brainz_identifier: Option<String>,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "album_name")]
    name: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "album_rating")]
    rating: u32,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "album_release_date")]
    release_date: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "artist_id")]
    artist_id: u32,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "artist_name")]
    artist_name: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "album_pline")]
    album_pline: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "album_copyright")]
    album_copyright: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "album_label")]
    album_label: String,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "primary_genres")]
    genres: Vec<Genre>,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "restricted")]
    is_restricted: bool,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "external_ids")]
    external_identities: ExternalIdentities,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    #[serde(rename = "updated_time")]
    updated_time: String,
}

#[derive(Getters, Setters, MutGetters)]
#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd ,Clone)]
#[serde(crate = "api_request_utils::serde")] // must be below the derive attribute
pub struct ExternalIdentities {
    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    spotify: Vec<String>,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    itunes: Vec<String>,

    #[getset(get = "pub",set = "pub", get_mut = "pub")]
    amazon_music: Vec<String>,
}