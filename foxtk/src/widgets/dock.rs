use crate::prelude::*;
use crate::to_cstring;
use foxtk_sys::*;

impl_widget!(
    DockBar,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl DockBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXDockBar_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    DockSite,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl DockSite {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXDockSite_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(DockTitle, IdExt, FrameExt, DrawableExt, WindowExt);
impl DockTitle {
    pub fn new(bar: &DockBar, title: &str) -> Self {
        unsafe { Self::from_raw(FXDockTitle_new(bar.as_raw(), to_cstring(title).as_ptr())) }
    }
    pub fn set_justify(&self, justify: Justify) {
        unsafe {
            FXDockTitle_set_justify(self.as_raw(), justify as u32);
        }
    }
    pub fn justify(&self) -> Justify {
        crate::enums::justify_from_raw(unsafe { FXDockTitle_get_justify(self.as_raw()) as u32 })
    }
    pub fn with_justify(self, justify: Justify) -> Self {
        self.set_justify(justify);
        self
    }
}

impl_widget!(
    ToolBar,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl ToolBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXToolBar_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(ToolBarGrip, IdExt, FrameExt, DrawableExt, WindowExt);
impl ToolBarGrip {
    pub fn new(toolbar: &ToolBar) -> Self {
        unsafe { Self::from_raw(FXToolBarGrip_new(toolbar.as_raw())) }
    }
}

impl_widget!(ToolBarTab, IdExt, FrameExt, DrawableExt, WindowExt);
impl ToolBarTab {
    pub fn new(toolbar: &ToolBar) -> Self {
        unsafe { Self::from_raw(FXToolBarTab_new(toolbar.as_raw())) }
    }
}

impl_widget!(
    ToolBarShell,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl ToolBarShell {
    pub fn new(owner: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(FXToolBarShell_new(owner.as_raw() as *mut FXWindow)) }
    }
    pub fn show(&self) {
        unsafe { FXToolBarShell_show(self.as_raw()) }
    }
    pub fn hide(&self) {
        unsafe { FXToolBarShell_hide(self.as_raw()) }
    }
}
