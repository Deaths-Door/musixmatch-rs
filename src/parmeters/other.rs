use api_request_utils::serde_json::Value;

use super::OutOfRangeError;

/// A valid country code (default US)
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct Country<'a>(std::option::Option<&'a str>);

impl Default for Country<'_> {
    fn default() -> Self {
        Self(Some("US"))
    }
}

impl<'a> std::ops::Deref for Country<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for Country<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Country<'_>> for api_request_utils::serde_json::Value {
    fn from(value: Country<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for Country<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for Country<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl Country<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("country", self.into());
    }
}

/// Your domain name
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct Domain<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for Domain<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for Domain<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Domain<'_>> for api_request_utils::serde_json::Value {
    fn from(value: Domain<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for Domain<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for Domain<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl Domain<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("domain", self.into());
    }
}

/// A valid ISRC identifier
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct TrackIsrc<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for TrackIsrc<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for TrackIsrc<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TrackIsrc<'_>> for api_request_utils::serde_json::Value {
    fn from(value: TrackIsrc<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for TrackIsrc<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for TrackIsrc<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl TrackIsrc<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("track_isrc", self.into());
    }
}

/// Teal from 0 to 1. If present, only the tracks with a translation ratio over this specific value, for a given language, are returned Set it to 1 for completed translation only, to 0.7 for a mimimum of 70% complete translation.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct MinCompleted(std::option::Option<f32>);

impl std::ops::Deref for MinCompleted {
    type Target = std::option::Option<f32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for MinCompleted {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<f32> for MinCompleted {
    type Error = OutOfRangeError<f32>;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(Some(value))
    }
}
impl TryFrom<Option<f32>> for MinCompleted {
    type Error = OutOfRangeError<f32>;
    fn try_from(value: Option<f32>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(Self(None)),
            Some(i) => match i >= 0.0 && i <= 1.0 {
                true => Ok(Self(Some(i))),
                false => Err(OutOfRangeError::new(
                    i,
                    "MinCompleted is not in allowed range (0 to 1)",
                )),
            },
        }
    }
}

impl From<MinCompleted> for Value {
    fn from(value: MinCompleted) -> Self {
        value.0.into()
    }
}

impl MinCompleted {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("min_completed", self.into());
    }
}

/// The Musixmatch commontrack id
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct CommontrackId(u32);

