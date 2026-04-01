pub struct Frame(foxtk_sys::FXParentPtr);

impl Frame {
    pub fn new(parent: &crate::MainWindow) -> Self {
        Self(unsafe { foxtk_sys::fox_vertical_frame_new(super::Parent::as_raw(parent)) })
    }
}

impl super::Parent for Frame {
    fn as_raw(&self) -> foxtk_sys::FXParentPtr {
        self.0
    }
}
