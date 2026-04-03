#[derive(Default)]
pub struct TextField(foxtk_sys::ObjectPtr);

impl super::ObjectExt for TextField {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl super::IdExt for TextField {}
impl super::WindowExt for TextField {}
impl super::TextFieldExt for TextField {}
