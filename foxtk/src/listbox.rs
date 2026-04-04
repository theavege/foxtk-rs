use crate::{ObjectExt, WindowExt};
use foxtk_sys::*;
use std::ffi::CString;

pub trait ListBoxExt: ObjectExt {
    fn append_item(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            fx_list_box_append_item(self.as_raw(), c_text.as_ptr(), std::ptr::null_mut());
        }
    }

    fn clear_items(&self) {
        unsafe {
            fx_list_box_clear_items(self.as_raw());
        }
    }

    fn get_current_item(&self) -> i32 {
        unsafe { fx_list_box_get_current_item(self.as_raw()) }
    }

    fn set_current_item(&self, index: i32) {
        unsafe {
            fx_list_box_set_current_item(self.as_raw(), index);
        }
    }

    fn get_item_text(&self, index: i32) -> String {
        unsafe {
            let ptr = fx_list_box_get_item_text(self.as_raw(), index);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }

    fn get_num_items(&self) -> i32 {
        unsafe { fx_list_box_get_num_items(self.as_raw()) }
    }
}

impl ListBoxExt for ListBox {}

pub struct ListBox(ObjectPtr);

impl ListBox {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self(fx_list_box_new(
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

impl ObjectExt for ListBox {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl crate::IdExt for ListBox {}

impl WindowExt for ListBox {}