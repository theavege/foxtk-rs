pub struct Button(foxtk_sys::ObjectPtr);

impl super::ObjectExt for Button {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl super::IdExt for Button {}
impl super::WindowExt for Button {}
impl super::LabelExt for Button {}
impl super::ButtonExt for Button {}
