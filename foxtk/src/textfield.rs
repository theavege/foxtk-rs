#[derive(Default)]
pub struct TextField(foxtk_sys::FXWidgetPtr);

impl TextField {
    pub fn new(parent: &impl super::Parent, ncols: i32) -> Self {
        super::Widget::from_raw(unsafe { foxtk_sys::fox_textfield_new(parent.as_raw(), ncols) })
    }
    pub fn set_text(&self, text_: &str) {
        let text = std::ffi::CString::new(text_).unwrap();
        unsafe { foxtk_sys::fox_textfield_set_text(self.0, text.as_ptr()) };
    }
}

impl super::Widget for TextField {
    fn as_raw(&self) -> foxtk_sys::FXWidgetPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::FXWidgetPtr) -> Self {
        Self(ptr)
    }
}