impl std::ops::Deref for CommontrackId {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for CommontrackId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u32> for CommontrackId {
    fn from(value: u32) -> Self {
        Self(value.into())
    }
}

impl From<u16> for CommontrackId {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl From<u8> for CommontrackId {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<CommontrackId> for Value {
    fn from(value: CommontrackId) -> Self {
        value.0.into()
    }
}

impl CommontrackId {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("commontrack_id", self.into());
    }
}
/// Musicbrainz album id
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct AlbumMbid(u32);

impl std::ops::Deref for AlbumMbid {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for AlbumMbid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u32> for AlbumMbid {
    fn from(value: u32) -> Self {
        Self(value.into())
    }
}

impl From<u16> for AlbumMbid {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl From<u8> for AlbumMbid {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<AlbumMbid> for Value {
    fn from(value: AlbumMbid) -> Self {
        value.0.into()
    }
}

impl AlbumMbid {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("album_mbid", self.into());
    }
}
/// Define the page number for paginated results
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct Page(u32);

impl std::ops::Deref for Page {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Page {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u32> for Page {
    fn from(value: u32) -> Self {
        Self(value as u32)
    }
}

impl From<u16> for Page {
    fn from(value: u16) -> Self {
        Self(value as u32)
    }
}

impl From<u8> for Page {
    fn from(value: u8) -> Self {
        Self(value as u32)
    }
}

impl From<Page> for Value {
    fn from(value: Page) -> Self {
        value.0.into()
    }
}

impl Page {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("page", self.into());
    }
}
/// The musiXmatch album id
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct AlbumId(u32);

impl std::ops::Deref for AlbumId {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for AlbumId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u32> for AlbumId {
    fn from(value: u32) -> Self {
        Self(value.into())
    }
}

impl From<u16> for AlbumId {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl From<u8> for AlbumId {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<AlbumId> for Value {
    fn from(value: AlbumId) -> Self {
        value.0.into()
    }
}

impl AlbumId {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("album_id", self.into());
    }
}
/// The musicbrainz recording id
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct TrackMbid(u32);

impl std::ops::Deref for TrackMbid {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for TrackMbid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u32> for TrackMbid {
    fn from(value: u32) -> Self {
        Self(value.into())
    }
}

impl From<u16> for TrackMbid {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl From<u8> for TrackMbid {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<TrackMbid> for Value {
    fn from(value: TrackMbid) -> Self {
        value.0.into()
    }
}

impl TrackMbid {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("track_mbid", self.into());
    }
}
/// The Musixmatch track id
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct TrackId(u32);

impl std::ops::Deref for TrackId {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for TrackId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u32> for TrackId {
    fn from(value: u32) -> Self {
        Self(value.into())
    }
}

impl From<u16> for TrackId {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl From<u8> for TrackId {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<TrackId> for Value {
    fn from(value: TrackId) -> Self {
        value.0.into()
    }
}

impl TrackId {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("track_id", self.into());
    }
}
/// Define the page size for paginated results. Range is 1 to 100.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct PageSize(u32);

impl std::ops::Deref for PageSize {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for PageSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<u32> for PageSize {
    type Error = OutOfRangeError<u32>;
    fn try_from(i: u32) -> Result<Self, Self::Error> {
        match (1..100).contains(&i) {
            true => Ok(Self(i)),
            false => Err(OutOfRangeError::new(
                i,
                "PageSize is not in allowed range (1 to 100).",
            )),
        }
    }
}

impl TryFrom<u16> for PageSize {
    type Error = OutOfRangeError<u16>;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(value as u32).map_err(|e| OutOfRangeError::new(e.value as u16, e.message))
    }
}

impl TryFrom<u8> for PageSize {
    type Error = OutOfRangeError<u8>;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(value as u32).map_err(|e| OutOfRangeError::new(e.value as u8, e.message))
    }
}

impl From<PageSize> for Value {
    fn from(value: PageSize) -> Self {
        value.0.into()
    }
}

impl PageSize {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("page_size", self.into());
    }
}
/// Search only a part of the given query string.Allowed range is (0.1 – 0.9)
#[derive(Clone, PartialEq, Debug, Default)]
pub struct QuorumFactor(std::option::Option<f32>);

impl std::ops::Deref for QuorumFactor {
    type Target = std::option::Option<f32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for QuorumFactor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<f32> for QuorumFactor {
    type Error = OutOfRangeError<f32>;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(Some(value))
    }
}
impl TryFrom<Option<f32>> for QuorumFactor {
    type Error = OutOfRangeError<f32>;
    fn try_from(value: Option<f32>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(Self(None)),
            Some(i) => match i >= 0.1 && i <= 0.9 {
                true => Ok(Self(Some(i))),
                false => Err(OutOfRangeError::new(
                    i,
                    "QuorumFactor is not in allowed range (0.1 – 0.9)",
                )),
            },
        }
    }
}

impl From<QuorumFactor> for Value {
    fn from(value: QuorumFactor) -> Self {
        value.0.into()
    }
}

impl QuorumFactor {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("quorum_factor", self.into());
    }
}
/// The language of the translated lyrics (ISO 639-1)
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct SelectedLanguage<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for SelectedLanguage<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SelectedLanguage<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<SelectedLanguage<'_>> for api_request_utils::serde_json::Value {
    fn from(value: SelectedLanguage<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for SelectedLanguage<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for SelectedLanguage<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl SelectedLanguage<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("selected_language", self.into());
    }
}
/// The lyrics
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct LyricsBody<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for LyricsBody<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for LyricsBody<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<LyricsBody<'_>> for api_request_utils::serde_json::Value {
    fn from(value: LyricsBody<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for LyricsBody<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for LyricsBody<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl LyricsBody<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("lyrics_body", self.into());
    }
}
/// Musixmatch artist id
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct ArtistId(u32);

impl std::ops::Deref for ArtistId {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for ArtistId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u32> for ArtistId {
    fn from(value: u32) -> Self {
        Self(value.into())
    }
}

impl From<u16> for ArtistId {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl From<u8> for ArtistId {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<ArtistId> for Value {
    fn from(value: ArtistId) -> Self {
        value.0.into()
    }
}
impl ArtistId {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("artist_id", self.into());
    }
}
/// Musicbrainz artist id
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct ArtistMbid(u32);

impl std::ops::Deref for ArtistMbid {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for ArtistMbid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u32> for ArtistMbid {
    fn from(value: u32) -> Self {
        Self(value.into())
    }
}

impl From<u16> for ArtistMbid {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl From<u8> for ArtistMbid {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<ArtistMbid> for Value {
    fn from(value: ArtistMbid) -> Self {
        value.0.into()
    }
}

impl ArtistMbid {
    pub(crate) fn add_to_hashmap<'l>(self, parmeters: &mut crate::parmeters::Parameters<'l>) {
        parmeters.insert("artist_mbid", self.into());
    }
}
