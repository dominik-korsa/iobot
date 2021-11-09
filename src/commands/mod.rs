use dialoguer::theme::ColorfulTheme;

pub mod generate;
pub mod init;
pub mod test;

pub fn get_theme() -> ColorfulTheme {
    ColorfulTheme::default()
}
