pub struct ProgressBar(foxtk_sys::ObjectPtr);

impl super::ObjectExt for ProgressBar {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl super::IdExt for ProgressBar {}
impl super::WindowExt for ProgressBar {}
impl super::ProgressBarExt for ProgressBar {}
