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
    ///
    /// `color` is the color of the border
    /// Example:
    /// ```
    /// # use libfilter::line::action::Action;
    /// # use libfilter::RGBA;
    /// let set_border_color = Action::SetBorderColor(RGBA{r: 255, g: 0, b: 0, a: 255});
    /// # assert_eq!(set_border_color.to_string(), "SetBorderColor 255 0 0 255");
    /// ```
    SetBorderColor(RGBA),

    /// Set the text color
    ///
    /// `color` is the color of the text
    /// Example:
    /// ```
    /// # use libfilter::line::action::Action;
    /// # use libfilter::RGBA;
    /// let set_text_color = Action::SetTextColor(RGBA{r: 255, g: 0, b: 0, a: 255});
    /// # assert_eq!(set_text_color.to_string(), "SetTextColor 255 0 0 255");
    /// ```
    SetTextColor(RGBA),

    /// Set the background color
    ///
    /// `color` is the color of the background
    /// Example:    
    /// ```
    /// # use libfilter::line::action::Action;
    /// # use libfilter::RGBA;
    /// let set_background_color = Action::SetBackgroundColor(RGBA{r: 255, g: 0, b: 0, a: 255});
    /// # assert_eq!(set_background_color.to_string(), "SetBackgroundColor 255 0 0 255");
    /// ```
    SetBackgroundColor(RGBA),

    /// Set the font size
    ///
    /// `size` is the font size
    /// Example:
    /// ```
    /// # use libfilter::line::action::Action;
    /// let set_font_size = Action::SetFontSize(16);
    /// # assert_eq!(set_font_size.to_string(), "SetFontSize 16");
    /// ```
    SetFontSize(u16),

    /// Play an alert sound
    ///
    /// `u8` is the sound id
    /// `u16` is the sound volume
    /// Example:
    /// ```
    /// # use libfilter::line::action::Action;
    /// let play_alert_sound = Action::PlayAlertSound((1, 100));
    /// # assert_eq!(play_alert_sound.to_string(), "PlayAlertSound 1 100");
    /// ```
    PlayAlertSound((u8, u16)),

    /// Play an alert sound with sound volume relative to where item dropped
    ///
    /// `u8` is the sound id
    /// `u16` is the sound volume
    /// Example:
    /// ```
    /// # use libfilter::line::action::Action;
    /// let play_alert_sound = Action::PlayAlertSoundPositional((1, 100));
    /// # assert_eq!(play_alert_sound.to_string(), "PlayAlertSoundPositional 1 100");
    /// ```
    PlayAlertSoundPositional((u8, u16)),

    /// Disable the drop sound
    ///
    /// Example:
    /// ```
    /// # use libfilter::line::action::Action;
    /// let disable_drop_sound = Action::DisableDroupSound;
    /// # assert_eq!(disable_drop_sound.to_string(), "DisableDroupSound");
    /// ```
    DisableDroupSound,

    /// Enable the drop sound
    ///
    /// Example:
    /// ```
    /// # use libfilter::line::action::Action;
    /// let enable_drop_sound = Action::EnableDropSound;
    /// # assert_eq!(enable_drop_sound.to_string(), "EnableDropSound");
    /// ```
    EnableDropSound,

    /// Set the custom alert sound
    ///
    /// `String` is the path to the sound
    /// Example:
    /// ```
    /// # use libfilter::line::action::Action;
    /// let custom_alert_sound = Action::CustomAlertSound(String::from("/path/to/sound.wav"));
    /// # assert_eq!(custom_alert_sound.to_string(), "CustomAlertSound \"/path/to/sound.wav\"");
    /// ```
    CustomAlertSound(String),

    /// Set the minimap icon
    ///
    /// `Size` is the size
    /// `Color` is the color
    /// `Shape` is the shape
    /// Example:
    /// ```
    /// # use libfilter::line::action::Action;
    /// # use libfilter::Size;
    /// # use libfilter::Color;
    /// # use libfilter::Shape;
    /// let minimap_icon = Action::MinimapIcon((Size::Small, Color::Red, Shape::Circle));
    /// # assert_eq!(minimap_icon.to_string(), "MinimapIcon Small Red Circle");
    /// ```
    MinimapIcon((Size, Color, Shape)),

    /// Play an effect
    ///
    /// `Color` is the color
    /// `bool` is whether the beam is temporary
    /// Example:
    /// ```
    /// # use libfilter::line::action::Action;
    /// # use libfilter::Color;
    /// let play_effect = Action::PlayEffect((Color::Red, true));
    /// # assert_eq!(play_effect.to_string(), "PlayEffect Red true");
    /// ```
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
            Action::CustomAlertSound(sound_path) => {
                write!(f, "CustomAlertSound \"{}\"", sound_path)
            }
            Action::MinimapIcon(fields) => {
                write!(f, "MinimapIcon {} {} {}", fields.0, fields.1, fields.2)
            }
            Action::PlayEffect(fields) => {
                write!(f, "PlayEffect {} {}", fields.0, fields.1)
            }
        }
    }
}
