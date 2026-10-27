use crate::prelude::*;
use crate::to_cstring;
use foxtk_sys::*;

impl_widget!(ColorSelector, IdExt, FrameExt, DrawableExt, WindowExt);
impl ColorSelector {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXColorSelector_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(ColorBar, IdExt, FrameExt, DrawableExt, WindowExt);
impl ColorBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXColorBar_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    ColorList,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl ColorList {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXColorList_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn set_style(&self, style: u32) {
        unsafe { FXColorList_set_style(self.as_raw(), style) }
    }
    pub fn style(&self) -> u32 {
        unsafe { FXColorList_get_style(self.as_raw()) }
    }
    pub fn append_item_with_color(&self, text: &str, color: Color) {
        unsafe {
            FXColorList_append_item_with_color(
                self.as_raw(),
                to_cstring(text).as_ptr(),
                color.bits(),
            )
        }
    }
    pub fn set_item_color(&self, index: i32, color: Color) {
        unsafe { FXColorList_set_item_color(self.as_raw(), index, color.bits()) }
    }
    pub fn item_color(&self, index: i32) -> Color {
        Color::from_hex(unsafe { FXColorList_get_item_color(self.as_raw(), index) })
    }
}

impl_widget!(ColorRing, IdExt, FrameExt, DrawableExt, WindowExt);
impl ColorRing {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXColorRing_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(ColorWell, IdExt, FrameExt, DrawableExt, WindowExt);
impl ColorWell {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXColorWell_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(ColorWheel, IdExt, FrameExt, DrawableExt, WindowExt);
impl ColorWheel {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXColorWheel_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(GradientBar, IdExt, FrameExt, DrawableExt, WindowExt);
impl GradientBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXGradientBar_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn num_segments(&self) -> i32 {
        unsafe { FXGradientBar_get_num_segments(self.as_raw()) }
    }
    pub fn current_segment(&self) -> i32 {
        unsafe { FXGradientBar_get_current_segment(self.as_raw()) }
    }
    pub fn set_current_segment(&self, index: i32) {
        unsafe { FXGradientBar_set_current_segment(self.as_raw(), index) }
    }
    pub fn set_segment_lower_color(&self, index: i32, color: Color) {
        unsafe { FXGradientBar_set_segment_lower_color(self.as_raw(), index, color.bits()) }
    }
    pub fn set_segment_upper_color(&self, index: i32, color: Color) {
        unsafe { FXGradientBar_set_segment_upper_color(self.as_raw(), index, color.bits()) }
    }
}
