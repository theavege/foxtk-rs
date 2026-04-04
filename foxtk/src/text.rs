use crate::{ObjectExt, WindowExt};
use foxtk_sys::*;
use std::ffi::CString;

pub trait TextExt: ObjectExt {
    fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            fx_text_set_text(self.as_raw(), c_text.as_ptr());
        }
    }

    fn get_text(&self) -> String {
        unsafe {
            let ptr = fx_text_get_text(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

impl TextExt for Text {}

pub struct Text(ObjectPtr);

impl Text {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self(fx_text_new(
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

impl ObjectExt for Text {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl crate::IdExt for Text {}

impl WindowExt for Text {}