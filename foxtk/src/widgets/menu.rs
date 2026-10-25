use crate::prelude::*;
use crate::to_cstring;
use foxtk_sys::*;

impl_widget!(
    MenuBar,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl MenuBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXMenuBar_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    MenuPane,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl MenuPane {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(foxtk_sys::FXMenuPane_new(parent.as_raw() as *mut FXWindow)) }
    }
}

impl_widget!(MenuButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl MenuButton {
    pub fn new(prt: &impl CompositeExt, text_: &str, pane: &MenuPane) -> Self {
        unsafe {
            Self::from_raw(foxtk_sys::FXMenuButton_new(
                prt.as_raw() as *mut FXComposite,
                to_cstring(text_).as_ptr(),
                pane.as_raw() as *mut FXPopup,
            ))
        }
        .with_layout(Layout::FillX)
    }
}

impl_widget!(MenuTitle, IdExt, FrameExt, DrawableExt, WindowExt);
impl MenuTitle {
    pub fn new(prt: &impl CompositeExt, text_: &str, pane: &MenuPane) -> Self {
        unsafe {
            Self::from_raw(foxtk_sys::FXMenuTitle_new(
                prt.as_raw() as *mut FXComposite,
                to_cstring(text_).as_ptr(),
                pane.as_raw() as *mut FXPopup,
            ))
        }
        .with_layout(Layout::FillX)
    }
}

impl_widget!(MenuRadio, IdExt, FrameExt, DrawableExt, WindowExt);
impl MenuRadio {
    pub fn new(parent: &impl CompositeExt, text: &str) -> Self {
        unsafe {
            Self::from_raw(FXMenuRadio_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(text).as_ptr(),
            ))
        }
    }
}

impl_widget!(MenuCheck, IdExt, FrameExt, DrawableExt, WindowExt);
impl MenuCheck {
    pub fn new(parent: &impl CompositeExt, text: &str) -> Self {
        unsafe {
            Self::from_raw(FXMenuCheck_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(text).as_ptr(),
            ))
        }
    }
}

impl_widget!(MenuCommand, IdExt, FrameExt, DrawableExt, WindowExt);
impl MenuCommand {
    pub fn new(parent: &impl CompositeExt, text: &str) -> Self {
        unsafe {
            Self::from_raw(FXMenuCommand_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(text).as_ptr(),
            ))
        }
    }
}

impl_widget!(MenuCaption, IdExt, FrameExt, DrawableExt, WindowExt);
impl MenuCaption {
    pub fn new(parent: &impl CompositeExt, text: &str) -> Self {
        unsafe {
            Self::from_raw(FXMenuCaption_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(text).as_ptr(),
            ))
        }
    }
}

impl_widget!(MenuCascade, IdExt, FrameExt, DrawableExt, WindowExt);
impl MenuCascade {
    pub fn new(parent: &impl CompositeExt, text: &str) -> Self {
        unsafe {
            Self::from_raw(FXMenuCascade_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(text).as_ptr(),
            ))
        }
    }
}

impl_widget!(MenuSeparator, IdExt, FrameExt, DrawableExt, WindowExt);
impl MenuSeparator {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXMenuSeparator_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    OptionMenu,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    PackerExt,
    CompositeExt
);
impl OptionMenu {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXOptionMenu_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(@as OptionButton, FXOption, IdExt, FrameExt, DrawableExt, WindowExt);
impl OptionButton {
    pub fn new(parent: &impl CompositeExt, text: &str) -> Self {
        unsafe {
            Self::from_raw(FXOption_new(
                parent.as_raw() as *mut FXComposite,
                to_cstring(text).as_ptr(),
            ))
        }
    }
}
