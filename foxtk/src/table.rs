use crate::{ObjectExt, WindowExt};
use foxtk_sys::*;
use std::ffi::CString;

pub trait TableExt: ObjectExt {
    fn set_table_size(&self, rows: i32, cols: i32) {
        unsafe {
            fx_table_set_table_size(self.as_raw(), rows, cols);
        }
    }

    fn set_item_text(&self, row: i32, col: i32, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            fx_table_set_item_text(self.as_raw(), row, col, c_text.as_ptr());
        }
    }

    fn get_item_text(&self, row: i32, col: i32) -> String {
        unsafe {
            let ptr = fx_table_get_item_text(self.as_raw(), row, col);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

impl TableExt for Table {}

pub struct Table(ObjectPtr);

impl Table {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self(fx_table_new(
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

impl ObjectExt for Table {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for Table {}

impl crate::IdExt for Table {}