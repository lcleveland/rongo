#[derive(Debug)]
pub enum Action {
    SetBorderColor,
    SetTextColor,
    SetBackgroundColor,
    SetFontSize,
    PlayAlertSound,
    PlayAlertSoundPositional,
    DisableDroupSound,
    EnableDropSound,
    CustomAlertSound,
    MinimapIcon,
    PlayEffect,
}
