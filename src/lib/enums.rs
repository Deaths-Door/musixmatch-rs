use strum::Display;

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