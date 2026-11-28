use crate::app::App;
use crate::prelude::*;
use foxtk_sys::*;

impl_widget!(GLVisual, IdExt);
impl GLVisual {
    pub fn new(app: &App) -> Self {
        unsafe { Self::from_raw(FXGLVisual_new(app.as_raw())) }
    }
}

impl_widget!(
    GLCanvas,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl GLCanvas {
    pub fn new(parent: &impl CompositeExt, visual: &GLVisual) -> Self {
        unsafe {
            Self::from_raw(FXGLCanvas_new(
                parent.as_raw() as *mut FXComposite,
                visual.as_raw(),
            ))
        }
    }
}

impl_widget!(
    GLViewer,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl GLViewer {
    pub fn new(parent: &impl CompositeExt, visual: &GLVisual) -> Self {
        unsafe {
            Self::from_raw(FXGLViewer_new(
                parent.as_raw() as *mut FXComposite,
                visual.as_raw(),
            ))
        }
    }
}
