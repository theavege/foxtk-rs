use crate::{ObjectExt, WindowExt};
use foxtk_sys::*;
use std::ffi::CString;

pub struct MenuBar(ObjectPtr);

impl MenuBar {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self(fx_menu_bar_new(
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

impl ObjectExt for MenuBar {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for MenuBar {}

impl crate::IdExt for MenuBar {}

pub struct MenuPane(ObjectPtr);

impl MenuPane {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self(fx_menu_pane_new(
                parent.as_raw(),
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

impl ObjectExt for MenuPane {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for MenuPane {}

impl crate::IdExt for MenuPane {}

pub struct MenuTitle(ObjectPtr);

impl MenuTitle {
    pub fn new(parent: &impl WindowExt, text: &str, pane: &MenuPane) -> Self {
        let c_text = CString::new(text).unwrap();
        unsafe {
            Self(fx_menu_title_new(
                parent.as_raw(),
                c_text.as_ptr(),
                std::ptr::null_mut(),
                pane.as_raw(),
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

impl ObjectExt for MenuTitle {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for MenuTitle {}

impl crate::IdExt for MenuTitle {}

pub struct MenuCommand(ObjectPtr);

impl MenuCommand {
    pub fn new(parent: &impl WindowExt, text: &str) -> Self {
        let c_text = CString::new(text).unwrap();
        unsafe {
            Self(fx_menu_command_new(
                parent.as_raw(),
                c_text.as_ptr(),
                std::ptr::null_mut(),
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
                0,
            ))
        }
    }

    pub fn set_callback<F>(&self, mut callback: F)
    where
        F: FnMut(Self) -> bool + 'static,
    {
        unsafe {
            fx_window_set_target(
                self.0,
                Some(crate::ccallback::<Self>),
                Box::into_raw(Box::new(Box::new(move |obj: Self| {
                    callback(obj)
                }) as Box<dyn FnMut(Self) -> bool>))
                    as *mut _,
            );
        }
    }
}

impl ObjectExt for MenuCommand {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for MenuCommand {}

impl crate::IdExt for MenuCommand {}