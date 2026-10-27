use crate::prelude::ObjectExt;
use std::ffi::{CString, c_char, c_long, c_void};

pub(crate) const HEIGHT: i32 = 30;

/// Convert `s` to a [`CString`], replacing embedded nulls with `_`.
pub(crate) fn to_cstring(s: &str) -> CString {
    CString::new(s).unwrap_or_else(|_| CString::new(s.replace('\0', "_")).unwrap())
}

/// Own the strings `FXApp_new` reads, so their pointers stay valid for the call.
pub(crate) fn cstring_args<I, S>(args: I) -> Vec<CString>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    args.into_iter()
        .map(|arg| to_cstring(arg.as_ref()))
        .collect()
}

/// Argument vector stored by `FXApp::init`.
///
/// FOX keeps the pointer and writes the terminating null at `argv[argc]`,
/// one past the last argument. The extra slot is that null. Both the
/// pointer buffer and the strings have to outlive the `FXApp`.
pub(crate) struct FoxArgv {
    _owned: Vec<CString>,
    ptrs: Vec<*mut c_char>,
}

impl FoxArgv {
    pub(crate) fn from_args<I, S>(args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let owned = cstring_args(args);
        let mut ptrs: Vec<*mut c_char> = owned
            .iter()
            .map(|arg| arg.as_ptr() as *mut c_char)
            .collect();
        ptrs.push(std::ptr::null_mut());
        Self {
            _owned: owned,
            ptrs,
        }
    }

    pub(crate) fn argc(&self) -> i32 {
        (self.ptrs.len() - 1) as i32
    }

    /// Pointer FOX may store and write through, including `argv[argc]`.
    pub(crate) fn argv(&mut self) -> *mut *mut c_char {
        self.ptrs.as_mut_ptr()
    }
}

/// Copy a FOX `const char*` into an owned [`String`]. A null pointer is empty.
pub(crate) fn cstr_lossy(ptr: *const std::ffi::c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }
}

pub(crate) unsafe extern "C" fn ctimer_callback<T: ObjectExt>(
    ptr: *mut T::T,
    context: *mut c_void,
) -> c_long {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let func = &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr)) as c_long
    }));
    result.unwrap_or(0)
}

pub(crate) unsafe extern "C" fn drop_callback_context<T>(context: *mut c_void) {
    if !context.is_null() {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            drop(Box::from_raw(context as *mut T));
        }));
    }
}

pub(crate) unsafe extern "C" fn cmouse_callback<T: ObjectExt>(
    ptr: *mut T::T,
    selector: i32,
    x: i32,
    y: i32,
    context: *mut c_void,
) -> c_long {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let func = &mut *(context as *mut Box<dyn FnMut(T, i32, i32, i32) -> bool>);
        func(T::from_raw(ptr), selector, x, y) as c_long
    }));
    result.unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{FoxArgv, cstr_lossy, cstring_args, to_cstring};
    use std::ffi::CStr;

    #[test]
    fn argument_strings_outlive_their_pointers() {
        let owned = cstring_args(["foxtk", "hello world", "юникод"]);
        let ptrs: Vec<*const std::ffi::c_char> = owned.iter().map(|arg| arg.as_ptr()).collect();
        for (arg, ptr) in owned.iter().zip(ptrs) {
            assert_eq!(unsafe { CStr::from_ptr(ptr) }, arg.as_c_str());
        }
    }

    #[test]
    fn fox_argv_keeps_a_slot_past_argc() {
        let mut argv = FoxArgv::from_args(["foxtk", "-sync", "keep"]);
        assert_eq!(argv.argc(), 3);
        let ptr = argv.argv();
        unsafe {
            assert_eq!(CStr::from_ptr(*ptr).to_str().unwrap(), "foxtk");
            assert_eq!(CStr::from_ptr(*ptr.add(2)).to_str().unwrap(), "keep");
            assert!((*ptr.add(3)).is_null());
        }
    }

    #[test]
    fn null_fox_string_is_empty() {
        assert_eq!(cstr_lossy(std::ptr::null()), "");
        let text = to_cstring("fox");
        assert_eq!(cstr_lossy(text.as_ptr()), "fox");
    }
}
