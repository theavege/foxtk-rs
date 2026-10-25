use crate::prelude::*;
use crate::to_cstring;
use foxtk_sys::*;

impl_widget!(Button, IdExt, DrawableExt, WindowExt, FrameExt);
impl Button {
    pub fn new(parent: &impl CompositeExt, title: &str) -> Self {
        unsafe {
            Self::from_raw(FXButton_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(&format!("&{title}")).as_ptr(),
            ))
        }
        .with_layout(Layout::FillX)
    }
    pub fn set_state(&self, state: ButtonState) {
        unsafe {
            FXButton_set_state(self.as_raw(), state as u32);
        }
    }
    pub fn with_style(self, style: ButtonStyle) -> Self {
        unsafe {
            FXButton_set_style(self.as_raw(), style as u32);
        }
        self
    }
}

impl_widget!(ArrowButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl ArrowButton {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXArrowButton_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn set_size(&self, size: i32) {
        unsafe {
            FXArrowButton_set_arrow_size(self.as_raw(), size);
        }
    }
    pub fn set_color(&self, color: Color) {
        unsafe {
            FXArrowButton_set_arrow_color(self.as_raw(), color.bits());
        }
    }
}

impl_widget!(CheckButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl CheckButton {
    pub fn new(parent: &impl CompositeExt, title: &str) -> Self {
        unsafe {
            Self::from_raw(FXCheckButton_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(title).as_ptr(),
            ))
        }
        .with_layout(Layout::FillX)
    }
    pub fn check(&self) -> bool {
        unsafe { FXCheckButton_get_check(self.as_raw()) != 0 }
    }
    pub fn set_check(&self, check: bool) {
        unsafe { FXCheckButton_set_check(self.as_raw(), check as u8) }
    }
    pub fn with_check(self, check: bool) -> Self {
        self.set_check(check);
        self
    }
}

impl_widget!(TriStateButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl TriStateButton {
    pub fn new(parent: &impl CompositeExt, text1: &str, text2: &str, text3: &str) -> Self {
        unsafe {
            Self::from_raw(FXTriStateButton_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(text1).as_ptr(),
                to_cstring(text2).as_ptr(),
                to_cstring(text3).as_ptr(),
            ))
        }
        .with_layout(Layout::FillX)
    }
}

impl_widget!(ToggleButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl ToggleButton {
    pub fn new(parent: &impl CompositeExt, title: &str, title_: &str) -> Self {
        unsafe {
            Self::from_raw(FXToggleButton_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(title).as_ptr(),
                to_cstring(title_).as_ptr(),
            ))
        }
        .with_layout(Layout::FillX)
    }
}

impl_widget!(RadioButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl RadioButton {
    pub fn new(parent: &impl CompositeExt, title: &str) -> Self {
        unsafe {
            Self::from_raw(FXRadioButton_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(title).as_ptr(),
            ))
        }
        .with_layout(Layout::FillX)
    }
    pub fn check(&self) -> bool {
        unsafe { FXRadioButton_get_check(self.as_raw()) != 0 }
    }
    pub fn set_check(&self, check: bool) {
        unsafe { FXRadioButton_set_check(self.as_raw(), check as u8) }
    }
}
