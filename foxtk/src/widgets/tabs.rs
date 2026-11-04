use super::frame::Composite;
use crate::prelude::*;
use crate::{cstr_lossy, to_cstring};
use foxtk_sys::*;

impl_widget!(
    Switcher,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl Switcher {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXSwitcher_new(parent.as_raw() as *mut FXComposite)) }
            .with_layout(Layout::Fill)
            .with_frame(FrameStyle::Thick)
    }
    pub fn set_current(&self, idx: i32) {
        unsafe { FXSwitcher_set_current(self.as_raw(), idx) }
    }

    #[deprecated(since = "0.0.9", note = "renamed to `set_current`")]
    pub fn set_curent(&self, idx: i32) {
        self.set_current(idx);
    }
}

impl_widget!(
    TabBar,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    PackerExt,
    CompositeExt
);
impl TabBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXTabBar_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    TabBook,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    PackerExt,
    CompositeExt
);
impl TabBook {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXTabBook_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn set_current(&self, index: i32) {
        unsafe {
            FXTabBook_set_current(self.as_raw(), index);
        }
    }
    pub fn current(&self) -> i32 {
        unsafe { FXTabBook_get_current(self.as_raw()) }
    }
    pub fn num_children(&self) -> i32 {
        unsafe { FXTabBook_get_num_children(self.as_raw()) }
    }
    pub fn with_current(self, index: i32) -> Self {
        self.set_current(index);
        self
    }
}

impl_widget!(
    TabItem,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    PackerExt,
    CompositeExt
);
impl TabItem {
    pub fn new(parent: &TabBook, text: &str) -> Self {
        let c_text = to_cstring(text);
        unsafe { Self::from_raw(FXTabItem_new(parent.as_raw(), c_text.as_ptr())) }
    }
    pub fn set_text(&self, text: &str) {
        unsafe {
            FXTabItem_set_text(self.as_raw(), to_cstring(text).as_ptr());
        }
    }
    pub fn text(&self) -> String {
        unsafe {
            let ptr = FXTabItem_get_text(self.as_raw());
            cstr_lossy(ptr)
        }
    }
}

impl_widget!(
    Shutter,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl Shutter {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXShutter_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn current(&self) -> i32 {
        unsafe { FXShutter_get_current(self.as_raw()) }
    }
    pub fn set_current(&self, panel: i32) {
        unsafe { FXShutter_set_current(self.as_raw(), panel) }
    }
}

impl_widget!(
    ShutterItem,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl ShutterItem {
    pub fn new(parent: &Shutter, text: &str) -> Self {
        unsafe {
            Self::from_raw(FXShutterItem_new(
                parent.as_raw(),
                to_cstring(text).as_ptr(),
            ))
        }
    }
    pub fn content(&self) -> Composite {
        unsafe { Composite::from_raw(FXShutterItem_get_content(self.as_raw())) }
    }
}
