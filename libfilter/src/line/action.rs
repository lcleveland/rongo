//! Actions that a line can specify

use crate::line::color::Color;
use crate::line::rgba::RGBA;
use crate::line::shape::Shape;
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
