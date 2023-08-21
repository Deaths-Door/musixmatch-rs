use strum::Display;
use api_request_utils::serde_json::Value;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[derive(Display)]
pub enum Chart {
    #[strum(serialize = "top")]
    TopEditorialChart,
    #[strum(serialize = "hot")]
    HotMostViewedLyricsLast2Hours,
    #[strum(serialize = "mxmweekly")]
    MxmWeeklyMostViewedLyricsLast7Days,
    #[strum(serialize = "mxmweekly_new")]
    MxmWeeklyNewMostViewedLyricsLast7DaysNewReleasesOnly,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[derive(Display)]
pub enum SubtitleFormat {
    #[strum(serialize = "lrc")]
    Lrc,
    #[strum(serialize = "dfxp")]
    Dfxp,
    #[strum(serialize = "stledu")]
    Stledu,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[derive(Display)]
pub enum SortBy {
    #[strum(serialize = "asc")]
    Ascending,
    #[strum(serialize = "desc")]
    Desecending,
}

impl From<Chart> for Value {
    fn from(format: Chart) -> Self {
        Value::from(format.to_string())
    }
}

impl From<SubtitleFormat> for Value {
    fn from(format: SubtitleFormat) -> Self {
        Value::from(format.to_string())
    }
}

impl From<SortBy> for Value {
    fn from(format: SortBy) -> Self {
        Value::from(format.to_string())
    }
}