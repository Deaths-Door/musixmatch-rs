use std::collections::HashMap;

use api_request_utils_rs::{
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
    }, ParameterHashMap
};

use crate::{
    SubtitleFormat, 
    Chart,

    Artist,
    Track,
    Lyrics,
    LyricMood,
    Snippet,
    Genre,
    Subtitle
};

struct MusixAbgleich<'a> {
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

/// impl track.search / track.richsync/ track.lyrics.translation.get /track.subtitle.translation.get
impl<'a> MusixAbgleich<'a> {
    pub fn new(api_key : &'a str,error_resolver : &'a dyn Fn(&RequestError<Value>)) -> Self {
        MusixAbgleich { client : Client::new(),api_key : api_key,error_resolver : Box::new(error_resolver) }
    }

    fn body_content<'de,T : api_request_utils_rs::serde::de::DeserializeOwned>(value : &Value) -> T {
        from_value::<T>(value.get("body").unwrap()).unwrap()
    }

    async fn default_request_handler<'l,T : api_request_utils_rs::serde::de::DeserializeOwned>(&self,endpoint : &str,parameters : ParameterHashMap<'l>) -> Option<T> {
        let request = self.default_get_requestor(endpoint,parameters);
        let response = Self::request::<Value,Value>(request).await;
        let result = Self::resolve_error(&response,|value| {
            (self.error_resolver)(&value)
        });


        result.and_then(|json|{
            Some(Self::body_content(&json))
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
    ///
    /// # Example
    ///
    /// ```rust
    /// # use musixmatch::top_artists_by_country;
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    ///     let artists = top_artists_by_country(Some("IT"), Some(1), Some(3)).await;
    ///     match artists {
    ///         Some(artists) => {
    ///             for artist in artists {
    ///                 println!("{}", artist.name);
    ///             }
    ///         }
    ///         None => {
    ///             println!("No artists found.");
    ///         }
    ///     }
    /// # }
    /// ```
    pub async fn top_artists_by_country(&self,country : Option<&str>,page : Option<u16>,page_size : Option<u8>) -> Option<Vec<Artist>> {
        let parameters = HashMap::new(); //add parms //add parms
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
    ///
    /// # Example
    ///
    /// ```rust
    /// # use musixmatch::{top_tracks_by_country, Chart};
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    ///     let tracks = top_tracks_by_country(
    ///         Some("IT"),
    ///         Some(Chart::Top),
    ///         Some(true),
    ///         Some(1),
    ///         Some(5),
    ///     ).await;
    ///     match tracks {
    ///         Some(tracks) => {
    ///             for track in tracks {
    ///                 println!("{}", track.name);
    ///             }
    ///         }
    ///         None => {
    ///             println!("No tracks found.");
    ///         }
    ///     }
    /// # }
    /// ```
    pub async fn top_tracks_by_country(&self,country : Option<&str>,chart_name : Option<Chart>,has_lyrics : Option<bool>,page : Option<u16>,page_size : Option<u8>) -> Option<Vec<Track>> {
        let parameters = HashMap::new(); //add parms//add parms
        self.default_request_handler("chart.tracks.get", parameters).await
    }
   
   //----------------------------------------------------------------- 

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
    ///
    /// # Authentication
    ///
    /// This method requires authentication.
    ///
    /// # Examples
    ///
    /// Match a song by title and artist:
    /// ```
    /// # use your_crate::MusixmatchAPI;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixmatchAPI::new("YOUR_API_KEY");
    /// let track = api.track(Some("Lose Yourself"), Some("Eminem"), None).await;
    /// if let Some(track) = track {
    ///     println!("Matched Track: {} by {}", track.title, track.artist_name);
    /// } else {
    ///     println!("No match found.");
    /// }
    /// # }
    /// ```
    pub async fn track(&self,title : Option<&str>,artist : Option<&str>,album : Option<&str>) -> Option<Track> {
        let parameters = HashMap::new(); //add parms
        self.default_request_handler("matcher.track.get", parameters).await
    }

    /// Get track information by Musixmatch commontrack_id.
    ///
    /// # Arguments
    ///
    /// * `id` - The Musixmatch commontrack_id.
    ///
    /// # Example
    ///
    /// ```rust
    /// let id = 5920049;
    /// let track = musixmatch.track_with_commontrack_id(id).await;
    /// ```
    pub async fn track_with_commontrack_id(&self, id: u32) -> Option<Track> {
        let id_string = id.to_string();
        let parameters = HashMap::from([("commontrack_id", Some(id_string.as_str()))]);
        self.default_request_handler("track.get", parameters).await
    }

    /// Get track information by ISRC identifier.
    ///
    /// # Arguments
    ///
    /// * `isrc` - A valid ISRC identifier.
    ///
    /// # Example
    ///
    /// ```rust
    /// let isrc = "USABC1234567";
    /// let track = musixmatch.track_with_track_isrc(isrc).await;
    /// ```
    pub async fn track_with_track_isrc(&self, isrc: &str) -> Option<Track> {
        let parameters = HashMap::from([("track_isrc", Some(isrc))]);
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
    ///
    /// # Examples
    ///
    /// Get the lyrics for a track by its ISRC:
    /// ```
    /// # use your_crate::MusixmatchAPI;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixmatchAPI::new("YOUR_API_KEY");
    /// let lyrics = api.track_lyrics_with_track_isrc("USAT21812220").await;
    /// if let Some(lyrics) = lyrics {
    ///     println!("Lyrics: {}", lyrics.lyrics_body);
    /// } else {
    ///     println!("No lyrics found.");
    /// }
    /// # }
    /// ```
    pub async fn track_lyrics_with_track_isrc(&self,isrc: &str) -> Option<Lyrics> {
        let parameters = HashMap::from([("track_isrc", Some(isrc))]);
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
    ///
    /// # Examples
    ///
    /// Get the lyrics for a track by title and artist:
    /// ```
    /// # use your_crate::MusixmatchAPI;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixmatchAPI::new("YOUR_API_KEY");
    /// let lyrics = api.track_lyrics(Some("Sexy and I Know It"), Some("LMFAO")).await;
    /// if let Some(lyrics) = lyrics {
    ///     println!("Lyrics: {}", lyrics.lyrics_body);
    /// } else {
    ///     println!("No lyrics found.");
    /// }
    /// # }
    /// ```
    pub async fn track_lyrics(&self,title : Option<&str>,artist : Option<&str>) -> Option<Lyrics> {
        let parameters = HashMap::new(); //add parms
        self.default_request_handler("matcher.lyrics.get", parameters).await
    }

    /// Get the lyrics of a track by Musixmatch commontrack_id.
    ///
    /// # Arguments
    ///
    /// * `id` - The Musixmatch commontrack_id.
    ///
    /// # Example
    ///
    /// ```rust
    /// let id = "5920049";
    /// let lyrics = musixmatch.track_lyrics_with_commontrack_id(id).await;
    /// ```
    pub async fn track_lyrics_with_commontrack_id(&self, id: &str) -> Option<Lyrics> {
        let parameters = HashMap::from([("commontrack_id", Some(id))]);
        self.default_request_handler("track.lyrics.get", parameters).await
    }

    /// Get the lyrics of a track by Musixmatch track_id.
    ///
    /// # Arguments
    ///
    /// * `id` - The Musixmatch track_id.
    ///
    /// # Example
    ///
    /// ```rust
    /// let id = "15953433";
    /// let lyrics = musixmatch.track_lyrics_with_track_id(id).await;
    /// ```
    pub async fn track_lyrics_with_track_id(&self, id: &str) -> Option<Lyrics> {
        let parameters = HashMap::from([("track_id", Some(id))]);
        self.default_request_handler("track.lyrics.get", parameters).await
    }

   //----------------------------------------------------------------- 

    /// Get the mood list (and raw value that generated it) of a lyrics by Musixmatch commontrack_id.
    ///
    /// # Arguments
    ///
    /// * `id` - The Musixmatch commontrack_id.
    ///
    /// # Example
    ///
    /// ```rust
    /// let id = "5920049";
    /// let mood = musixmatch.track_lyrics_mood_with_commontrack_id(id).await;
    /// ```
    pub async fn track_lyrics_mood_with_commontrack_id(&self, id: &str) -> Option<Lyrics> {
        let parameters = HashMap::from([("commontrack_id", Some(id))]);
        self.default_request_handler("track.lyrics.mood.get", parameters).await
    }

    /// Get the mood list (and raw value that generated it) of a lyrics by track ISRC.
    ///
    /// # Arguments
    ///
    /// * `isrc` - A valid ISRC identifier.
    ///
    /// # Example
    ///
    /// ```rust
    /// let isrc = "US123456789";
    /// let mood = musixmatch.track_lyrics_mood_with_track_isrc(isrc).await;
    /// ```
    pub async fn track_lyrics_mood_with_track_isrc(&self, isrc: &str) -> Option<LyricMood> {
        let parameters = HashMap::from([("track_isrc", Some(isrc))]);
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use musixmatch::MusixMatch;
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// #     let musixmatch = MusixMatch::new("your_api_key");
    /// #
    /// #     let track_id = 16860631;
    /// #     let snippet = musixmatch.track_snippet(track_id).await;
    /// #     println!("{:?}", snippet);
    /// # }
    /// ```
    pub async fn track_snippet(&self, track_id: u32) -> Option<Snippet> {
        let id_string = track_id.to_string();
        let parameters = HashMap::from([("track_id", Some(id_string.as_str()))]);
        self.default_request_handler("track.snippet.get", parameters).await
    }

    pub async fn track_subtitle(&self,commontrack_id : u32,subtitle_length/*seconds*/ : Option<u16>,max_deviation : Option<u8> /*seconds*/,format : Option<SubtitleFormat>) -> Option<Subtitle> {
        let parameters = HashMap::from([]);
        self.default_request_handler("tracks.subtitle.get", parameters).await
    }

    pub async fn genres(&self) -> Option<Vec<Genre>> {
        let parameters = HashMap::new(); //add parms
        self.default_request_handler("music.genres.get", parameters).await
    }

    pub async fn subtitle(&self,title : Option<&str>,artist : Option<&str>,album : Option<&str>,subtitle_length/*seconds*/ : Option<u16>,max_deviation : Option<u8> /*seconds*/) -> Option<Track> {
        let parameters = HashMap::new(); //add parms
        self.default_request_handler("matcher.subtitle.get", parameters).await
    }

}