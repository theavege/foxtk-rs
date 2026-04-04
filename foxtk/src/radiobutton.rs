pub struct RadioButton(foxtk_sys::ObjectPtr);

impl Default for RadioButton {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl super::ObjectExt for RadioButton {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl super::IdExt for RadioButton {}
impl super::WindowExt for RadioButton {}
impl super::LabelExt for RadioButton {}
impl super::RadioButtonExt for RadioButton {}