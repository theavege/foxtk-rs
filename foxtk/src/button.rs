pub struct Button(foxtk_sys::FXWidgetPtr);

impl Button {
    pub fn new(parent: &impl super::Parent, title_: &str) -> Self {
        let title = std::ffi::CString::new(format!("&{title_}").as_str()).unwrap();
        super::Widget::from_raw(unsafe {
            foxtk_sys::fox_button_new(parent.as_raw(), title.as_ptr())
        })
    }
    pub fn state(&self) -> u32 {
        unsafe { foxtk_sys::fox_button_get_state(self.0) }
    }
    pub fn text(&self) -> String {
        unsafe {
            let ptr = foxtk_sys::fox_button_get_text(self.0);
            if !ptr.is_null() {
                std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }
}

impl super::Widget for Button {
    fn as_raw(&self) -> foxtk_sys::FXWidgetPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::FXWidgetPtr) -> Self {
        Self(ptr)
    }
}
