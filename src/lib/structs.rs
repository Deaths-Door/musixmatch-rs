use api_request_utils_rs::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Translation {
    language : String,
    translation : String
}

#[derive(Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Artist {
    #[serde(rename="artist_id")]
    id : u16,

    #[serde(rename="artist_name")]
    name : String,

    #[serde(rename="artist_name_translation_list")]
    name_translations : Vec<Translation>,

    #[serde(rename="artist_comment")]
    comment : String,

    #[serde(rename="artist_country")]
    country : String,

    #[serde(rename="artist_alias_list")]
    alias : Vec<String>,

    #[serde(rename="artist_rating")]
    rating : u8,

    #[serde(rename="restricted")]
    is_restricted : bool,

    #[serde(rename="begin_date_year")]
    begin_year : u16,

    #[serde(rename="begin_date")]
    begin_data : String,

    #[serde(rename="end_date_year")]
    end_year : u16,

    #[serde(rename="end_date")]
    end_data : String,
}

#[derive(Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Track {
    #[serde(rename = "track_id")]
    id: i32,
    #[serde(rename = "track_name")]
    name: String,
    #[serde(rename = "track_rating")]
    rating: i32,
    #[serde(rename = "num_favourite")]
    number_mal_added_to_favourite_by_music_match_users: i32,

    #[serde(rename = "commontrack_id")]
    common_track_id: i32,

    #[serde(rename = "instrumental")]
    is_instrumental: bool,

    #[serde(rename = "explicit")]
    is_explicit: bool,

    #[serde(rename = "has_lyrics")]
    has_lyrics: bool,

    #[serde(rename = "has_subtitles")]
    has_subtitles: bool,

    #[serde(rename = "has_richsync")]
    has_richsync: bool,

    #[serde(rename = "album_id")]
    album_id: i32,

    #[serde(rename = "album_name")]
    album_name: String,

    #[serde(rename = "artist_id")]
    artist_id: i32,

    #[serde(rename = "artist_name")]
    artist_name: String,

    #[serde(rename = "track_share_url")]
    share_url: String,

    #[serde(rename = "track_edit_url")]
    edit_url: String,

    #[serde(rename = "restricted")]
    is_restricted: bool,

    #[serde(rename = "updated_time")]
    updated_time: String,

    #[serde(rename = "primary_genres")]
    genres: Vec<Genre>,

    #[serde(rename = "track_name_translation_list")]
    translated_names: Vec<Translation>,
}

#[derive(Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Genre {
    #[serde(rename = "music_genre_id")]
    id: i32,
    #[serde(rename = "music_genre_parent_id")]
    parent_id: i32,
    #[serde(rename = "music_genre_name")]
    name: String,
    #[serde(rename = "music_genre_name_extended")]
    name_extended: String,
    #[serde(rename = "music_genre_vanity")]
    vanity: String
}

#[derive(Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Lyrics {
    #[serde(rename = "lyrics_id")]
    id: i32,
    
    #[serde(rename = "restricted")]
    is_restricted: bool,

    #[serde(rename = "instrumental")]
    is_instrumental: bool,

    #[serde(rename = "explicit")]
    is_explicit: bool,

    #[serde(rename = "lyrics_body")]
    lyrics: String,

    #[serde(rename = "lyrics_language")]
    language: String,

    #[serde(rename = "script_tracking_url")]
    script_tracking_url: String,

    #[serde(rename = "pixel_tracking_url")]
    pixel_tracking_url: String,

    #[serde(rename = "lyrics_copyright")]
    copyright: String,

    #[serde(rename = "backlink_url")]
    backlink_url: String
    ,
    #[serde(rename = "updated_time")]
    updated_time: String,
}

#[derive(Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct LyricMood {
    mood_list: Vec<Mood>,
    raw_data: RawData,
}

#[derive(Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
struct Mood {
    label: String,
    value: f32,
}

#[derive(Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
struct RawData {
    valence: f32,
    arousal: f32,
}

#[derive(Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct Snippet {
    #[serde(rename = "snippet_language")]
    language: String,
    #[serde(rename = "snippet_id")]
    id: i32,
    #[serde(rename = "restricted")]
    is_restricted: bool,
    #[serde(rename = "instrumental")]
    is_instrumental: bool,
    #[serde(rename = "snippet_body")]
    snippet_body: String,
    #[serde(rename = "script_tracking_url")]
    script_tracking_url: String,
    #[serde(rename = "pixel_tracking_url")]
    pixel_tracking_url: String,
    #[serde(rename = "html_tracking_url")]
    html_tracking_url: String,
    #[serde(rename = "updated_time")]
    updated_time: String,
}

#[derive(Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Subtitle {

}