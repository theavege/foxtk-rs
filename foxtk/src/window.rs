pub struct MainWindow(foxtk_sys::ObjectPtr);

impl super::ObjectExt for MainWindow {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::IdExt for MainWindow {}
impl super::WindowExt for MainWindow {}
impl super::MainWindowExt for MainWindow {}
