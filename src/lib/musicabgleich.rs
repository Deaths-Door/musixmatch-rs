use std::collections::HashMap;

use api_request_utils::{
    RequestInfo,
    RequestModifiers,
    RequestDefaults,
    RequestHandler,
    RequestError,
    reqwest::{
        Client,
        RequestBuilder,
    },
    serde_json::{
        Value,
        from_value
    },
    ParameterHashMap, 
};

use crate::{
    SubtitleFormat, 
    Chart,
    SortBy,

    Artist,
    Track,
    Lyrics,
    LyricMood,
    Snippet,
    Genre,
    Subtitle,
    Album,

    TrackSearchQuery
};

pub struct MusixAbgleich<'a> {
    client : Client,
    api_key : &'a str, 
    error_resolver : Box<&'a dyn Fn(&RequestError<Value>)>
}

impl RequestInfo for MusixAbgleich<'_> {
    const BASE_URL : &'static str = "https://api.musixmatch.com/ws/1.1";
}

impl RequestModifiers for MusixAbgleich<'_> {}

impl RequestDefaults for MusixAbgleich<'_> { 
    fn client(&self) -> &Client {
        &self.client
    }
    fn default_parameters(&self,request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.query(&[("apikey", self.api_key)])
    }
}

impl RequestHandler for MusixAbgleich<'_> {}

