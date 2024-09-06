use api_request_utils::serde_json::Value;

/// Sort results in either Ascending or Desecending order
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, strum::Display)]
pub enum SortBy {
    ///
    #[strum(serialize = "asc")]
    Ascending,
    ///
    #[strum(serialize = "desc")]
    Desecending,
}

/// Sort by our popularity index for tracks (asc|desc)
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct SortTrackRating(crate::parmeters::sorting::SortBy);

impl std::ops::Deref for SortTrackRating {
    type Target = crate::parmeters::sorting::SortBy;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for SortTrackRating {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<crate::parmeters::sorting::SortBy> for SortTrackRating {
    fn from(value: crate::parmeters::sorting::SortBy) -> Self {
        Self(value)
    }
}

impl From<SortTrackRating> for api_request_utils::serde_json::Value {
    fn from(value: SortTrackRating) -> Self {
        Value::from(value.to_string())
    }
}

/// Sort by release date (asc|desc)
#[derive(Clone, PartialEq, Eq, Debug, Hash)]

pub struct SortReleaseDate(crate::parmeters::sorting::SortBy);

impl std::ops::Deref for SortReleaseDate {
    type Target = crate::parmeters::sorting::SortBy;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for SortReleaseDate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<crate::parmeters::sorting::SortBy> for SortReleaseDate {
    fn from(value: crate::parmeters::sorting::SortBy) -> Self {
        Self(value)
    }
}

impl From<SortReleaseDate> for api_request_utils::serde_json::Value {
    fn from(value: SortReleaseDate) -> Self {
        Value::from(value.to_string())
    }
}
