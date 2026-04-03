pub struct App(foxtk_sys::ObjectPtr);

impl super::ObjectExt for App {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::AppExt for App {}
