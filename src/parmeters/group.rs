use api_request_utils::serde_json::Value;

/// Group by Album Name
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct GroupAlbumName<'a>(std::option::Option<&'a str>);

impl<'a> std::ops::Deref for GroupAlbumName<'a> {
    type Target = std::option::Option<&'a str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for GroupAlbumName<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<GroupAlbumName<'_>> for Value {
    fn from(value: GroupAlbumName<'_>) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for GroupAlbumName<'a> {
    fn from(value: &'a str) -> Self {
        Self::from(Some(value))
    }
}
impl<'a> From<Option<&'a str>> for GroupAlbumName<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self(value)
    }
}
