use crate::{ObjectExt, WindowExt};
use foxtk_sys::*;
use std::ffi::CString;

pub trait TabBookExt: ObjectExt {}

impl TabBookExt for TabBook {}

pub struct TabBook(ObjectPtr);

impl TabBook {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self(fx_tab_book_new(
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

impl ObjectExt for TabBook {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for TabBook {}

impl crate::IdExt for TabBook {}

pub struct TabItem(ObjectPtr);

impl TabItem {
    pub fn new(parent: &impl WindowExt, text: &str) -> Self {
        let c_text = CString::new(text).unwrap();
        unsafe {
            Self(fx_tab_item_new(
                parent.as_raw(),
                c_text.as_ptr(),
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
            ))
        }
    }
}

impl ObjectExt for TabItem {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for TabItem {}

impl crate::IdExt for TabItem {}