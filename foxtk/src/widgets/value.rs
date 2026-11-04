use crate::prelude::*;
use foxtk_sys::*;

impl_widget!(Knob, IdExt, FrameExt, DrawableExt, WindowExt);
impl Knob {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXKnob_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(Dial, IdExt, FrameExt, DrawableExt, WindowExt);
impl Dial {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXDial_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(Slider, IdExt, FrameExt, DrawableExt, WindowExt);
impl Slider {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXSlider_new(parent.as_raw() as *mut FXComposite)) }
            .with_selector(Selector::CHANGED)
            .with_layout(Layout::FillX)
    }
}

impl_widget!(
    Spinner,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl Spinner {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXSpinner_new(parent.as_raw() as *mut FXComposite)) }
            .with_layout(Layout::FillX)
    }
    pub fn decrement(&self) {
        unsafe { FXSpinner_decrement(self.as_raw()) }
    }
}

impl_widget!(RealSlider, IdExt, FrameExt, DrawableExt, WindowExt);
impl RealSlider {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXRealSlider_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(RealSpinner, IdExt, FrameExt, DrawableExt, WindowExt);
impl RealSpinner {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXRealSpinner_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(Ruler, IdExt, FrameExt, DrawableExt, WindowExt);
impl Ruler {
    pub fn new(parent: &impl CompositeExt, orientation: u32) -> Self {
        unsafe {
            Self::from_raw(FXRuler_new(
                parent.as_raw() as *mut FXComposite,
                orientation,
            ))
        }
    }
}
