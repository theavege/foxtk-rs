#[repr(C)]
#[derive(Default)]
pub enum ButtonState {
    #[default]
    Up = 0,
    Down,
    Engaged,
}

#[repr(C)]
#[derive(Default)]
pub enum ListStyle {
    #[default]
    Extended = 0,
    Single = 0x00100000,
    Browse = 0x00200000,
    Multiple = 0x00300000,
    Auto = 0x00400000,
}

#[repr(C)]
#[derive(Default)]
pub enum ButtonStyle {
    #[default]
    Default = 0x04000000,
    Autogray = 0x00800000,
    Autohide = 0x01000000,
    Toolbar = 0x02000000,
    Initial = 0x08000000,
}

#[repr(C)]
#[derive(Default)]
pub enum GroupBoxStyle {
    #[default]
    Left = 0,
    Center = 0x00020000,
    Right = 0x00040000,
}

#[repr(C)]
#[derive(Default)]
pub enum Message {
    Error = 0,
    Warning,
    Question,
    #[default]
    Information,
}

#[repr(C)]
#[derive(Default)]
pub enum MessageBox {
    #[default]
    Ok = 0x10000000,
    OkCancel = 0x20000000,
}

#[repr(C)]
#[derive(Default)]
pub enum Justify {
    #[default]
    Normal = 0,
    Left = 0x00008000,
    Right = 0x00010000,
    Top = 0x00020000,
    Bottom = 0x00040000,
    HzApart = 0x00008000 | 0x00010000,
    VtApart = 0x00020000 | 0x00040000,
}

pub(crate) fn justify_from_raw(value: u32) -> Justify {
    match value {
        x if x == Justify::Left as u32 => Justify::Left,
        x if x == Justify::Right as u32 => Justify::Right,
        x if x == Justify::Top as u32 => Justify::Top,
        x if x == Justify::Bottom as u32 => Justify::Bottom,
        x if x == Justify::HzApart as u32 => Justify::HzApart,
        x if x == Justify::VtApart as u32 => Justify::VtApart,
        _ => Justify::Normal,
    }
}

pub(crate) fn splitter_style_from_raw(value: u32) -> SplitterStyle {
    match value {
        x if x == SplitterStyle::Vertical as u32 => SplitterStyle::Vertical,
        x if x == SplitterStyle::Reversed as u32 => SplitterStyle::Reversed,
        x if x == SplitterStyle::Tracking as u32 => SplitterStyle::Tracking,
        x if x == SplitterStyle::Normal as u32 => SplitterStyle::Normal,
        _ => SplitterStyle::Horizontal,
    }
}

#[repr(C)]
#[derive(Default)]
pub enum Decor {
    #[default]
    NONE = 0,
    Title = 0x00020000,
    Minimize = 0x00040000,
    Maximize = 0x00080000,
    Close = 0x00100000,
    Border = 0x00200000,
    Shrinkable = 0x00400000,
    Stretchable = 0x00800000,
    Resize = 0x00400000 | 0x00800000,
    Menu = 0x01000000,
    All = 0x00020000
        | 0x00040000
        | 0x00080000
        | 0x00100000
        | 0x00200000
        | 0x00400000
        | 0x00800000
        | 0x01000000,
}

#[repr(C)]
#[derive(Default)]
pub enum Layout {
    #[default]
    Normal = 0,
    FillX = 0x00000400,
    FillY = 0x00000800,
    Fill = 0x00000400 | 0x00000800,
    FixWidth = 0x00000100,
    FixHeight = 0x00000200,
}

#[repr(C)]
#[derive(Default)]
pub enum FrameStyle {
    #[default]
    None = 0,
    Sunken = 0x00001000,
    Raised = 0x00002000,
    Thick = 0x00004000,
    Ridge = 0x00004000 | 0x00002000 | 0x00001000,
    Line = 0x00002000 | 0x00001000,
    Normal = 0x00000800 | 0x00004000,
}

#[repr(C)]
#[derive(Default)]
pub enum MatrixStyle {
    #[default]
    ByRows = 0,
    ByColumns = 0x00020000,
}

#[repr(C)]
#[derive(Default)]
pub enum ProgressBarStyle {
    #[default]
    Horizontal = 0,
    Vertical = 0x00008000,
    Percentage = 0x00010000,
    Dial = 0x00020000,
    Normal = 0x00040000,
}

#[repr(C)]
#[derive(Default)]
pub enum SplitterStyle {
    #[default]
    Horizontal = 0,
    Vertical = 0x00008000,
    Reversed = 0x00010000,
    Tracking = 0x00020000,
    Normal = 0x00040000,
}

#[repr(C)]
#[derive(Default)]
pub enum Selector {
    #[default]
    COMMAND = 0,
    CHANGED,
}

#[repr(C)]
#[derive(Default)]
pub enum DialogBoxStyle {
    #[default]
    Normal = 0,
    Modal = 0x00000001,
    Resizable = 0x00000002,
    Minimize = 0x00000004,
    Maximize = 0x00000008,
}

#[repr(C)]
#[derive(Default)]
pub enum TabBookStyle {
    #[default]
    Normal = 0,
    BottomTabs = 0x00008000,
    LeftTabs = 0x00010000,
    RightTabs = 0x00020000,
}

#[repr(C)]
#[derive(Default)]
pub enum RulerOrientation {
    #[default]
    Horizontal = 0,
    Vertical = 1,
}

#[derive(Default)]
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
