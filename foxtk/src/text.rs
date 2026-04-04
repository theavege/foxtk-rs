pub struct Text(foxtk_sys::ObjectPtr);

impl super::ObjectExt for Text {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::IdExt for Text {}

impl super::WindowExt for Text {}

impl super::TextExt for Text {}
