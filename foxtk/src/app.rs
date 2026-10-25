use crate::prelude::*;
use crate::{cstr_lossy, cstring_args, ctimer_callback, drop_callback_context, to_cstring};
use foxtk_sys::*;
use std::ffi::c_void;
use std::ptr::NonNull;

/// A repeating FOX timer. Dropping the handle cancels the timer and releases
/// its callback; the borrowed app guarantees the timer cannot outlive it.
/// The callback's `bool` is FOX's event result and does not control repetition.
#[must_use = "keep the timeout handle alive to keep the timer running"]
pub struct Timeout<'app> {
    app: &'app App,
    handle: std::option::Option<NonNull<FXTimeout>>,
}

impl Drop for Timeout<'_> {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            unsafe { FXApp_remove_timeout(self.app.as_raw(), handle.as_ptr()) };
        }
    }
}

impl_widget!(App,);
/// The application object manages the message queue, timers, chores, signal handling, GUI updating, and other system facilities. Each FOX application will have exactly one application instance. Every FOX application will start by constructing one FXApp object prior to building the GUI.  Usually, the FXApp object is the last object to be deleted as well.
///
/// Using the code below, the application object will be constructed on the stack and hence is automatically destroyed when the program terminates.  Also, when the application object is destroyed, all the windows and other resources it knows about are destroyed as well.
impl App {
    pub fn new(name: &str, vendor: &str) -> Self {
        let name = to_cstring(name);
        let vendor = to_cstring(vendor);
        // Own every argument for the whole call. Mapping to pointers first
        // drops each CString at the end of the closure.
        let args = cstring_args(std::env::args());
        let mut argv: Vec<*mut std::ffi::c_char> = args
            .iter()
            .map(|arg| arg.as_ptr() as *mut std::ffi::c_char)
            .collect();
        unsafe {
            Self::from_raw(FXApp_new(
                name.as_ptr(),
                vendor.as_ptr(),
                argv.len() as i32,
                argv.as_mut_ptr(),
            ))
        }
    }
    pub fn add_timeout<F: FnMut(Self) -> bool + 'static>(&self, ms: u32, func: F) -> Timeout<'_> {
        let context: *mut Box<dyn FnMut(Self) -> bool> = Box::into_raw(Box::new(Box::new(func)));
        let handle = unsafe {
            FXApp_add_timeout(
                self.as_raw(),
                Some(ctimer_callback::<Self>),
                ms,
                context as *mut c_void,
                Some(drop_callback_context::<Box<dyn FnMut(Self) -> bool>>),
            )
        };
        let handle = NonNull::new(handle);
        if handle.is_none() {
            unsafe {
                drop_callback_context::<Box<dyn FnMut(Self) -> bool>>(context as *mut c_void)
            };
        }
        Timeout { app: self, handle }
    }
    pub fn run(&self) -> i32 {
        unsafe { FXApp_run(self.as_raw()) }
    }
}

impl_widget!(RecentFiles, IdExt);
impl RecentFiles {
    pub fn new(app: &App) -> Self {
        unsafe { Self::from_raw(FXRecentFiles_new(app.as_raw())) }
    }
    pub fn max_files(&self) -> i32 {
        unsafe { FXRecentFiles_get_max_files(self.as_raw()) }
    }
    pub fn set_max_files(&self, maximum: i32) {
        unsafe { FXRecentFiles_set_max_files(self.as_raw(), maximum) }
    }
    pub fn file(&self, index: i32) -> String {
        unsafe {
            let ptr = FXRecentFiles_get_file(self.as_raw(), index);
            cstr_lossy(ptr)
        }
    }
    pub fn set_file(&self, index: i32, filename: &str) {
        unsafe { FXRecentFiles_set_file(self.as_raw(), index, to_cstring(filename).as_ptr()) }
    }
    pub fn append_file(&self, filename: &str) {
        unsafe { FXRecentFiles_append_file(self.as_raw(), to_cstring(filename).as_ptr()) }
    }
    pub fn remove_file(&self, filename: &str) {
        unsafe { FXRecentFiles_remove_file(self.as_raw(), to_cstring(filename).as_ptr()) }
    }
    pub fn clear(&self) {
        unsafe { FXRecentFiles_clear(self.as_raw()) }
    }
}

impl_widget!(ToolTip, IdExt);
impl ToolTip {
    pub fn new(app: &App) -> Self {
        unsafe { Self::from_raw(FXToolTip_new(app.as_raw())) }
    }
    pub fn show(&self) {
        unsafe { FXToolTip_show(self.as_raw()) }
    }
    pub fn text(&self) -> String {
        unsafe {
            let ptr = FXToolTip_get_text(self.as_raw());
            cstr_lossy(ptr)
        }
    }
    pub fn set_text(&self, text: &str) {
        unsafe { FXToolTip_set_text(self.as_raw(), to_cstring(text).as_ptr()) }
    }
    pub fn set_text_color(&self, color: Color) {
        unsafe { FXToolTip_set_text_color(self.as_raw(), color.bits()) }
    }
    pub fn set_font(&self, family: &str, size: i32) {
        unsafe { FXToolTip_set_font(self.as_raw(), to_cstring(family).as_ptr(), size) }
    }
}
