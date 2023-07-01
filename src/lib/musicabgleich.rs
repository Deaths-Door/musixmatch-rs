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
    },
    ParameterHashMap
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
    Subtitle,
    Album
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

/// impl track.search / track.richsync/ track.lyrics.translation.get /track.subtitle.translation.get / artist.search / artist.albums.get / artist.related.get / album-tracks-get / catalogue.dump.get / work.post / work.validity.post / tracking.url.get
impl<'a> MusixAbgleich<'a> {
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

    fn insert<'l,T : ToString>(parameters : &mut ParameterHashMap<'l>,name : &'l str,value : Option<T>){
        value.map(|it| {
            let string = it.to_string();
            parameters.insert(name,Some(string.as_str()))
        });        
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
        let mut parameters = HashMap::new(); 
        parameters.insert("country",country);
        Self::insert(&mut parameters, "page", page);
        Self::insert(&mut parameters, "page_size", page_size);

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
        let mut parameters = HashMap::new();
        parameters.insert("country",country);
        Self::insert(&mut parameters, "chart_name", chart_name);
        Self::insert(&mut parameters, "f_has_lyrics", has_lyrics);
        Self::insert(&mut parameters, "page", page);
        Self::insert(&mut parameters, "page_size", page_size);

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
    /// # use musixmatch::MusixAbgleich;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixAbgleich::new("YOUR_API_KEY");
    /// let track = api.track(Some("Lose Yourself"), Some("Eminem"), None).await;
    /// if let Some(track) = track {
    ///     println!("Matched Track: {} by {}", track.title, track.artist_name);
    /// } else {
    ///     println!("No match found.");
    /// }
    /// # }
    /// ```
    pub async fn track(&self,title : Option<&str>,artist : Option<&str>,album : Option<&str>) -> Option<Track> {
        let mut parameters = HashMap::new(); 
        parameters.insert("q_title",title);
        parameters.insert("q_artist",artist);
        parameters.insert("q_album",album);
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
        let string = id.to_string();
        let parameters = HashMap::from([("commontrack_id", Some(string.as_str()))]);
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
    /// # use musixmatch::MusixAbgleich;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixAbgleich::new("YOUR_API_KEY");
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
    /// # use musixmatch::MusixAbgleich;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixAbgleich::new("YOUR_API_KEY");
    /// let lyrics = api.track_lyrics(Some("Sexy and I Know It"), Some("LMFAO")).await;
    /// if let Some(lyrics) = lyrics {
    ///     println!("Lyrics: {}", lyrics.lyrics_body);
    /// } else {
    ///     println!("No lyrics found.");
    /// }
    /// # }
    /// ```
    pub async fn track_lyrics(&self,title : Option<&str>,artist : Option<&str>) -> Option<Lyrics> {
        let mut parameters = HashMap::new(); 
        parameters.insert("q_title",title);
        parameters.insert("q_artist",artist);
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
        let string = track_id.to_string();
        let parameters = HashMap::from([("track_id", Some(string.as_str()))]);
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
    ///
    /// # Authentication
    ///
    /// This method requires authentication with an API key and a commercial plan.
    ///
    /// # Examples
    ///
    /// Get a subtitle in LRC format for a track:
    /// ```
    /// # use musixmatch::MusixAbgleich;
    /// # use musixmatch::SubtitleFormat;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixAbgleich::new("YOUR_API_KEY");
    /// let subtitle = api.track_subtitle(10074988, Some(120), Some(5), Some(SubtitleFormat::LRC)).await;
    /// if let Some(subtitle) = subtitle {
    ///     println!("Subtitle: {}", subtitle.subtitle_body);
    /// } else {
    ///     println!("No subtitle found.");
    /// }
    /// # }
    /// ```
    pub async fn track_subtitle(&self,commontrack_id : u32,subtitle_length/*seconds*/ : Option<u16>,max_deviation : Option<u8> /*seconds*/,format : Option<SubtitleFormat>) -> Option<Subtitle> {
        let mut parameters = HashMap::new();
        let string = commontrack_id.to_string();
        parameters.insert("commontrack_id", Some(string.as_str()));
        Self::insert(&mut parameters, "f_subtitle_length", subtitle_length);
        Self::insert(&mut parameters, "f_subtitle_length_max_deviation", max_deviation);
        Self::insert(&mut parameters, "subtitle_format", format);

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
    ///
    /// # Authentication
    ///
    /// This method requires authentication with an API key and a commercial plan.
    ///
    /// # Examples
    ///
    /// Get the subtitles for a song:
    /// ```
    /// # use musixmatch::MusixAbgleich;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixAbgleich::new("YOUR_API_KEY");
    /// let subtitles = api.subtitle(
    ///     Some("sexy and i know it"),
    ///     Some("lmfao"),
    ///     None,
    ///     Some(200),
    ///     Some(3),
    /// ).await;
    /// if let Some(subtitles) = subtitles {
    ///     for subtitle in subtitles {
    ///         println!("Subtitle: {}", subtitle.subtitle_body);
    ///     }
    /// } else {
    ///     println!("No subtitles found.");
    /// }
    /// # }
    /// ```
    pub async fn subtitle(&self,title : Option<&str>,artist : Option<&str>,album : Option<&str>,subtitle_length/*seconds*/ : Option<u16>,max_deviation : Option<u8> /*seconds*/) -> Option<Track> {
        let mut parameters = HashMap::new();
        parameters.insert("q_title",title);
        parameters.insert("q_artist",artist);
        parameters.insert("q_album",album);
        
        Self::insert(&mut parameters, "f_subtitle_length", subtitle_length);
        Self::insert(&mut parameters, "f_subtitle_length_max_deviation", max_deviation);

        self.default_request_handler("matcher.subtitle.get", parameters).await
    }

    //----------------------------------------------------------------- 

    /// Get the list of music genres in the catalogue.
    ///
    /// # Authentication
    ///
    /// This method requires authentication.
    ///
    /// # Examples
    ///
    /// Get the list of all genres:
    /// ```
    /// # use musixmatch::MusixAbgleich;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixAbgleich::new("YOUR_API_KEY");
    /// let genres = api.genres().await;
    /// if let Some(genres) = genres {
    ///     for genre in genres {
    ///         println!("Genre: {}", genre.genre_name);
    ///     }
    /// } else {
    ///     println!("No genres found.");
    /// }
    /// # }
    /// ```
    pub async fn genres(&self) -> Option<Vec<Genre>> {
        let parameters = HashMap::new();
        self.default_request_handler("music.genres.get", parameters).await
    }

    //----------------------------------------------------------------- 

    /// Get the artist data from the Musixmatch database using the Musixmatch artist ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The Musixmatch artist ID
    ///
    /// # Examples
    ///
    /// Get the artist data for the artist with Musixmatch ID 118:
    /// ```
    /// # use musixmatch::MusixAbgleich;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixAbgleich::new("YOUR_API_KEY");
    /// let artist_data = api.artist_with_musixmatch_id(118).await;
    /// if let Some(artist_data) = artist_data {
    ///     // Process artist data
    ///     println!("Artist Data: {:?}", artist_data);
    /// } else {
    ///     println!("Artist data not found.");
    /// }
    /// # }
    /// ```
    pub async fn artist_with_musixmatch_id(&self,id : u32) -> Option<Artist> {
        let string = id.to_string();
        let parameters = HashMap::from([("artist_id",Some(string.as_str()))]);
        self.default_request_handler("artist.get", parameters).await
    }

    /// Get the artist data from the Musixmatch database using the Musicbrainz artist ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The Musicbrainz artist ID.
    ///
    /// # Examples
    ///
    /// Get the artist data for the artist with Musicbrainz ID 118:
    /// ```
    /// # use musixmatch::MusixAbgleich;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixAbgleich::new("YOUR_API_KEY");
    /// let artist_data = api.artist_with_musixbrainz_id(118).await;
    /// if let Some(artist_data) = artist_data {
    ///     // Process artist data
    ///     println!("Artist Data: {:?}", artist_data);
    /// } else {
    ///     println!("Artist data not found.");
    /// }
    /// # }
    /// ```
    pub async fn artist_with_musixbrainz_id(&self,id : u32) -> Option<Artist> {
        let string = id.to_string();
        let parameters = HashMap::from([("artist_mbid",Some(string.as_str()))]);
        self.default_request_handler("artist.get", parameters).await
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
    ///
    /// # Examples
    ///
    /// Get the album data for the album with Musixmatch ID 14250417:
    /// ```
    /// # use musixmatch::MusixAbgleich;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api = MusixAbgleich::new("YOUR_API_KEY");
    /// let album_data = api.album(14250417).await;
    /// if let Some(album_data) = album_data {
    ///     // Process album data
    ///     println!("Album Data: {:?}", album_data);
    /// } else {
    ///     println!("Album data not found.");
    /// }
    /// # }
    /// ```
    pub async fn album(&self,id : u32) -> Option<Album> {
        let string = id.to_string();
        let parameters = HashMap::from([("album_id",Some(string.as_str()))]);
        self.default_request_handler("album.get", parameters).await
    }

    //----------------------------------------------------------------- 
}