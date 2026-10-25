use super::window::Popup;
use crate::prelude::*;
use crate::to_cstring;
use foxtk_sys::*;

impl_widget!(
    MDIClient,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl MDIClient {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXMDIClient_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    MDIChild,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl MDIChild {
    pub fn new(client: &MDIClient, title: &str) -> Self {
        unsafe { Self::from_raw(FXMDIChild_new(client.as_raw(), to_cstring(title).as_ptr())) }
    }
}

impl_widget!(MDIMenu, IdExt, FrameExt, DrawableExt, WindowExt);
impl MDIMenu {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXMDIMenu_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(MDIDeleteButton, IdExt, FrameExt, DrawableExt, WindowExt);
impl MDIDeleteButton {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXMDIDeleteButton_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(MDIMaximizeButton, IdExt, FrameExt, DrawableExt, WindowExt);
impl MDIMaximizeButton {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXMDIMaximizeButton_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(MDIMinimizeButton, IdExt, FrameExt, DrawableExt, WindowExt);
impl MDIMinimizeButton {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXMDIMinimizeButton_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(MDIRestoreButton, IdExt, FrameExt, DrawableExt, WindowExt);
impl MDIRestoreButton {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXMDIRestoreButton_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(MDIWindowButton, IdExt, FrameExt, DrawableExt, WindowExt);
impl MDIWindowButton {
    pub fn new(parent: &impl CompositeExt, popup: &Popup) -> Self {
        unsafe {
            Self::from_raw(FXMDIWindowButton_new(
                parent.as_raw() as *mut FXComposite,
                popup.as_raw(),
            ))
        }
    }
}
