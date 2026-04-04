pub struct ComboBox(foxtk_sys::ObjectPtr);

impl super::ObjectExt for ComboBox {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::IdExt for ComboBox {}

impl super::WindowExt for ComboBox {}

impl super::ComboBoxExt for ComboBox {}
