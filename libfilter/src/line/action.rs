//! Actions that a line can specify

use crate::line::rgba::RGBA;

#[derive(Debug)]
/// Actions that a line can specify
pub enum Action {
    /// Set the border color
    SetBorderColor(RGBA),
    /// Set the text color
    SetTextColor(RGBA),
    /// Set the background color
    SetBackgroundColor(RGBA),
    /// Set the font size
    SetFontSize(u16),
    /// Play an alert sound
    PlayAlertSound((u8, u16)),
    /// Play an alert sound with sound volume relative to where item dropped
    PlayAlertSoundPositional((u8, u16)),
    /// Disable the drop sound
    DisableDroupSound,
    /// Enable the drop sound
    EnableDropSound,
    /// Set the custom alert sound
    CustomAlertSound(String),
    /// Set the minimap icon
    MinimapIcon((Size, Color, Shape)),
    /// Play an effect
    PlayEffect((bool, Color, bool)),
}

#[derive(Debug)]
/// The size of the minimap icon
pub enum Size {
    /// Disable the minimap icon
    Disable,
    /// Small icon
    Small,
    /// Medium icon
    Medium,
    /// Largest icon
    Largest,
}

#[derive(Debug)]
/// Predefined colors for the minimap icon and light beam
pub enum Color {
    /// Red
    Red,
    /// Green
    Green,
    /// Blue
    Blue,
    /// Brown
    Brown,
    /// White
    White,
    /// Yellow
    Yellow,
    /// Cyan
    Cyan,
    /// Grey
    Grey,
    /// Orange
    Orange,
    /// Pink
    Pink,
    /// Purple
    Purple,
}

#[derive(Debug)]
/// Predefined shapes for the minimap icon
pub enum Shape {
    /// Circle
    Circle,
    /// Diamond
    Diamond,
    /// Hexagon
    Hexagon,
    /// Square
    Square,
    /// Star
    Star,
    /// Triangle
    Triangle,
    /// Cross
    Cross,
    /// Moon
    Moon,
    /// Raindrop
    Raindrop,
    /// Kite
    Kite,
    /// Pentagon
    Pentagon,
    /// Upside down house
    UpsideDownHouse,
}
