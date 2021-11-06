use dialoguer::theme::ColorfulTheme;

pub mod generate;
pub mod init;

pub fn get_theme() -> ColorfulTheme {
    ColorfulTheme::default()
}
