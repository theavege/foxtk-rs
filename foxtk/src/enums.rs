pub enum ButtonState {
    Up = 0,
    Down,
    Engaged,
}

pub enum ButtonStyle {
    Default = 0x04000000,
    Autogray = 0x00800000,
    Autohide = 0x01000000,
    Toolbar = 0x02000000,
    Initial = 0x08000000,
}

pub enum Message {
    Error = 0,
    Warning,
    Question,
    Information,
}

pub enum MessageBox {
    Ok = 0x10000000,
    OkCancel = 0x20000000,
}

pub enum Justify {
    Normal = 0,
    Left = 0x00008000,
    Right = 0x00010000,
    Top = 0x00020000,
    Bottom = 0x00040000,
    HzApart = 0x00008000 | 0x00010000,
    VtApart = 0x00020000 | 0x00040000,
}

pub enum Layout {
    Normal = 0,
    FillX = 0x00000400,
    FillY = 0x00000800,
    Fill = 0x00000400 | 0x00000800,
    FixWidth = 0x00000100,
    FixHeight = 0x00000200,
}

pub enum FrameStyle {
    None = 0,
    Sunken = 0x00001000,
    Raised = 0x00002000,
    Thick = 0x00004000,
    Ridge = 0x00004000 | 0x00002000 | 0x00001000,
    Line = 0x00002000 | 0x00001000,
    Normal = 0x00000800 | 0x00004000,
}

pub enum Selector {
    COMMAND = 0,
    CHANGED,
}

pub struct Color(u32);
impl Color {
    pub fn from_rgb(r: u32, g: u32, b: u32) -> Self {
        Self(unsafe { foxtk_sys::fx_rgb(r, g, b) })
    }
    pub fn from_rgba(r: u32, g: u32, b: u32, a: u32) -> Self {
        Self(unsafe { foxtk_sys::fx_rgba(r, g, b, a) })
    }
    pub fn from_hex(hex: u32) -> Self {
        Self::from_rgb((hex >> 16) & 0xff, (hex >> 8) & 0xff, hex & 0xff)
    }
    pub fn bits(&self) -> u32 {
        self.0
    }
}