/*
/// At this moment these endpoints are not implemented 
/// * catalogue.dump.get 
/// * work.post 
/// * work.validity.post 
/// * track.richsync 
impl<'a> MusixAbgleich<'a> {
    /// Constructs a new instance of the MusixAbgleich type.
    ///
    /// This function creates a new MusixAbgleich instance with the provided API key and error resolver.
    /// The MusixAbgleich type is designed for making API requests and handling errors using a client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A reference to a string representing the API key used for authentication.
    /// * `error_resolver` - This is responsible for handling errors that occur during API requests.
    ///
    pub fn new(api_key : &'a str,error_resolver : &'a dyn Fn(&RequestError<Value>)) -> Self {
        MusixAbgleich { client : Client::new(),api_key : api_key,error_resolver : Box::new(error_resolver) }
    }

    async fn default_request_handler<'l,T : api_request_utils_rs::serde::de::DeserializeOwned>(&self,endpoint : &str,parameters : ParameterHashMap<'l>) -> Option<T> {
        let request = self.default_get_requestor(endpoint,parameters);
        let response = Self::request::<Value,Value>(request).await;
        let result = Self::resolve_error(&response,|value| {
            (self.error_resolver)(&value)
        });

        result.and_then(|json|{
            Some(from_value::<T>(json.get("body").unwrap().clone()).unwrap())
        })
    }
    
   //----------------------------------------------------------------- 

    /// Retrieves the top artists by country.
    ///
    /// # Arguments
    ///
    /// * `country` - A valid country code (default: "US").
    /// * `page` - The page number for paginated results.
    /// * `page_size` - The page size for paginated results. Range is 1 to 100.
    pub async fn top_artists_by_country(&self,country : Option<&str>,page : Option<u16>,page_size : Option<u8>) -> Option<Vec<Artist>> {
        let parameters = HashMap::from(
            [
                ("country",Value::from(country)),
                ("page",Value::from(page)),
                ("page_size",Value::from(page_size))
            ]
        ); 
        self.default_request_handler("chart.artists.get", parameters).await
    }

    /// Retrieves the top tracks by country.
    ///
    /// # Arguments
    ///
    /// * `country` - A valid 2-letter country code (default: "US"). Set "XW" for worldwide.
    /// * `chart_name` - Select among available charts: "top" (editorial chart), "hot" (most viewed lyrics in the last 2 hours),
    ///   "mxmweekly" (most viewed lyrics in the last 7 days), "mxmweekly_new" (most viewed lyrics in the last 7 days limited to new releases only).
    /// * `has_lyrics` - When set, filter only contents with lyrics.
    /// * `page` - The page number for paginated results.
    /// * `page_size` - The page size for paginated results. Range is 1 to 100.
    pub async fn top_tracks_by_country(&self,country : Option<&str>,chart_name : Option<Chart>,has_lyrics : Option<bool>,page : Option<u16>,page_size : Option<u8>) -> Option<Vec<Track>> {
        let parameters = HashMap::from(
            [
                ("country",Value::from(country)),
                ("chart_name",Value::from(chart_name)),
                ("f_has_lyrics",Value::from(has_lyrics)),
                ("page",Value::from(page)),
                ("page_size",Value::from(page_size))
            ]
        ); 
        self.default_request_handler("chart.tracks.get", parameters).await
    }
   
   //----------------------------------------------------------------- 

    /// Search for track in our database.
    /// 
    /// # Parameters
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
    pub async fn search_track<'l>(&self,query : TrackSearchQuery<'l>) -> Option<Track> {
        self.default_request_handler("track.search", query.0).await
    }

    /// Match a song against the Musixmatch database.
    ///
    /// In some cases, you may already have some information about the track title, artist name, album, etc.
    /// This method performs the matching process for you in a single call, taking care of the details and providing instant results.
    ///
    /// # Parameters
    ///
    /// - `title`: Optional. The song title.
    /// - `artist`: Optional. The song artist.
    /// - `album`: Optional. The song album.
    pub async fn track(&self,title : Option<&str>,artist : Option<&str>,album : Option<&str>) -> Option<Track> {
        let parameters = HashMap::from(
            [
                ("q_title",Value::from(title)),
                ("q_artist",Value::from(artist)),
                ("q_album",Value::from(album))
            ]
        ); 
        self.default_request_handler("matcher.track.get", parameters).await
    }

    /// Get track information by Musixmatch commontrack_id.
    ///
    /// # Arguments
    ///
    /// * `id` - The Musixmatch commontrack_id.
    pub async fn track_with_commontrack_id(&self, id: u32) -> Option<Track> {
        let parameters = HashMap::from( [ ("commontrack_id", Value::from(id)) ] );
        self.default_request_handler("track.get", parameters).await
    }

    /// Get track information by ISRC identifier.
    ///
    /// # Arguments
    ///
    /// * `isrc` - A valid ISRC identifier.
    pub async fn track_with_track_isrc(&self, isrc: &str) -> Option<Track> {
        let parameters = HashMap::from([("track_isrc", Value::from(isrc))]);
        self.default_request_handler("track.get", parameters).await
    }

   //----------------------------------------------------------------- 

    /// Get the lyrics for a track based on its ISRC.
    ///
    /// If you have the ISRC (International Standard Recording Code) of a track in your catalogue,
    /// you can use this method to retrieve the lyrics associated with that track.
    ///
    /// # Parameters
    ///
    /// - `isrc`: The ISRC identifier of the track.
    pub async fn track_lyrics_with_track_isrc(&self,isrc: &str) -> Option<Lyrics> {
        let parameters = HashMap::from([("track_isrc",Value::from(isrc))]);
        self.default_request_handler("matcher.lyrics.get", parameters).await
    }


    /// Get the lyrics for a track based on its title and artist.
    ///
    /// Use this method to retrieve the lyrics of a track based on its title and artist.
    ///
    /// # Parameters
    ///
    /// - `title`: Optional. The song title.
    /// - `artist`: Optional. The song artist.
    pub async fn track_lyrics(&self,title : Option<&str>,artist : Option<&str>) -> Option<Lyrics> {
        let parameters = HashMap::from(
            [
                ("q_title",Value::from(title)),
                ("q_artist",Value::from(artist)),
            ]
        ); 
        self.default_request_handler("matcher.lyrics.get", parameters).await
    }

    /// Get the lyrics of a track by Musixmatch commontrack_id.
    ///
    /// # Arguments
    ///
    /// * `id` - The Musixmatch commontrack_id.
    pub async fn track_lyrics_with_commontrack_id(&self, id: &str) -> Option<Lyrics> {
        let parameters = HashMap::from([("commontrack_id", Value::from(id))]);
        self.default_request_handler("track.lyrics.get", parameters).await
    }

    /// Get the lyrics of a track by Musixmatch track_id.
    ///
    /// # Arguments
    ///
    /// * `id` - The Musixmatch track_id.
    pub async fn track_lyrics_with_track_id(&self, id: &str) -> Option<Lyrics> {
        let parameters = HashMap::from([("track_id", Value::from(id))]);
        self.default_request_handler("track.lyrics.get", parameters).await
    }


    //-----------------------------------------------------------------  

    /// Get a translated lyrics for a given language
    ///
    /// # Arguments
    /// * `commontrack_id` : The Musixmatch commontrack id
    /// * `selected_language` : The language of the translated lyrics (ISO 639-1)  
    /// * `min_completed` : Teal from 0 to 1. If present, only the tracks with a translation ratio over this specific value, for a given language, are returned Set it to 1 for completed translation only, to 0.7 for a mimimum of 70% complete translation.
    pub async fn track_lyrics_translations_with_commontrack_id(&self, id: &str,min_completed : Option<f32> /*percent*/,selected_language : Option<&str>/* (ISO 639-1) */) -> Option<Lyrics> {
        let parameters = HashMap::from(
            [
                ("commontrack_id", Value::from(id)),
                ("min_completed", Value::from(min_completed)),
                ("selected_language", Value::from(selected_language)),

            ]
        );
        self.default_request_handler("track.lyrics.translation.get", parameters).await
    }

    /// Get a translated lyrics for a given language
    ///
    /// # Arguments
    /// * `track_id` : The Musixmatch track id
    /// * `selected_language` : The language of the translated lyrics (ISO 639-1)  
    /// * `min_completed` : Teal from 0 to 1. If present, only the tracks with a translation ratio over this specific value, for a given language, are returned Set it to 1 for completed translation only, to 0.7 for a mimimum of 70% complete translation.
    pub async fn track_lyrics_translations_with_track_id(&self, id: &str,min_completed : Option<f32> /*percent*/,selected_language : Option<&str>/* (ISO 639-1) */) -> Option<Lyrics> {
        let parameters = HashMap::from(
            [
                ("track_id", Value::from(id)),
                ("min_completed", Value::from(min_completed)),
                ("selected_language", Value::from(selected_language)),

            ]
        );
        self.default_request_handler("track.lyrics.translation.get", parameters).await
    }

    /// Get a translated lyrics for a given language
    ///
    /// # Arguments
    /// * `track_isrc` : A valid ISRC identifier
    /// * `selected_language` : The language of the translated lyrics (ISO 639-1)  
    /// * `min_completed` : Teal from 0 to 1. If present, only the tracks with a translation ratio over this specific value, for a given language, are returned Set it to 1 for completed translation only, to 0.7 for a mimimum of 70% complete translation.
    pub async fn track_lyrics_translations_with_track_irsc(&self, id: &str,min_completed : Option<f32> /*percent*/,selected_language : Option<&str>/* (ISO 639-1) */) -> Option<Lyrics> {
        let parameters = HashMap::from(
            [
                ("track_isrc", Value::from(id)),
                ("min_completed", Value::from(min_completed)),
                ("selected_language", Value::from(selected_language)),

            ]
        );
        self.default_request_handler("track.lyrics.translation.get", parameters).await
    }

    /// Get a translated lyrics for a given language
    ///
    /// # Arguments
    /// * `track_mbid` : The musicbrainz recording id
    /// * `selected_language` : The language of the translated lyrics (ISO 639-1)  
    /// * `min_completed` : Teal from 0 to 1. If present, only the tracks with a translation ratio over this specific value, for a given language, are returned Set it to 1 for completed translation only, to 0.7 for a mimimum of 70% complete translation.
    pub async fn track_lyrics_translations_with_musixbrainx_id(&self, id: &str,min_completed : Option<f32> /*percent*/,selected_language : Option<&str>/* (ISO 639-1) */) -> Option<Lyrics> {
        let parameters = HashMap::from(
            [
                ("track_mbid", Value::from(id)),
                ("min_completed", Value::from(min_completed)),
                ("selected_language", Value::from(selected_language)),

            ]
        );
        self.default_request_handler("track.lyrics.translation.get", parameters).await
    }

   //----------------------------------------------------------------- 

    /// Get the mood list (and raw value that generated it) of a lyrics by Musixmatch commontrack_id.
    ///
    /// # Arguments
    ///
    /// * `id` - The Musixmatch commontrack_id.
    pub async fn track_lyrics_mood_with_commontrack_id(&self, id: &str) -> Option<Lyrics> {
        let parameters = HashMap::from([("commontrack_id", Value::from(id))]);
        self.default_request_handler("track.lyrics.mood.get", parameters).await
    }

    /// Get the mood list (and raw value that generated it) of a lyrics by track ISRC.
    ///
    /// # Arguments
    ///
    /// * `isrc` - A valid ISRC identifier.
    pub async fn track_lyrics_mood_with_track_isrc(&self, isrc: &str) -> Option<LyricMood> {
        let parameters = HashMap::from([("track_isrc", Value::from(isrc))]);
        self.default_request_handler("track.lyrics.mood.get", parameters).await
    }

   //----------------------------------------------------------------- 

    /// Get the snippet for a given track.
    ///
    /// A lyrics snippet is a very short representation of a song lyrics.
    /// It’s usually twenty to a hundred characters long and it’s calculated
    /// by extracting a sequence of words from the lyrics.
    ///
    /// # Parameters
    ///
    /// - `track_id`: The musiXmatch track ID.
    pub async fn track_snippet(&self, track_id: u32) -> Option<Snippet> {
        let parameters = HashMap::from([("track_id", Value::from(track_id))]);
        self.default_request_handler("track.snippet.get", parameters).await
    }

   //----------------------------------------------------------------- 

    /// Retrieve the subtitle of a track.
    ///
    /// This method returns the subtitle of a track in LRC or DFXP format.
    /// You can specify the desired length of the subtitle and the maximum deviation allowed.
    ///
    /// # Parameters
    ///
    /// - `commontrack_id`: The Musixmatch commontrack id.
    /// - `subtitle_length`: Optional. The desired length of the subtitle in seconds.
    /// - `max_deviation`: Optional. The maximum deviation allowed from the desired subtitle length in seconds.
    /// - `format`: Optional. The format of the subtitle (LRC, DFXP, STLEDU). Defaults to LRC.
    pub async fn track_subtitle(&self,commontrack_id : u32,subtitle_length/*seconds*/ : Option<u16>,max_deviation : Option<u8> /*seconds*/,format : Option<SubtitleFormat>) -> Option<Subtitle> {
        let parameters = HashMap::from(
            [
                ("commontrack_id", Value::from(commontrack_id)),
                ("f_subtitle_length", Value::from(subtitle_length)),
                ("f_subtitle_length_max_deviation", Value::from(max_deviation)),
                ("subtitle_format", Value::from(format))
            ]
        );
        self.default_request_handler("tracks.subtitle.get", parameters).await
    }


    /// Get the subtitles for a song given its title, artist, and duration.
    ///
    /// You can use the `f_subtitle_length_max_deviation` parameter to fetch subtitles within a given duration range.
    ///
    /// IMPORTANT NOTICE:
    /// You have to include one of the two available tracking systems in your page/application:
    /// 1. JavaScript script for web sites
    ///    Include the URL returned in the `script_tracking_url` field as a script:
    ///    `<script type="text/javascript" src="http://tracking.musixmatch.com/t1.0/AMa6hJCIEzn1v8RuOP">`
    /// 2. Image pixel
    ///    Include the URL returned in the `pixel_tracking_url` field as an image src when it's not possible to use the script:
    ///    `<img src="http://tracking.musixmatch.com/t1.0/AMa6hJCIEzn1v8RuXW">`
    /// Furthermore, every time a subtitle is present in a page, the `lyrics_copyright` field must also be clearly visible.
    ///
    /// # Parameters
    ///
    /// - `title`: The song title.
    /// - `artist`: The song artist.
    /// - `album`: Optional. The song album.
    /// - `subtitle_length`: Optional. Filter by subtitle length in seconds.
    /// - `max_deviation`: Optional. Max deviation for a subtitle length in seconds.
    pub async fn subtitle(&self,title : Option<&str>,artist : Option<&str>,album : Option<&str>,subtitle_length/*seconds*/ : Option<u16>,max_deviation : Option<u8> /*seconds*/) -> Option<Track> {
        let parameters = HashMap::from(
            [
                ("q_title",Value::from(title)),
                ("q_artist",Value::from(artist)),
                ("q_album",Value::from(album)),
                ("f_subtitle_length", Value::from(subtitle_length)),
                ("f_subtitle_length_max_deviation", Value::from(max_deviation)),
            ]
        ); 

        self.default_request_handler("matcher.subtitle.get", parameters).await
    }

    //----------------------------------------------------------------- 
    
    /// Get a translated subtitle for a given language.
    ///
    /// # Parameters
    /// `selected_language`: The language of the translated lyrics (ISO 639-1).
    /// `min_completed`: A value between 0 and 1. If present, only the tracks with a translation ratio over this specific value, for a given language, are returned. Set it to 1 for completed translation only, or to 0.7 for a minimum of 70% complete translation.
    /// `commontrack_id`: The Musixmatch commontrack ID.
    /// `f_subtitle_length`: The desired length of the subtitle in seconds.
    /// `f_subtitle_length_max_deviation`: The maximum deviation allowed from the f_subtitle_length in seconds.
    pub async fn track_subtitle_translations_with_commontrack_id(&self, id: &str,min_completed : Option<f32> /*percent*/,selected_language : Option<&str>/* (ISO 639-1) */,subtitle_length/*seconds*/ : Option<u16>,max_deviation : Option<u8> /*seconds*/)-> Option<Subtitle> {
        let parameters = HashMap::from(
            [
                ("commontrack_id", Value::from(id)),
                ("min_completed", Value::from(min_completed)),
                ("selected_language", Value::from(selected_language)),
                ("f_subtitle_length", Value::from(subtitle_length)),
                ("f_subtitle_length_max_deviation", Value::from(max_deviation)),

            ]
        );
        self.default_request_handler::<Value>("track.subtitle.translation.get", parameters).await.and_then(|value|{
            Some(from_value::<Subtitle>(value.get("subtitle_translated").unwrap().clone()).unwrap())
        })
    }

    /// Get a translated subtitle for a given language.
    ///
    /// # Parameters
    /// `selected_language`: The language of the translated lyrics (ISO 639-1).
    /// `min_completed`: A value between 0 and 1. If present, only the tracks with a translation ratio over this specific value, for a given language, are returned. Set it to 1 for completed translation only, or to 0.7 for a minimum of 70% complete translation.
    /// `track_isrc`: A valid ISRC identifier.
    /// `f_subtitle_length`: The desired length of the subtitle in seconds.
    /// `f_subtitle_length_max_deviation`: The maximum deviation allowed from the f_subtitle_length in seconds.
    pub async fn track_subtitle_translations_with_track_isrc(&self, id: &str,min_completed : Option<f32> /*percent*/,selected_language : Option<&str>/* (ISO 639-1) */,subtitle_length/*seconds*/ : Option<u16>,max_deviation : Option<u8> /*seconds*/) -> Option<Subtitle> {
        let parameters = HashMap::from(
            [
                ("track_isrc", Value::from(id)),
                ("min_completed", Value::from(min_completed)),
                ("selected_language", Value::from(selected_language)),
                ("f_subtitle_length", Value::from(subtitle_length)),
                ("f_subtitle_length_max_deviation", Value::from(max_deviation)),
            ]
        );
        self.default_request_handler::<Value>("track.subtitle.translation.get", parameters).await.and_then(|value|{
            Some(from_value::<Subtitle>(value.get("subtitle_translated").unwrap().clone()).unwrap())
        })
    }

    //----------------------------------------------------------------- 

    /// Search for artists in our database.
    /// 
    /// # Parameters
    /// 
    /// `q_artist` : The song artist
    /// `f_artist_id` : When set, filter by this artist id
    /// `f_artist_mbid` : When set, filter by this artist musicbrainz id
    /// `page` : Define the page number for paginated results
    /// `page_size` :Define the page size for paginated results. Range is 1 to 100.
    pub async fn search_artist(&self, artist_song: Option<&str>, artist_id: Option<u32>, artist_mbid: Option<&str>, page: Option<u32>, page_size: Option<u8>) -> Option<Artist> {
        let parameters = HashMap::from(
            [
                ("q_artist", Value::from(artist_song)),
                ("f_artist_id", Value::from(artist_id)),
                ("f_artist_mbid", Value::from(artist_mbid)),
                ("page", Value::from(page)),
                ("page_size", Value::from(page_size))
            ]
        );
        self.default_request_handler("artist.search", parameters).await
    }
    

    /// Get the artist data from the Musixmatch database using the Musixmatch artist ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The Musixmatch artist ID
    pub async fn artist_with_musixmatch_id(&self,id : u32) -> Option<Artist> {
        let parameters = HashMap::from([("artist_id",Value::from(id))]);
        self.default_request_handler("artist.get", parameters).await
    }

    /// Get the artist data from the Musixmatch database using the Musicbrainz artist ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The Musicbrainz artist ID.
    pub async fn artist_with_musixbrainz_id(&self,id : u32) -> Option<Artist> {
        let parameters = HashMap::from([("artist_mbid",Value::from(id))]);
        self.default_request_handler("artist.get", parameters).await
    }

    //----------------------------------------------------------------- 

    /// Get the album discography of an artist
    /// 
    /// # Parameters
    /// 
    /// artist_id : Musixmatch artist id
    /// g_album_name : Group by Album Name
    /// s_release_date : Sort by release date (asc|desc)
    /// page : Define the page number for paginated results
    /// page_size : Define the page size for paginated results. Range is 1 to 100.
    pub async fn artist_relating_albums_with_id(&self,id:u32,album_name: Option<bool>,release_date_sort: Option<SortBy>, page: Option<u32>, page_size: Option<u8>) -> Option<Vec<Album>> {
        let parameters = HashMap::from([
            ("artist_id", Value::from(id)),
            ("g_album_name", Value::from(album_name)),
            ("s_release_date", Value::from(release_date_sort)),
            ("page", Value::from(page)),
            ("page_size", Value::from(page_size)),
        ]);
    
        self.default_request_handler("artist.albums.get", parameters).await
    }

    /// Get the album discography of an artist
    /// 
    /// # Parameters
    /// 
    /// artist_mbid : Musicbrainz artist id
    /// g_album_name : Group by Album Name
    /// s_release_date : Sort by release date (asc|desc)
    /// page : Define the page number for paginated results
    /// page_size : Define the page size for paginated results. Range is 1 to 100. 
    pub async fn artist_relating_albums_with_musixbrainz_id(&self,id:u32,album_name: Option<bool>,release_date_sort: Option<SortBy>, page: Option<u32>, page_size: Option<u8>) -> Option<Vec<Album>> {
        let parameters = HashMap::from([
            ("artist_mbid", Value::from(id)),
            ("g_album_name", Value::from(album_name)),
            ("s_release_date", Value::from(release_date_sort)),
            ("page", Value::from(page)),
            ("page_size", Value::from(page_size)),
        ]);
    
        self.default_request_handler("artist.albums.get", parameters).await
    }

    //----------------------------------------------------------------- 

    /// Get a list of artists somehow related to a given one.
    /// 
    /// # Parameters
    /// 
    /// `artist_id` : The musiXmatch artist id
    /// `page` : Define the page number for paginated results
    /// `page_size` : Define the page size for paginated results. Range is 1 to 100
    pub async fn artist_relating_artist_with_id(&self,id:u32,page: Option<u32>, page_size: Option<u8>) -> Option<Vec<Artist>> {
        let parameters = HashMap::from([
            ("artist_id", Value::from(id)),
            ("page", Value::from(page)),
            ("page_size", Value::from(page_size)),
        ]);
    
        self.default_request_handler("artist.related.get", parameters).await
    }

    /// Get a list of artists somehow related to a given one.
    /// 
    /// # Parameters
    /// 
    /// `artist_mbid` : The musicbrainz artist id
    /// `page` : Define the page number for paginated results
    /// `page_size` : Define the page size for paginated results. Range is 1 to 100
    pub async fn artist_relating_artist_with_musixbrainz_id(&self,id:u32,page: Option<u32>, page_size: Option<u8>) -> Option<Vec<Artist>> {
        let parameters = HashMap::from([
            ("artist_mbid", Value::from(id)),
            ("page", Value::from(page)),
            ("page_size", Value::from(page_size)),
        ]);
    
        self.default_request_handler("artist.related.get", parameters).await
    }

    //----------------------------------------------------------------- 

    /// Get an album from the Musixmatch database.
    ///
    /// Retrieves information about an album including its name, release date, release type,
    /// and cover art.
    ///
    /// # Parameters
    ///
    /// - `id`: The Musixmatch album ID.
    pub async fn album(&self,id : u32) -> Option<Album> {
        let parameters = HashMap::from([("album_id",Value::from(id))]);
        self.default_request_handler("album.get", parameters).await
    }

    /// This api provides you the list of the songs of an album.
    /// 
    /// # Parameters
    /// 
    /// album_id : Musixmatch album id
    /// has_lyrics :When set, filter only contents with lyrics
    /// page : Define the page number for paginated results
    /// page_size : Define the page size for paginated results. Range is 1 to 100.
    pub async fn album_tracks_with_id(&self,id: u32,has_lyrics: Option<bool>, page: Option<u32>, page_size: Option<u8>) -> Option<Vec<Track>> {
        let parameters = HashMap::from([
            ("album_id", Value::from(id)),
            ("f_has_lyrics", Value::from(has_lyrics)),
            ("page", Value::from(page)),
            ("page_size", Value::from(page_size)),
        ]);

        self.default_request_handler("album.tracks.get", parameters).await
    }

    
    /// This api provides you the list of the songs of an album.
    /// 
    /// # Parameters
    /// 
    /// album_mbid : Musicbrainz album id
    /// has_lyrics :When set, filter only contents with lyrics
    /// page : Define the page number for paginated results
    /// page_size : Define the page size for paginated results. Range is 1 to 100.
    pub async fn album_tracks_with_musixbrainz_id(&self,id: u32 ,has_lyrics: Option<bool>, page: Option<u32>, page_size: Option<u8>) -> Option<Vec<Track>> {
        let parameters = HashMap::from([
            ("album_mbid", Value::from(id)),
            ("f_has_lyrics", Value::from(has_lyrics)),
            ("page", Value::from(page)),
            ("page_size", Value::from(page_size)),
        ]);

        self.default_request_handler("album.tracks.get", parameters).await
    }


    //----------------------------------------------------------------- 

    /// Get the list of music genres in the catalogue.
    pub async fn genres(&self) -> Option<Vec<Genre>> {
        let parameters = HashMap::new();
        self.default_request_handler("music.genres.get", parameters).await
    }

    //----------------------------------------------------------------- 


    /// Get the base url for the tracking script
    /// 
    /// With this api you’ll be able to get the base url for the tracking script you need to insert in your page to legalize your existent lyrics library.
    ///
    /// Read more here: <https://developer.musixmatch.com/documentation/rights-clearance-on-your-existing-catalog>
    ///
    /// In case you’re fetching the lyrics by the musiXmatch api called track.lyrics.get you don’t need to implement this API call.
    /// # Parameters
    /// 
    /// `domain` : Your domain name
    pub async fn tracking_url(&self,domain : &str) -> Option<String> {
        let parameters = HashMap::from([("domain",Value::from(domain))]);
        self.default_request_handler::<Value>("tracking.url.get", parameters).await.and_then(|value|{
            Some(value.get("url").unwrap().as_str().unwrap().to_string())
        })
    }
}*/