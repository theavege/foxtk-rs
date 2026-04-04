use crate::{ObjectExt, WindowExt};
use foxtk_sys::*;
use std::ffi::CString;

pub trait LabelExt: ObjectExt {
    fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            fx_label_set_text(self.as_raw(), c_text.as_ptr());
        }
    }

    fn get_text(&self) -> String {
        unsafe {
            let ptr = fx_label_get_text(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }

    fn set_justify(&self, justify: u32) {
        unsafe {
            fx_label_set_justify(self.as_raw(), justify);
        }
    }

    fn get_justify(&self) -> u32 {
        unsafe { fx_label_get_justify(self.as_raw()) }
    }
}

pub struct Label(ObjectPtr);

impl Label {
    pub fn new(parent: &impl WindowExt, text: &str) -> Self {
        let c_text = CString::new(text).unwrap();
        unsafe {
            Self(fx_label_new(
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

impl ObjectExt for Label {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for Label {}

impl crate::IdExt for Label {}
