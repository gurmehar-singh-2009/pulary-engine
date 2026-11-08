#[derive(Default)]
pub struct LightTheme {
    pub background: u32 = 0xffffff,
}

#[derive(Default)]
pub struct DarkTheme {}

pub type Theme = LightTheme;
pub static THEME: Theme = Theme { .. };
