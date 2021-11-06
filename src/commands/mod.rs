use dialoguer::theme::ColorfulTheme;

pub mod init;
pub mod init_prototype;

pub fn get_theme() -> ColorfulTheme {
    ColorfulTheme::default()
}
