use crate::{ObjectExt, WindowExt};
use foxtk_sys::*;
use std::ffi::CString;

pub trait TreeListExt: ObjectExt {
    fn add_item_first(&self, parent_item: Option<&TreeItem>, text: &str) -> TreeItem {
        let c_text = CString::new(text).unwrap();
        unsafe {
            TreeItem(fx_tree_list_add_item_first(
                self.as_raw(),
                parent_item.map(|i| i.0).unwrap_or(std::ptr::null_mut()),
                c_text.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ))
        }
    }

    fn clear_items(&self) {
        unsafe {
            fx_tree_list_clear_items(self.as_raw());
        }
    }
}

impl TreeListExt for TreeList {}

pub struct TreeList(ObjectPtr);

impl TreeList {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self(fx_tree_list_new(
                parent.as_raw(),
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ))
        }
    }
}

impl ObjectExt for TreeList {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl crate::IdExt for TreeList {}

impl WindowExt for TreeList {}

pub struct TreeItem(ObjectPtr);

impl TreeItem {
    pub fn as_raw(&self) -> ObjectPtr {
        self.0
    }
}