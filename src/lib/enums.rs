use strum::Display;
use api_request_utils_rs::serde_json::Value;

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

#[derive(Display)]
pub enum SubtitleFormat {
    #[strum(serialize = "lrc")]
    Lrc,
    #[strum(serialize = "dfxp")]
    Dfxp,
    #[strum(serialize = "stledu")]
    Stledu,
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