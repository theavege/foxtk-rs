use crate::{ObjectExt, WindowExt};
use foxtk_sys::*;

pub struct Canvas(ObjectPtr);

impl Canvas {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self(fx_canvas_new(
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

impl ObjectExt for Canvas {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for Canvas {}

impl crate::IdExt for Canvas {}