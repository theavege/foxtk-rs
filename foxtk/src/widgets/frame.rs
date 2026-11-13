use crate::prelude::*;
use crate::{HEIGHT, to_cstring};
use foxtk_sys::*;

impl_widget!(
    GroupBox,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    PackerExt,
    CompositeExt
);
impl GroupBox {
    pub fn new(parent: &impl CompositeExt, title_: &str) -> Self {
        unsafe {
            Self::from_raw(FXGroupBox_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(title_).as_ptr(),
            ))
        }
        .with_frame(FrameStyle::Thick)
        .with_layout(Layout::FillX)
    }
}

impl_widget!(
    VerticalFrame,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl VerticalFrame {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXVerticalFrame_new(parent.as_raw() as *mut FXComposite)) }
            .with_layout(Layout::FillX)
            .with_frame(FrameStyle::Thick)
    }
}

impl_widget!(
    HorizontalFrame,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl HorizontalFrame {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXHorizontalFrame_new(parent.as_raw() as *mut FXComposite)) }
            .with_height(HEIGHT)
            .with_frame(FrameStyle::Thick)
    }
}

impl_widget!(
    Matrix,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl Matrix {
    pub fn new(parent: &impl CompositeExt, rows: i32) -> Self {
        unsafe {
            Self::from_raw(FXMatrix_new(
                parent.as_raw() as *mut FXComposite,
                rows,
                MatrixStyle::ByRows as u32,
            ))
        }
        .with_layout(Layout::Fill)
    }
    pub fn set_num_rows(&self, rows: i32) {
        unsafe {
            FXMatrix_set_num_rows(self.as_raw(), rows);
        }
    }
    pub fn set_num_columns(&self, cols: i32) {
        unsafe {
            FXMatrix_set_num_columns(self.as_raw(), cols);
        }
    }
    pub fn num_rows(&self) -> i32 {
        unsafe { FXMatrix_get_num_rows(self.as_raw()) }
    }
    pub fn num_columns(&self) -> i32 {
        unsafe { FXMatrix_get_num_columns(self.as_raw()) }
    }
}

impl_widget!(
    Splitter,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl Splitter {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe {
            Self::from_raw(FXSplitter_new(
                parent.as_raw() as *mut FXComposite,
                SplitterStyle::Normal as u32,
            ))
        }
        .with_layout(Layout::Fill)
    }
    pub fn with_style(self, style: SplitterStyle) -> Self {
        self.set_style(style);
        self
    }
    pub fn set_style(&self, style: SplitterStyle) {
        unsafe {
            FXSplitter_set_style(self.as_raw(), style as u32);
        }
    }
    pub fn style(&self) -> SplitterStyle {
        crate::enums::splitter_style_from_raw(unsafe { FXSplitter_get_style(self.as_raw()) as u32 })
    }
    pub fn set_split(&self, index: i32, size: i32) {
        unsafe {
            FXSplitter_set_split(self.as_raw(), index, size);
        }
    }
    pub fn split(&self, index: i32) -> i32 {
        unsafe { FXSplitter_get_split(self.as_raw(), index) }
    }
    pub fn set_bar_size(&self, size: i32) {
        unsafe {
            FXSplitter_set_bar_size(self.as_raw(), size);
        }
    }
    pub fn bar_size(&self) -> i32 {
        unsafe { FXSplitter_get_bar_size(self.as_raw()) }
    }
}

impl_widget!(ScrollBar, IdExt, FrameExt, DrawableExt, WindowExt);
impl ScrollBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXScrollBar_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn position(&self) -> i32 {
        unsafe { FXScrollBar_get_position(self.as_raw()) }
    }
    pub fn set_position(&self, pos: i32) {
        unsafe {
            FXScrollBar_set_position(self.as_raw(), pos);
        }
    }
    pub fn set_range(&self, hi: i32) {
        unsafe {
            FXScrollBar_set_range(self.as_raw(), hi);
        }
    }
}

impl_widget!(
    Packer,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl Packer {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXPacker_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(Spring, IdExt, FrameExt, DrawableExt, WindowExt);
impl Spring {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXSpring_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    Composite,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);

impl_widget!(@as FourSplitter, FX4Splitter, IdExt, FrameExt, DrawableExt, WindowExt, CompositeExt);
impl FourSplitter {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FX4Splitter_new(parent.as_raw() as *mut FXComposite)) }
            .with_layout(Layout::Fill)
    }
    pub fn horizontal_split(&self) -> i32 {
        unsafe { FX4Splitter_get_h_split(self.as_raw()) }
    }
    pub fn vertical_split(&self) -> i32 {
        unsafe { FX4Splitter_get_v_split(self.as_raw()) }
    }
    pub fn set_horizontal_split(&self, split: i32) {
        unsafe { FX4Splitter_set_h_split(self.as_raw(), split) }
    }
    pub fn set_vertical_split(&self, split: i32) {
        unsafe { FX4Splitter_set_v_split(self.as_raw(), split) }
    }
    pub fn set_style(&self, style: u32) {
        unsafe { FX4Splitter_set_style(self.as_raw(), style) }
    }
    pub fn bar_size(&self) -> i32 {
        unsafe { FX4Splitter_get_bar_size(self.as_raw()) }
    }
    pub fn set_bar_size(&self, size: i32) {
        unsafe { FX4Splitter_set_bar_size(self.as_raw(), size) }
    }
}

impl_widget!(HorizontalSeparator, IdExt, FrameExt, DrawableExt, WindowExt);
impl HorizontalSeparator {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe {
            Self::from_raw(FXHorizontalSeparator_new(
                parent.as_raw() as *mut FXComposite
            ))
        }
    }
}

impl_widget!(Separator, IdExt, FrameExt, DrawableExt, WindowExt);
impl Separator {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXSeparator_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn set_style(&self, style: u32) {
        unsafe { FXSeparator_set_style(self.as_raw(), style) }
    }
    pub fn style(&self) -> u32 {
        unsafe { FXSeparator_get_style(self.as_raw()) }
    }
}

impl_widget!(VerticalSeparator, IdExt, FrameExt, DrawableExt, WindowExt);
impl VerticalSeparator {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXVerticalSeparator_new(parent.as_raw() as *mut FXComposite)) }
    }
}
