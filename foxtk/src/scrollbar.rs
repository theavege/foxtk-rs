pub struct ScrollBar(foxtk_sys::ObjectPtr);

impl super::ObjectExt for ScrollBar {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::WindowExt for ScrollBar {}

impl super::IdExt for ScrollBar {}

impl super::ScrollBarExt for ScrollBar {}
