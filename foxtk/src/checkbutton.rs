#[derive(Default, Clone)]
pub struct CheckButton(foxtk_sys::ObjectPtr);

impl super::ObjectExt for CheckButton {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl super::IdExt for CheckButton {}
impl super::WindowExt for CheckButton {}
impl super::LabelExt for CheckButton {}
impl super::CheckButtonExt for CheckButton {}
