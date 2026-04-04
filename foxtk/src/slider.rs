#[derive(Default, Clone)]
pub struct RangeSlider(foxtk_sys::ObjectPtr);

impl super::ObjectExt for RangeSlider {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::IdExt for RangeSlider {}
impl super::WindowExt for RangeSlider {}
impl super::RangeSliderExt for RangeSlider {}
