/// The desired length of the subtitle (seconds)
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct FilterSubtitleLength(std::option::Option<std::time::Duration>);

impl std::ops::Deref for FilterSubtitleLength {
    type Target = std::option::Option<std::time::Duration>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for FilterSubtitleLength {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<std::time::Duration> for FilterSubtitleLength {
    fn from(value: std::time::Duration) -> Self {
        Self(Some(value))
    }
}

impl From<Option<std::time::Duration>> for FilterSubtitleLength {
    fn from(value: Option<std::time::Duration>) -> Self {
        Self(value)
    }
}
impl From<FilterSubtitleLength> for api_request_utils::serde_json::Value {
    fn from(value: FilterSubtitleLength) -> Self {
        value.0.map(|v| v.as_secs()).into()
    }
}

impl FilterSubtitleLength {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("f_subtitle_length", self.into());
    }
}
/// When set, filter only contents with lyrics
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct FilterHasLyrics(std::option::Option<bool>);

impl std::ops::Deref for FilterHasLyrics {
    type Target = std::option::Option<bool>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for FilterHasLyrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FilterHasLyrics> for api_request_utils::serde_json::Value {
    fn from(value: FilterHasLyrics) -> Self {
        value.0.into()
    }
}

impl From<bool> for FilterHasLyrics {
    fn from(value: bool) -> Self {
        Self(Some(value))
    }
}

impl From<Option<bool>> for FilterHasLyrics {
    fn from(value: Option<bool>) -> Self {
        Self(value)
    }
}

impl FilterHasLyrics {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("f_has_lyrics", self.into());
    }
}
/// When set, filter by this artist id
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct FilterArtistId(std::option::Option<u32>);

impl std::ops::Deref for FilterArtistId {
    type Target = std::option::Option<u32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for FilterArtistId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FilterArtistId> for api_request_utils::serde_json::Value {
    fn from(value: FilterArtistId) -> Self {
        value.0.into()
    }
}

impl From<u32> for FilterArtistId {
    fn from(value: u32) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<u32>> for FilterArtistId {
    fn from(value: Option<u32>) -> Self {
        Self(value.map(|v| v.into()))
    }
}

impl From<u16> for FilterArtistId {
    fn from(value: u16) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<u16>> for FilterArtistId {
    fn from(value: Option<u16>) -> Self {
        Self(value.map(|v| v.into()))
    }
}

impl From<u8> for FilterArtistId {
    fn from(value: u8) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<u8>> for FilterArtistId {
    fn from(value: Option<u8>) -> Self {
        Self(value.map(|v| v.into()))
    }
}

impl FilterArtistId {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("f_artist_id", self.into());
    }
}
/// When set, filter the tracks with release date older than value, format is YYYYMMDD
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct FilterTrackReleaseGroupFirstReleaseDateMax<T: chrono::Datelike>(std::option::Option<T>);

impl<T: chrono::Datelike> std::ops::Deref for FilterTrackReleaseGroupFirstReleaseDateMax<T> {
    type Target = std::option::Option<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: chrono::Datelike> std::ops::DerefMut for FilterTrackReleaseGroupFirstReleaseDateMax<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: chrono::Datelike> From<T> for FilterTrackReleaseGroupFirstReleaseDateMax<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl<T: chrono::Datelike> From<Option<T>> for FilterTrackReleaseGroupFirstReleaseDateMax<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}
impl<T: chrono::Datelike> From<FilterTrackReleaseGroupFirstReleaseDateMax<T>>
    for api_request_utils::serde_json::Value
{
    fn from(value: FilterTrackReleaseGroupFirstReleaseDateMax<T>) -> Self {
        value
            .0
            .map(|value| {
                // Requires in this format YYYYMMDD
                // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7a0b5d480219a061a29c97bbbb8e666a
                format!(
                    "{:04}{:02}{:02}",
                    value.year(),
                    value.month(),
                    (value.day() as usize)
                )
            })
            .into()
    }
}

impl<T: chrono::Datelike> FilterTrackReleaseGroupFirstReleaseDateMax<T> {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("f_track_release_group_first_release_date_max", self.into());
    }
}
/// When set, filter by this music category id
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct FilterMusicGenreId(std::option::Option<u32>);

impl std::ops::Deref for FilterMusicGenreId {
    type Target = std::option::Option<u32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for FilterMusicGenreId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FilterMusicGenreId> for api_request_utils::serde_json::Value {
    fn from(value: FilterMusicGenreId) -> Self {
        value.0.into()
    }
}

impl From<u32> for FilterMusicGenreId {
    fn from(value: u32) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<u32>> for FilterMusicGenreId {
    fn from(value: Option<u32>) -> Self {
        Self(value.map(|v| v.into()))
    }
}

impl From<u16> for FilterMusicGenreId {
    fn from(value: u16) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<u16>> for FilterMusicGenreId {
    fn from(value: Option<u16>) -> Self {
        Self(value.map(|v| v.into()))
    }
}

impl From<u8> for FilterMusicGenreId {
    fn from(value: u8) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<u8>> for FilterMusicGenreId {
    fn from(value: Option<u8>) -> Self {
        Self(value.map(|v| v.into()))
    }
}

impl FilterMusicGenreId {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("f_music_genre_id", self.into());
    }
}
/// The maximum deviation allowed from the f_subtitle_length (seconds)
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct FilterSubtitleLengthMaxDeviation(std::option::Option<std::time::Duration>);

impl std::ops::Deref for FilterSubtitleLengthMaxDeviation {
    type Target = std::option::Option<std::time::Duration>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for FilterSubtitleLengthMaxDeviation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<std::time::Duration> for FilterSubtitleLengthMaxDeviation {
    fn from(value: std::time::Duration) -> Self {
        Self(Some(value))
    }
}

impl From<Option<std::time::Duration>> for FilterSubtitleLengthMaxDeviation {
    fn from(value: Option<std::time::Duration>) -> Self {
        Self(value)
    }
}
impl From<FilterSubtitleLengthMaxDeviation> for api_request_utils::serde_json::Value {
    fn from(value: FilterSubtitleLengthMaxDeviation) -> Self {
        value.0.map(|v| v.as_secs()).into()
    }
}

impl FilterSubtitleLengthMaxDeviation {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("f_subtitle_length_max_deviation", self.into());
    }
}
/// The desired length of the sync (seconds)
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct FilterRichsyncLength(std::option::Option<std::time::Duration>);

impl std::ops::Deref for FilterRichsyncLength {
    type Target = std::option::Option<std::time::Duration>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for FilterRichsyncLength {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<std::time::Duration> for FilterRichsyncLength {
    fn from(value: std::time::Duration) -> Self {
        Self(Some(value))
    }
}

impl From<Option<std::time::Duration>> for FilterRichsyncLength {
    fn from(value: Option<std::time::Duration>) -> Self {
        Self(value)
    }
}
impl From<FilterRichsyncLength> for api_request_utils::serde_json::Value {
    fn from(value: FilterRichsyncLength) -> Self {
        value.0.map(|v| v.as_secs()).into()
    }
}

impl FilterRichsyncLength {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("f_richsync_length", self.into());
    }
}
/// When set, filter the tracks with release date newer than value, format is YYYYMMDD
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct FilterTrackReleaseGroupFirstReleaseDateMin<T: chrono::Datelike>(std::option::Option<T>);

impl<T: chrono::Datelike> std::ops::Deref for FilterTrackReleaseGroupFirstReleaseDateMin<T> {
    type Target = std::option::Option<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: chrono::Datelike> std::ops::DerefMut for FilterTrackReleaseGroupFirstReleaseDateMin<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: chrono::Datelike> From<T> for FilterTrackReleaseGroupFirstReleaseDateMin<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl<T: chrono::Datelike> From<Option<T>> for FilterTrackReleaseGroupFirstReleaseDateMin<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}
impl<T: chrono::Datelike> From<FilterTrackReleaseGroupFirstReleaseDateMin<T>>
    for api_request_utils::serde_json::Value
{
    fn from(value: FilterTrackReleaseGroupFirstReleaseDateMin<T>) -> Self {
        value
            .0
            .map(|value| {
                // Requires in this format YYYYMMDD
                // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7a0b5d480219a061a29c97bbbb8e666a
                format!(
                    "{:04}{:02}{:02}",
                    value.year(),
                    value.month(),
                    (value.day() as usize)
                )
            })
            .into()
    }
}

impl<T: chrono::Datelike> FilterTrackReleaseGroupFirstReleaseDateMin<T> {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("f_track_release_group_first_release_date_min", self.into());
    }
}
/// The maximum deviation allowed from the f_sync_length (seconds)
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct FilterRichsyncLengthMaxDeviation(std::option::Option<std::time::Duration>);

impl std::ops::Deref for FilterRichsyncLengthMaxDeviation {
    type Target = std::option::Option<std::time::Duration>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for FilterRichsyncLengthMaxDeviation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<std::time::Duration> for FilterRichsyncLengthMaxDeviation {
    fn from(value: std::time::Duration) -> Self {
        Self(Some(value))
    }
}

impl From<Option<std::time::Duration>> for FilterRichsyncLengthMaxDeviation {
    fn from(value: Option<std::time::Duration>) -> Self {
        Self(value)
    }
}
impl From<FilterRichsyncLengthMaxDeviation> for api_request_utils::serde_json::Value {
    fn from(value: FilterRichsyncLengthMaxDeviation) -> Self {
        value.0.map(|v| v.as_secs()).into()
    }
}

impl FilterRichsyncLengthMaxDeviation {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("f_richsync_length_max_deviation", self.into());
    }
}
/// Filter by the lyrics language (en,it,..)
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct FilterLyricsLanguage<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for FilterLyricsLanguage<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FilterLyricsLanguage<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FilterLyricsLanguage<'_>> for api_request_utils::serde_json::Value {
    fn from(value: FilterLyricsLanguage<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for FilterLyricsLanguage<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for FilterLyricsLanguage<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl FilterLyricsLanguage<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("f_lyrics_language", self.into());
    }
}
/// When set, filter by this artist musicbrainz id
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct FilterArtistMbid(std::option::Option<u32>);

impl std::ops::Deref for FilterArtistMbid {
    type Target = std::option::Option<u32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for FilterArtistMbid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FilterArtistMbid> for api_request_utils::serde_json::Value {
    fn from(value: FilterArtistMbid) -> Self {
        value.0.into()
    }
}

impl From<u32> for FilterArtistMbid {
    fn from(value: u32) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<u32>> for FilterArtistMbid {
    fn from(value: Option<u32>) -> Self {
        Self(value.map(|v| v.into()))
    }
}

impl From<u16> for FilterArtistMbid {
    fn from(value: u16) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<u16>> for FilterArtistMbid {
    fn from(value: Option<u16>) -> Self {
        Self(value.map(|v| v.into()))
    }
}

impl From<u8> for FilterArtistMbid {
    fn from(value: u8) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<u8>> for FilterArtistMbid {
    fn from(value: Option<u8>) -> Self {
        Self(value.map(|v| v.into()))
    }
}

impl FilterArtistMbid {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("f_artist_mbid", self.into());
    }
}
