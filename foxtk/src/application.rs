use std::ffi::CString;
use std::os::raw::{c_char, c_int};

unsafe extern "C" fn ctimer<T: ApplicationExt>(
    ptr: foxtk_sys::FXAppPtr,
    context: *mut std::os::raw::c_void,
) -> std::os::raw::c_long {
    unsafe {
        let func: &mut Box<dyn FnMut(T) -> bool> =
            &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr)) as std::os::raw::c_long
    }
}

pub struct Application(foxtk_sys::FXAppPtr);

impl Application {
    pub fn as_raw(&self) -> foxtk_sys::FXAppPtr {
        self.0
    }
    pub fn new(name_: &str, vendor_: &str) -> Self {
        let name = std::ffi::CString::new(name_).unwrap();
        let vendor = std::ffi::CString::new(vendor_).unwrap();
        let args = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();
        let app = Self(unsafe { foxtk_sys::fox_app_new(name.as_ptr(), vendor.as_ptr()) });
        unsafe {
            foxtk_sys::fox_app_init(
                app.0,
                args.len() as c_int,
                args.as_ptr() as *mut *mut c_char,
            )
        };
        app
    }
    pub fn add_timeout<F: FnMut(Self) -> bool + 'static>(&self, ms: u32, func: F) {
        let raw_ptr: *mut Box<dyn FnMut(Self) -> bool> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            foxtk_sys::fox_app_add_timeout(
                self.as_raw(),
                Some(ctimer::<Self>),
                ms,
                raw_ptr as *mut std::os::raw::c_void,
            );
        }
    }
    pub fn run(&self) -> i32 {
        unsafe { foxtk_sys::fox_app_run(self.0) }
    }
}

pub trait ApplicationExt {
    fn from_raw(ptr: foxtk_sys::FXAppPtr) -> Self;
}

impl ApplicationExt for Application {
    fn from_raw(ptr: foxtk_sys::FXAppPtr) -> Self {
        Self(ptr)
    }
}
