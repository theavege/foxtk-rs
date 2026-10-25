use crate::prelude::*;
use crate::{HEIGHT, cstr_lossy, to_cstring};
use foxtk_sys::*;

impl_widget!(
    StatusBar,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl StatusBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXStatusBar_new(parent.as_raw() as *mut FXComposite)) }
            .with_layout(Layout::FillX)
    }
    pub fn set_text(&self, text: &str) {
        unsafe {
            FXStatusBar_set_text(self.as_raw(), to_cstring(text).as_ptr());
        }
    }
    pub fn text(&self) -> String {
        unsafe {
            let ptr = FXStatusBar_get_text(self.as_raw());
            cstr_lossy(ptr)
        }
    }
    pub fn set_help_text(&self, text: &str) {
        unsafe {
            FXStatusBar_set_help_text(self.as_raw(), to_cstring(text).as_ptr());
        }
    }
    pub fn help_text(&self) -> String {
        unsafe {
            let ptr = FXStatusBar_get_help_text(self.as_raw());
            cstr_lossy(ptr)
        }
    }
}

impl_widget!(Label, IdExt, FrameExt, DrawableExt, WindowExt);
impl Label {
    pub fn new(parent: &impl CompositeExt, title: &str) -> Self {
        unsafe {
            Self::from_raw(FXLabel_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(&format!("&{title}")).as_ptr(),
            ))
        }
        .with_height(HEIGHT)
    }
    pub fn with_justify(self, justify: Justify) -> Self {
        unsafe {
            FXLabel_set_justify(self.as_raw(), justify as u32);
        }
        self
    }
}

impl_widget!(ProgressBar, IdExt, DrawableExt, WindowExt, FrameExt);
impl ProgressBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXProgressBar_new(parent.as_raw() as *mut FXComposite)) }
            .with_height(HEIGHT)
    }
    pub fn progress(&self) -> u32 {
        unsafe { FXProgressBar_get_progress(self.as_raw()) }
    }
    pub fn total(&self) -> u32 {
        unsafe { FXProgressBar_get_total(self.as_raw()) }
    }
    pub fn increment(&self, value: u32) {
        unsafe { FXProgressBar_increment(self.as_raw(), value) }
    }
    pub fn show_number(&self) {
        unsafe { FXProgressBar_show_number(self.as_raw()) }
    }
    pub fn hide_number(&self) {
        unsafe { FXProgressBar_hide_number(self.as_raw()) }
    }
    pub fn bar_size(&self) -> i32 {
        unsafe { FXProgressBar_get_bar_size(self.as_raw()) }
    }
    pub fn set_progress(&self, progress: u32) {
        unsafe { FXProgressBar_set_progress(self.as_raw(), progress) }
    }
    pub fn set_value(&self, value: u32) {
        self.set_progress(value);
    }
    pub fn set_total(&self, total: u32) {
        unsafe { FXProgressBar_set_total(self.as_raw(), total) }
    }
    pub fn set_bar_size(&self, size: i32) {
        unsafe { FXProgressBar_set_bar_size(self.as_raw(), size) }
    }
    pub fn with_total(self, value: u32) -> Self {
        self.set_total(value);
        self
    }
}

impl_widget!(Text, IdExt, FrameExt, DrawableExt, WindowExt);
impl Text {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXText_new(parent.as_raw() as *mut FXComposite)) }
            .with_selector(Selector::CHANGED)
            .with_layout(Layout::Fill)
    }
}

impl_widget!(TextField, IdExt, FrameExt, DrawableExt, WindowExt);
impl TextField {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXTextField_new(parent.as_raw() as *mut FXComposite)) }
            .with_selector(Selector::CHANGED)
            .with_layout(Layout::FillX)
    }
}

impl_widget!(StatusLine, IdExt, FrameExt, DrawableExt, WindowExt);
impl StatusLine {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXStatusLine_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(@as SevenSegment, FX7Segment, IdExt, FrameExt, DrawableExt, WindowExt);
impl SevenSegment {
    pub fn new(parent: &impl CompositeExt, text: &str) -> Self {
        unsafe {
            Self::from_raw(FX7Segment_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(text).as_ptr(),
            ))
        }
    }
    pub fn set_justify(&self, justify: Justify) {
        unsafe { FX7Segment_set_justify(self.as_raw(), justify as u32) }
    }
    pub fn justify(&self) -> Justify {
        crate::enums::justify_from_raw(unsafe { FX7Segment_get_justify(self.as_raw()) })
    }
}
