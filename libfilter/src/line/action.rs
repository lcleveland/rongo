//! Actions that a line can specify

use crate::line::rgba::RGBA;
use crate::line::size::Size;
use std::fmt;

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
    PlayEffect((Color, bool)),
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
    /// No color
    None,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
            Color::Brown => write!(f, "Brown"),
            Color::White => write!(f, "White"),
            Color::Yellow => write!(f, "Yellow"),
            Color::Cyan => write!(f, "Cyan"),
            Color::Grey => write!(f, "Grey"),
            Color::Orange => write!(f, "Orange"),
            Color::Pink => write!(f, "Pink"),
            Color::Purple => write!(f, "Purple"),
            Color::None => write!(f, "None"),
        }
    }
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

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Circle => write!(f, "Circle"),
            Shape::Diamond => write!(f, "Diamond"),
            Shape::Hexagon => write!(f, "Hexagon"),
            Shape::Square => write!(f, "Square"),
            Shape::Star => write!(f, "Star"),
            Shape::Triangle => write!(f, "Triangle"),
            Shape::Cross => write!(f, "Cross"),
            Shape::Moon => write!(f, "Moon"),
            Shape::Raindrop => write!(f, "Raindrop"),
            Shape::Kite => write!(f, "Kite"),
            Shape::Pentagon => write!(f, "Pentagon"),
            Shape::UpsideDownHouse => write!(f, "UpsideDownHouse"),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::SetBorderColor(color) => write!(f, "SetBorderColor {}", color),
            Action::SetTextColor(color) => write!(f, "SetTextColor {}", color),
            Action::SetBackgroundColor(color) => write!(f, "SetBackgroundColor {}", color),
            Action::SetFontSize(size) => write!(f, "SetFontSize {}", size),
            Action::PlayAlertSound(sound) => write!(f, "PlayAlertSound {} {}", sound.0, sound.1),
            Action::PlayAlertSoundPositional(fields) => {
                write!(f, "PlayAlertSoundPositional {} {}", fields.0, fields.1)
            }
            Action::DisableDroupSound => write!(f, "DisableDroupSound"),
            Action::EnableDropSound => write!(f, "EnableDropSound"),
            Action::CustomAlertSound(sound_path) => write!(f, "CustomAlertSound {}", sound_path),
            Action::MinimapIcon(fields) => {
                write!(f, "MinimapIcon {} {} {}", fields.0, fields.1, fields.2)
            }
            Action::PlayEffect(fields) => {
                write!(f, "PlayEffect {} {}", fields.0, fields.1)
            }
        }
    }
}
