use crate::line::rgba::RGBA;
#[derive(Debug)]
pub enum Action {
    SetBorderColor(RGBA),
    SetTextColor(RGBA),
    SetBackgroundColor(RGBA),
    SetFontSize(u16),
    PlayAlertSound((u8, u16)),
    PlayAlertSoundPositional((u8, u16)),
    DisableDroupSound,
    EnableDropSound,
    CustomAlertSound(String),
    MinimapIcon((Size, Color, Shape)),
    PlayEffect((bool, Color, bool)),
}

#[derive(Debug)]
pub enum Size {
    Disable,
    Small,
    Medium,
    Largest,
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Brown,
    White,
    Yellow,
    Cyan,
    Grey,
    Orange,
    Pink,
    Purple,
}

#[derive(Debug)]
pub enum Shape {
    Circle,
    Diamond,
    Hexagon,
    Square,
    Star,
    Triangle,
    Cross,
    Moon,
    Raindrop,
    Kite,
    Pentagon,
    UpsideDownHouse,
}
