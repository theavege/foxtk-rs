use crate::{ObjectExt, WindowExt};
use foxtk_sys::*;

pub trait ScrollBarExt: ObjectExt {
    fn get_position(&self) -> i32 {
        unsafe { fx_scroll_bar_get_position(self.as_raw()) }
    }

    fn set_position(&self, pos: i32) {
        unsafe {
            fx_scroll_bar_set_position(self.as_raw(), pos);
        }
    }

    fn set_range(&self, lo: i32, hi: i32) {
        unsafe {
            fx_scroll_bar_set_range(self.as_raw(), lo, hi);
        }
    }
}

impl ScrollBarExt for ScrollBar {}

pub struct ScrollBar(ObjectPtr);

impl ScrollBar {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self(fx_scroll_bar_new(
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

impl ObjectExt for ScrollBar {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for ScrollBar {}

impl crate::IdExt for ScrollBar {}