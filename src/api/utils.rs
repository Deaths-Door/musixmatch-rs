use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use api_request_utils::serde_json::Value;

#[derive(Default)]
pub(in crate::api) struct Parameters<'a>(std::collections::HashMap<&'a str, Value>);

impl<'a> Deref for Parameters<'a> {
    type Target = HashMap<&'a str, Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Parameters<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Modified  from https://users.rust-lang.org/t/optional-generics-in-struct-creating-macro/53526/2
#[allow(unused_macros)]
macro_rules! create_struct {
    ($($name:ident $(<$($generics:tt),*>)? {$field_type:ty} ),*) => {
        $(
            pub struct $name $(<$($generics),*>)? (pub $field_type);
        )*
    };
}

macro_rules! create_parameters {
    ($($name:ident $(<$($generics:tt),*>)? {$field_type:ty} => $key:expr),*) => {
        $(
            create_parameters!(define => $name $(<$($generics),*>)? {$field_type});
            create_parameters!(hashmap => $name $(<$($generics),*>)? => $key);
            create_parameters!(from => $name $(<$($generics),*>)?);
        )*
    };


    (define =>$($name:ident $(<$($generics:tt),*>)? {$field_type:ty}),*) => {
        $(
            pub struct $name $(<$($generics),*>)? (pub $field_type);
        )*
    };


    // The main difference is the addition of trait bounds , and a custom from
    (time => $($name:ident,$key: expr),*) => {
        $(
            pub struct $name<T>(T) where T : chrono::Datelike;

            impl<T> $name::<T> where T :chrono::Datelike  {
                pub(in crate::api) fn add_to_hashmap<'l>(self, parameters: &mut crate::api::Parameters<'l>) {
                  parameters.insert($key, Value::from(self));
                }
            }

            impl<T> From<$name::<T>> for Value  where T :chrono::Datelike  {
                fn from(value: $name <T>) -> Self {
                    // Requires in this format YYYYMMDD
                    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7a0b5d480219a061a29c97bbbb8e666a
                    format!("{:04}{:02}{:02}",value.0.year(),value.0.month(),(value.0.day() as usize)).into()
                }
            }
        )*
    };

    (hashmap => $($name:ident $(<$($generics:tt),*>)?),* => $key:expr) => {
        $(
            impl$(<$($generics),*>)? $name $(<$($generics),*>)? {
                pub(in crate::api) fn add_to_hashmap<'l>(self, parameters: &mut crate::api::Parameters<'l>) {
                  parameters.insert($key, Value::from(self));
                }
            }
        )*
    };

    (from => $($name:ident $(<$($generics:tt),*>)?),*) => {
        $(
            impl $(<$($generics),*>)? From<$name $(<$($generics),*>)?> for Value  {
                fn from(value: $name $(<$($generics),*>)?) -> Self {
                    value.0.into()
                }
            }
        )*
    };
}

pub(in crate::api) use create_parameters;

use super::SortBy;

// TODO; implement from inner value and intergers for these structs , and default for the Option<..> ones
create_parameters! {
    Page { u16 } => "page",
    // TODO: Required range is 1..100
    PageSize { u8 } => "page_size",
    Country<'a> { Option<&'a str> } => "country",

    QueryTrack<'a> { Option<&'a str> } => "q_track",
    QueryArtist<'a> { Option<&'a str> } => "q_artist",
    QueryLyrics<'a> { Option<&'a str> } => "q_lyrics",
    QueryTrackArtist<'a> { Option<&'a str> } => "q_track_artist",

    QueryWriter<'a> { Option<&'a str>} => "q_writer",
    QueryAny<'a> { Option<&'a str> } => "q",

    FilterHasLyrics { Option<bool> } => "f_has_lyrics",
    FilterArtistId { Option<u32> } => "f_artist_id",
    FilterMusicGenre { Option<u32> } => "f_music_genre_id",
    FilterLyricsLanguage<'a> { Option<&'a str> } => "f_lyrics_language",

    SortArtistRating { SortBy } => "s_artist_rating",
    SortTrackRating { SortBy } => "s_track_rating"
}

create_parameters! { time =>
    FilterReleaseGroupFirstReleaseDateMin , "f_track_release_group_first_release_date_min",
    FilterReleaseGroupFirstReleaseDateMax , "f_track_release_group_first_release_date_max"
}

// TODO: Allowed range is (0.1 - 0.9).
create_parameters!(define =>  QueryQuoromFactor { Option<f32> });
create_parameters!(from =>  QueryQuoromFactor);

pub enum TrackId<'a> {
    // TODO; implement from inner value and intergers for these structs , and default for the Option<..> ones
    Common(u32),
    Isrc(&'a str),
}

impl TrackId<'_> {
    pub(in crate::api) fn add_to_hashmap<'a>(self, parameters: &mut Parameters<'a>) {
        match self {
            TrackId::Common(id) => {
                parameters.0.insert("commontrack_id", Value::from(id));
            }
            TrackId::Isrc(id) => {
                parameters.0.insert("track_isrc", Value::from(id));
            }
        };
    }
}

impl<'a> From<&'a str> for TrackId<'a> {
    fn from(value: &'a str) -> Self {
        Self::Isrc(value)
    }
}

/*


pub struct TrackTitle<'a>(pub Option<&'a str>);
pub struct TrackArtist<'a>(pub Option<&'a str>);

macro_rules! impl_from {
    (ints for $name : ty,$($i : ty),*) => {
        $(
            impl From<$i> for $name {
                fn from(value: $i) -> Self {
                    Self(value.into())
                }
            }
        )*
    };

    (enum ints for $name : ty, $variant : ident,$($i : ty),*) => {
        $(
            impl From<$i> for $name {
                fn from(value: $i) -> Self {
                    Self:: $variant (value.into())
                }
            }
        )*
    };
}

add_to_hashmap!(Page, "page");
add_to_hashmap!(PageSize, "page_size");
add_to_hashmap!(Country<'_>, "country");
add_to_hashmap!(FilterHasLyrics, "f_has_lyrics");
add_to_hashmap!(Chart, "chart_name");

impl_from!(Page, PageSize, Country<'_>, FilterHasLyrics);
impl_from!(ints for Page,u8,u16);
impl_from!(ints for PageSize, u8);
impl_from!(enum ints for TrackId<'_>,Common, u8,u16,u32);


*/
