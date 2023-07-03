#![allow(dead_code)]

use api_request_utils_rs::serde::{Deserialize,Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Translation {
    pub language : String,
    pub translation : String
}

#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Artist {
    #[serde(rename="artist_id")]
    pub id : u16,

    #[serde(rename="artist_name")]
    pub name : String,

    #[serde(rename="artist_name_translation_list")]
    pub name_translations : Vec<Translation>,

    #[serde(rename="artist_comment")]
    pub comment : String,

    #[serde(rename="artist_country")]
    pub country : String,

    #[serde(rename="artist_alias_list")]
    pub alias : Vec<String>,

    #[serde(rename="artist_rating")]
    pub rating : u8,

    #[serde(rename="restricted")]
    pub is_restricted : bool,

    #[serde(rename="begin_date_year")]
    pub begin_year : u16,

    #[serde(rename="begin_date")]
    pub begin_data : String,

    #[serde(rename="end_date_year")]
    pub end_year : u16,

    #[serde(rename="end_date")]
    pub end_data : String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Track {
    
    #[serde(rename = "track_id")]
    pub id: i32,

    #[serde(rename = "track_name")]
    pub name: String,
    
    #[serde(rename = "track_rating")]
    pub rating: i32,
    
    #[serde(rename = "num_favourite")]
    pub number_mal_added_to_favourite_by_music_match_users: i32,

    #[serde(rename = "commontrack_id")]
    pub common_track_id: i32,

    #[serde(rename = "instrumental")]
    pub is_instrumental: bool,

    #[serde(rename = "explicit")]
    pub is_explicit: bool,

    #[serde(rename = "has_lyrics")]
    pub has_lyrics: bool,

    #[serde(rename = "has_subtitles")]
    pub has_subtitles: bool,

    #[serde(rename = "has_richsync")]
    pub has_richsync: bool,

    #[serde(rename = "album_id")]
    pub album_id: i32,

    #[serde(rename = "album_name")]
    pub album_name: String,

    #[serde(rename = "artist_id")]
    pub artist_id: i32,

    #[serde(rename = "artist_name")]
    pub artist_name: String,

    #[serde(rename = "track_share_url")]
    pub share_url: String,

    #[serde(rename = "track_edit_url")]
    pub edit_url: String,

    #[serde(rename = "restricted")]
    pub is_restricted: bool,

    #[serde(rename = "updated_time")]
    pub updated_time: String,

    #[serde(rename = "primary_genres")]
    pub genres: Vec<Genre>,

    #[serde(rename = "track_name_translation_list")]
    pub translated_names: Vec<Translation>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Genre {
    #[serde(rename = "music_genre_id")]
    pub id: i32,
    #[serde(rename = "music_genre_parent_id")]
    pub parent_id: i32,
    #[serde(rename = "music_genre_name")]
    pub name: String,
    #[serde(rename = "music_genre_name_extended")]
    pub name_extended: String,
    #[serde(rename = "music_genre_vanity")]
    pub vanity: String
}

#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Lyrics {
    #[serde(rename = "lyrics_id")]
    pub id: i32,
    
    #[serde(rename = "restricted")]
    pub is_restricted: bool,

    #[serde(rename = "instrumental")]
    pub is_instrumental: bool,

    #[serde(rename = "explicit")]
    pub is_explicit: bool,

    #[serde(rename = "lyrics_body")]
    pub lyrics: String,

    #[serde(rename = "lyrics_language")]
    pub language: String,

    #[serde(rename = "script_tracking_url")]
    pub script_tracking_url: String,

    #[serde(rename = "pixel_tracking_url")]
    pub pixel_tracking_url: String,

    #[serde(rename = "lyrics_copyright")]
    pub copyright: String,

    #[serde(rename = "backlink_url")]
    pub backlink_url: String,

    #[serde(rename = "updated_time")]
    pub updated_time: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct LyricMood {
    pub mood_list: Vec<Mood>,
    pub raw_data: RawData,
}

#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
struct Mood {
    pub label: String,
    pub value: f32,
}

#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
struct RawData {
    pub valence: f32,
    pub arousal: f32,
}

#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Snippet {
    #[serde(rename = "snippet_language")]
    pub language: String,
    #[serde(rename = "snippet_id")]
    pub id: i32,
    #[serde(rename = "restricted")]
    pub is_restricted: bool,
    #[serde(rename = "instrumental")]
    pub is_instrumental: bool,
    #[serde(rename = "snippet_body")]
    pub snippet_body: String,
    #[serde(rename = "script_tracking_url")]
    pub script_tracking_url: String,
    #[serde(rename = "pixel_tracking_url")]
    pub pixel_tracking_url: String,
    #[serde(rename = "html_tracking_url")]
    pub html_tracking_url: String,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Subtitle {
    #[serde(rename = "subtitle_id")]
    pub subtitle_id: i32,
    #[serde(rename = "restricted")]
    pub is_restricted: bool,
    pub subtitle_body: String,
    pub subtitle_language: String,
    pub script_tracking_url: String,
    pub pixel_tracking_url: String,
    pub html_tracking_url: String,
    pub lyrics_copyright: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Album {
    #[serde(rename = "album_id")]
    pub id: i32,
    #[serde(rename = "album_mbid")]
    pub music_brainz_identifier: Option<String>,
    #[serde(rename = "album_name")]
    pub name: String,
    #[serde(rename = "album_rating")]
    pub rating: i32,
    #[serde(rename = "album_release_date")]
    pub release_date: String,
    #[serde(rename = "artist_id")]
    pub artist_id: i32,
    #[serde(rename = "artist_name")]
    pub artist_name: String,
    #[serde(rename = "album_pline")]
    pub album_pline: String,
    #[serde(rename = "album_copyright")]
    pub album_copyright: String,
    #[serde(rename = "album_label")]
    pub album_label: String,
    #[serde(rename = "primary_genres")]
    pub genres: Vec<Genre>,
    #[serde(rename = "restricted")]
    pub is_restricted: bool,
    #[serde(rename = "external_ids")]
    pub external_identities: ExternalIdentities,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq,PartialOrd )]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct ExternalIdentities {
    pub spotify: Vec<String>,
    pub itunes: Vec<String>,
    pub amazon_music: Vec<String>,
}