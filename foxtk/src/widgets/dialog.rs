use crate::prelude::*;
use crate::{cstr_lossy, to_cstring};
use foxtk_sys::*;

impl_widget!(
    FileDialog,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl FileDialog {
    pub fn new(parent: &impl WindowExt, title: &str) -> Self {
        unsafe {
            Self::from_raw(FXFileDialog_new(
                parent.as_raw() as *mut FXWindow,
                to_cstring(title).as_ptr(),
            ))
        }
    }
    pub fn show(&self) {
        unsafe {
            FXDialogBox_show(self.as_raw() as *mut FXDialogBox);
        }
    }
    pub fn set_directory(&self, directory: &str) {
        unsafe {
            FXFileDialog_set_directory(self.as_raw(), to_cstring(directory).as_ptr());
        }
    }
    pub fn directory(&self) -> String {
        unsafe {
            let ptr = FXFileDialog_get_directory(self.as_raw());
            cstr_lossy(ptr)
        }
    }
    pub fn set_filename(&self, filename: &str) {
        unsafe {
            FXFileDialog_set_filename(self.as_raw(), to_cstring(filename).as_ptr());
        }
    }
    pub fn filename(&self) -> String {
        unsafe {
            let ptr = FXFileDialog_get_filename(self.as_raw());
            cstr_lossy(ptr)
        }
    }
    pub fn set_pattern(&self, pattern: &str) {
        unsafe {
            FXFileDialog_set_pattern(self.as_raw(), to_cstring(pattern).as_ptr());
        }
    }
    pub fn pattern(&self) -> String {
        unsafe {
            let ptr = FXFileDialog_get_pattern(self.as_raw());
            cstr_lossy(ptr)
        }
    }
}

impl_widget!(FontSelector, IdExt, FrameExt, DrawableExt, WindowExt);
impl FontSelector {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXFontSelector_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    PrintDialog,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl PrintDialog {
    pub fn new(parent: &impl WindowExt, title: &str) -> Self {
        unsafe {
            Self::from_raw(FXPrintDialog_new(
                parent.as_raw() as *mut FXWindow,
                to_cstring(title).as_ptr(),
            ))
        }
    }
}

impl_widget!(
    ColorDialog,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl ColorDialog {
    pub fn new(parent: &impl WindowExt, title: &str) -> Self {
        unsafe {
            Self::from_raw(FXColorDialog_new(
                parent.as_raw() as *mut FXWindow,
                to_cstring(title).as_ptr(),
            ))
        }
    }
}

impl_widget!(
    DirDialog,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl DirDialog {
    pub fn new(parent: &impl WindowExt, title: &str) -> Self {
        unsafe {
            Self::from_raw(FXDirDialog_new(
                parent.as_raw() as *mut FXWindow,
                to_cstring(title).as_ptr(),
            ))
        }
    }
    pub fn execute(&self) -> u32 {
        unsafe { FXDirDialog_execute(self.as_raw()) }
    }
    pub fn set_directory(&self, path: &str) {
        unsafe { FXDirDialog_set_directory(self.as_raw(), to_cstring(path).as_ptr()) }
    }
    pub fn directory(&self) -> String {
        unsafe {
            let ptr = FXDirDialog_get_directory(self.as_raw());
            cstr_lossy(ptr)
        }
    }
    pub fn show_files(&self, showing: bool) {
        unsafe { FXDirDialog_show_files(self.as_raw(), showing as u8) }
    }
    pub fn show_hidden_files(&self, showing: bool) {
        unsafe { FXDirDialog_show_hidden_files(self.as_raw(), showing as u8) }
    }
    pub fn open_directory(parent: &impl WindowExt, caption: &str, path: &str) -> String {
        unsafe {
            let ptr = FXDirDialog_get_open_directory(
                parent.root().as_raw(),
                to_cstring(caption).as_ptr(),
                to_cstring(path).as_ptr(),
            );
            cstr_lossy(ptr)
        }
    }
}

impl_widget!(
    FontDialog,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl FontDialog {
    pub fn new(parent: &impl WindowExt, title: &str) -> Self {
        unsafe {
            Self::from_raw(FXFontDialog_new(
                parent.as_raw() as *mut FXWindow,
                to_cstring(title).as_ptr(),
            ))
        }
    }
    pub fn execute(&self) -> u32 {
        unsafe { FXFontDialog_execute(self.as_raw()) }
    }
    pub fn set_font_selection(&self, family: &str, point_size: i32) {
        unsafe {
            FXFontDialog_set_font_selection(self.as_raw(), to_cstring(family).as_ptr(), point_size)
        }
    }
    pub fn font_selection(&self) -> (String, i32) {
        unsafe {
            let mut point_size = 0;
            let ptr = FXFontDialog_get_font_selection(self.as_raw(), &mut point_size);
            (cstr_lossy(ptr), point_size)
        }
    }
}

impl_widget!(
    ProgressDialog,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl ProgressDialog {
    pub fn new(parent: &impl WindowExt, caption: &str, label: &str) -> Self {
        unsafe {
            Self::from_raw(FXProgressDialog_new(
                parent.as_raw() as *mut FXWindow,
                to_cstring(caption).as_ptr(),
                to_cstring(label).as_ptr(),
            ))
        }
    }
    pub fn show(&self) {
        unsafe { FXProgressDialog_show(self.as_raw()) }
    }
    pub fn hide(&self) {
        unsafe { FXProgressDialog_hide(self.as_raw()) }
    }
    pub fn set_message(&self, message: &str) {
        unsafe { FXProgressDialog_set_message(self.as_raw(), to_cstring(message).as_ptr()) }
    }
    pub fn set_bar_style(&self, style: ProgressBarStyle) {
        unsafe { FXProgressDialog_set_bar_style(self.as_raw(), style as u32) }
    }
    pub fn set_progress(&self, value: u32) {
        unsafe { FXProgressDialog_set_progress(self.as_raw(), value) }
    }
    pub fn set_total(&self, total: u32) {
        unsafe { FXProgressDialog_set_total(self.as_raw(), total) }
    }
    pub fn increment(&self, value: u32) {
        unsafe { FXProgressDialog_increment(self.as_raw(), value) }
    }
    pub fn is_cancelled(&self) -> bool {
        unsafe { FXProgressDialog_is_cancelled(self.as_raw()) != 0 }
    }
    pub fn set_cancelled(&self, cancelled: bool) {
        unsafe { FXProgressDialog_set_cancelled(self.as_raw(), cancelled as u8) }
    }
}

impl_widget!(
    ReplaceDialog,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl ReplaceDialog {
    pub fn new(parent: &impl WindowExt, caption: &str) -> Self {
        unsafe {
            Self::from_raw(FXReplaceDialog_new(
                parent.as_raw() as *mut FXWindow,
                to_cstring(caption).as_ptr(),
            ))
        }
    }
    pub fn execute(&self) -> u32 {
        unsafe { FXReplaceDialog_execute(self.as_raw()) }
    }
    pub fn search_text(&self) -> String {
        unsafe {
            let ptr = FXReplaceDialog_get_search_text(self.as_raw());
            cstr_lossy(ptr)
        }
    }
    pub fn set_search_text(&self, text: &str) {
        unsafe { FXReplaceDialog_set_search_text(self.as_raw(), to_cstring(text).as_ptr()) }
    }
    pub fn replace_text(&self) -> String {
        unsafe {
            let ptr = FXReplaceDialog_get_replace_text(self.as_raw());
            cstr_lossy(ptr)
        }
    }
    pub fn set_replace_text(&self, text: &str) {
        unsafe { FXReplaceDialog_set_replace_text(self.as_raw(), to_cstring(text).as_ptr()) }
    }
    pub fn search_mode(&self) -> u32 {
        unsafe { FXReplaceDialog_get_search_mode(self.as_raw()) }
    }
    pub fn set_search_mode(&self, mode: u32) {
        unsafe { FXReplaceDialog_set_search_mode(self.as_raw(), mode) }
    }
}

impl_widget!(
    SearchDialog,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl SearchDialog {
    pub fn new(parent: &impl WindowExt, caption: &str) -> Self {
        unsafe {
            Self::from_raw(FXSearchDialog_new(
                parent.as_raw() as *mut FXWindow,
                to_cstring(caption).as_ptr(),
            ))
        }
    }
    pub fn execute(&self) -> u32 {
        unsafe { FXSearchDialog_execute(self.as_raw()) }
    }
    pub fn search_text(&self) -> String {
        unsafe {
            let ptr = FXSearchDialog_get_search_text(self.as_raw());
            cstr_lossy(ptr)
        }
    }
    pub fn set_search_text(&self, text: &str) {
        unsafe { FXSearchDialog_set_search_text(self.as_raw(), to_cstring(text).as_ptr()) }
    }
    pub fn search_mode(&self) -> u32 {
        unsafe { FXSearchDialog_get_search_mode(self.as_raw()) }
    }
    pub fn set_search_mode(&self, mode: u32) {
        unsafe { FXSearchDialog_set_search_mode(self.as_raw(), mode) }
    }
}
