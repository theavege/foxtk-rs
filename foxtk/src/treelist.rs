pub struct TreeList(foxtk_sys::ObjectPtr);

impl super::TreeListExt for TreeList {}

impl super::ObjectExt for TreeList {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::IdExt for TreeList {}

impl super::WindowExt for TreeList {}

pub struct TreeItem(foxtk_sys::ObjectPtr);

impl super::ObjectExt for TreeItem {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
