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
    pub fn set_icon(&self, icon: &super::Icon) {
        unsafe { FXLabel_set_icon(self.as_raw(), icon.as_raw()) }
    }
    pub fn with_icon(self, icon: &super::Icon) -> Self {
        self.set_icon(icon);
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
    pub fn length(&self) -> i32 {
        unsafe { FXText_length(self.as_raw()) }
    }
    pub fn cursor_pos(&self) -> i32 {
        unsafe { FXText_cursor_pos(self.as_raw()) }
    }
    pub fn cursor_row(&self) -> i32 {
        unsafe { FXText_cursor_row(self.as_raw()) }
    }
    pub fn cursor_column(&self) -> i32 {
        unsafe { FXText_cursor_column(self.as_raw()) }
    }
    pub fn is_modified(&self) -> bool {
        unsafe { FXText_is_modified(self.as_raw()) != 0 }
    }
    pub fn set_modified(&self, modified: bool) {
        unsafe { FXText_set_modified(self.as_raw(), modified as u8) }
    }
    pub fn set_tab_columns(&self, columns: i32) {
        unsafe { FXText_set_tab_columns(self.as_raw(), columns) }
    }
    pub fn style(&self) -> u32 {
        unsafe { FXText_get_style(self.as_raw()) }
    }
    pub fn set_style(&self, style: u32) {
        unsafe { FXText_set_style(self.as_raw(), style) }
    }
    pub fn set_word_wrap(&self, wrap: bool) {
        const WORD_WRAP: u32 = 0x0020_0000;
        let style = self.style();
        self.set_style(if wrap {
            style | WORD_WRAP
        } else {
            style & !WORD_WRAP
        });
    }
    pub fn find(&self, needle: &str, start: i32, flags: u32) -> Option<(i32, i32)> {
        let mut beg = 0;
        let mut end = 0;
        let found = unsafe {
            FXText_find_text(
                self.as_raw(),
                to_cstring(needle).as_ptr(),
                start,
                flags,
                &mut beg,
                &mut end,
            )
        };
        if found == 0 { None } else { Some((beg, end)) }
    }
    pub fn select(&self, pos: i32, len: i32) {
        unsafe { FXText_set_selection(self.as_raw(), pos, len) }
    }
    pub fn cut(&self) {
        unsafe { FXText_cut(self.as_raw()) }
    }
    pub fn copy(&self) {
        unsafe { FXText_copy(self.as_raw()) }
    }
    pub fn paste(&self) {
        unsafe { FXText_paste(self.as_raw()) }
    }
    pub fn delete_selection(&self) {
        unsafe { FXText_delete_selection(self.as_raw()) }
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
