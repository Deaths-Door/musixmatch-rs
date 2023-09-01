/*
   
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
}*/