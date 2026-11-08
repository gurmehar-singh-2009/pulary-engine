#[derive(Default)]
pub struct LightTheme {
    pub background: u32     = 0xf4f3ef,
    pub foreground: u32     = 0xe8e6e1,
    pub surface_raised: u32 = 0xfbfaf8,
    pub border: u32         = 0xd3d0c9,
    pub grid: u32           = 0xdcd9d2,

    pub text_c1: u32 = 0x1f2328,
    pub text_c2: u32 = 0x4a4f57,
    pub text_c3: u32 = 0x81868e,

    pub accent: u32       = 0x3d72c4,
    pub accent_hover: u32 = 0x2f5ea8,
    pub selection: u32    = 0xcddff5,

    pub success: u32 = 0x3a8a5c,
    pub warning: u32 = 0xb5791f,
    pub error: u32   = 0xc44d4d,
    pub info: u32    = 0x2f8b9e,

    pub axis_x: u32 = 0xcf5250,
    pub axis_y: u32 = 0x5a9a3c,
    pub axis_z: u32 = 0x3d72c4,
}

#[derive(Default)]
pub struct DarkTheme {
    pub background: u32     = 0x1b1d22,
    pub foreground: u32     = 0x24272e,
    pub surface_raised: u32 = 0x2c3038,
    pub border: u32         = 0x3a3f48,
    pub grid: u32           = 0x2e3239,

    pub text_c1: u32 = 0xe2e4e8,
    pub text_c2: u32 = 0xadb2ba,
    pub text_c3: u32 = 0x7a808a,

    pub accent: u32       = 0x6f9fe8,
    pub accent_hover: u32 = 0x8cb3ef,
    pub selection: u32    = 0x2f4363,

    pub success: u32 = 0x6cbf84,
    pub warning: u32 = 0xdeae5a,
    pub error: u32   = 0xe27272,
    pub info: u32    = 0x5fb6c8,

    pub axis_x: u32 = 0xe06c6c,
    pub axis_y: u32 = 0x8cc265,
    pub axis_z: u32 = 0x6f9fe8,
}

pub type Theme = DarkTheme;
pub static THEME: Theme = Theme { .. };
