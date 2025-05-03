pub enum Event {
    RequestFrame(u32, u32),
    ChangeProperties(Properties),
}

pub enum Properties {
    SetFontSize(u32),
    SetTimeMilis(f64),
    SetTopPadding(u32),
    SetRowSpacing(u32),
    SetSpeedFactor(f64),

    SetMaxRows(usize),
    StartRendering(()),
    PauseRendering(()),
    SetFontName(String),
}
