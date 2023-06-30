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