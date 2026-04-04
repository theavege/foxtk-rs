pub struct TabBook(foxtk_sys::ObjectPtr);

impl super::TabBookExt for TabBook {}

impl super::ObjectExt for TabBook {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::WindowExt for TabBook {}

impl super::IdExt for TabBook {}

pub struct TabItem(foxtk_sys::ObjectPtr);

impl super::TabItemExt for TabItem {}

impl super::ObjectExt for TabItem {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::WindowExt for TabItem {}

impl super::IdExt for TabItem {}
