pub struct Table(foxtk_sys::ObjectPtr);

impl super::TableExt for Table {}

impl super::ObjectExt for Table {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::WindowExt for Table {}

impl super::IdExt for Table {}
