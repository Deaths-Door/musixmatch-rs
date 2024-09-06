/// Search among writers
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct QueryWriter<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for QueryWriter<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueryWriter<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<QueryWriter<'_>> for api_request_utils::serde_json::Value {
    fn from(value: QueryWriter<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for QueryWriter<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for QueryWriter<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl QueryWriter<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parameters: &mut crate::parmeters::Parameters<'l>) {
        parameters.insert("q_writer", self.into());
    }
}
/// Any word in the song title or artist name or lyrics
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct QueryAll<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for QueryAll<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueryAll<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<QueryAll<'_>> for api_request_utils::serde_json::Value {
    fn from(value: QueryAll<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for QueryAll<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for QueryAll<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl QueryAll<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parameters: &mut crate::parmeters::Parameters<'l>) {
        parameters.insert("q", self.into());
    }
}
/// Any word in the lyrics
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct QueryLyrics<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for QueryLyrics<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueryLyrics<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<QueryLyrics<'_>> for api_request_utils::serde_json::Value {
    fn from(value: QueryLyrics<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for QueryLyrics<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for QueryLyrics<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl QueryLyrics<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parameters: &mut crate::parmeters::Parameters<'l>) {
        parameters.insert("q_lyrics", self.into());
    }
}
/// The song album
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct QueryAlbum<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for QueryAlbum<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueryAlbum<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<QueryAlbum<'_>> for api_request_utils::serde_json::Value {
    fn from(value: QueryAlbum<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for QueryAlbum<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for QueryAlbum<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl QueryAlbum<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parameters: &mut crate::parmeters::Parameters<'l>) {
        parameters.insert("q_album", self.into());
    }
}
/// The song title
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct QueryTrack<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for QueryTrack<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueryTrack<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<QueryTrack<'_>> for api_request_utils::serde_json::Value {
    fn from(value: QueryTrack<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for QueryTrack<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for QueryTrack<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl QueryTrack<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parameters: &mut crate::parmeters::Parameters<'l>) {
        parameters.insert("q_track", self.into());
    }
}
/// The song artist
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct QueryArtist<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for QueryArtist<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueryArtist<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<QueryArtist<'_>> for api_request_utils::serde_json::Value {
    fn from(value: QueryArtist<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for QueryArtist<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for QueryArtist<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl QueryArtist<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parameters: &mut crate::parmeters::Parameters<'l>) {
        parameters.insert("q_artist", self.into());
    }
}
/// Any word in the song title or artist name
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct QueryTrackArtist<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for QueryTrackArtist<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueryTrackArtist<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<QueryTrackArtist<'_>> for api_request_utils::serde_json::Value {
    fn from(value: QueryTrackArtist<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for QueryTrackArtist<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for QueryTrackArtist<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}

impl QueryTrackArtist<'_> {
    pub(crate) fn add_to_hashmap<'l>(self, parameters: &mut crate::parmeters::Parameters<'l>) {
        parameters.insert("q_track_artist", self.into());
    }
}
