pub struct MainWindow(foxtk_sys::FXParentPtr);

impl MainWindow {
    pub fn new(app: &crate::Application, title_: &str, w: i32, h: i32) -> Self {
        let title = std::ffi::CString::new(title_).unwrap();
        let wgt =
            Self(unsafe { foxtk_sys::fox_main_window_new(app.as_raw(), title.as_ptr(), w, h) });
        wgt.show();
        wgt
    }
    pub fn show(&self) {
        unsafe { foxtk_sys::fox_main_window_show(self.0) }
    }
}

impl super::Parent for MainWindow {
    fn as_raw(&self) -> foxtk_sys::FXParentPtr {
        self.0
    }
}
