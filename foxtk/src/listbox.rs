pub struct ListBox(foxtk_sys::ObjectPtr);

impl super::ObjectExt for ListBox {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::IdExt for ListBox {}
impl super::WindowExt for ListBox {}
impl super::ListBoxExt for ListBox {}
