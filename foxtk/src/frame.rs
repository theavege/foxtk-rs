pub struct VerticalFrame(foxtk_sys::ObjectPtr);

impl super::ObjectExt for VerticalFrame {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::IdExt for VerticalFrame {}
impl super::WindowExt for VerticalFrame {}
impl super::VerticalFrameExt for VerticalFrame {}
