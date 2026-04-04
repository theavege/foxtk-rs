#[derive(Default, Clone)]
pub struct Spinner(foxtk_sys::ObjectPtr);

impl super::ObjectExt for Spinner {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl super::IdExt for Spinner {}
impl super::WindowExt for Spinner {}
impl super::SpinnerExt for Spinner {}
