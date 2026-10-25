use crate::app::App;
use crate::prelude::*;
use crate::{cmouse_callback, drop_callback_context, to_cstring};
use foxtk_sys::*;
use std::ffi::c_void;

impl_widget!(Window, IdExt, FrameExt, DrawableExt, WindowExt);

impl_widget!(
    TopWindow,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);

impl_widget!(SplashWindow, IdExt, FrameExt, DrawableExt, WindowExt);
impl SplashWindow {
    pub fn new(app: &App) -> Self {
        unsafe { Self::from_raw(FXSplashWindow_new(app.as_raw())) }
    }
}

impl_widget!(Canvas, IdExt, FrameExt, DrawableExt, WindowExt);
impl Canvas {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXCanvas_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn set_mouse_callback<F: FnMut(Self, i32, i32, i32) -> bool + 'static>(&self, func: F) {
        let context: *mut Box<dyn FnMut(Self, i32, i32, i32) -> bool> =
            Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            FXCanvas_set_mouse_callback(
                self.as_raw(),
                Some(cmouse_callback::<Self>),
                context as *mut c_void,
                Some(drop_callback_context::<Box<dyn FnMut(Self, i32, i32, i32) -> bool>>),
            );
        }
    }
}

impl_widget!(
    DialogBox,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl DialogBox {
    pub fn new(parent: &impl WindowExt, title: &str) -> Self {
        unsafe {
            Self::from_raw(FXDialogBox_new(
                parent.as_raw() as *mut FXWindow,
                to_cstring(title).as_ptr(),
            ))
        }
    }
    pub fn show(&self) {
        unsafe {
            FXDialogBox_show(self.as_raw());
        }
    }
    pub fn hide(&self) {
        unsafe {
            FXDialogBox_hide(self.as_raw());
        }
    }
    pub fn shown(&self) -> bool {
        unsafe { FXDialogBox_shown(self.as_raw()) != 0 }
    }
}

impl_widget!(
    MainWindow,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    TopWindowExt
);
impl MainWindow {
    pub fn new(app: &App, title_: &str, w: i32, h: i32) -> Self {
        unsafe {
            Self::from_raw(FXMainWindow_new(
                app.as_raw(),
                to_cstring(title_).as_ptr(),
                w,
                h,
            ))
        }
        .with_pad(0)
    }
    pub fn show(&self) {
        unsafe {
            FXMainWindow_show(self.as_raw());
        }
    }
}

impl_widget!(
    Wizard,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl Wizard {
    pub fn new(parent: &impl WindowExt, title: &str) -> Self {
        unsafe {
            Self::from_raw(FXWizard_new(
                parent.as_raw() as *mut FXWindow,
                to_cstring(title).as_ptr(),
            ))
        }
    }
}

impl_widget!(Popup, IdExt, FrameExt, DrawableExt, WindowExt, TopWindowExt);
impl Popup {
    pub fn new(owner: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(FXPopup_new(owner.as_raw() as *mut FXWindow)) }
    }
}

impl_widget!(DragCorner, IdExt, FrameExt, DrawableExt, WindowExt);
impl DragCorner {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXDragCorner_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(ScrollCorner, IdExt, FrameExt, DrawableExt, WindowExt);
impl ScrollCorner {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXScrollCorner_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    ScrollPane,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl ScrollPane {
    pub fn new(owner: &impl WindowExt, num_visible: i32) -> Self {
        unsafe {
            Self::from_raw(FXScrollPane_new(
                owner.as_raw() as *mut FXWindow,
                num_visible,
            ))
        }
    }
}

impl_widget!(
    ScrollWindow,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl ScrollWindow {
    pub fn new(
        parent: &impl CompositeExt,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Self {
        unsafe {
            Self::from_raw(FXScrollWindow_new(
                parent.as_raw() as *mut FXComposite,
                style,
                x,
                y,
                width,
                height,
            ))
        }
    }
    pub fn x_position(&self) -> i32 {
        unsafe { FXScrollWindow_get_x_position(self.as_raw()) }
    }
    pub fn y_position(&self) -> i32 {
        unsafe { FXScrollWindow_get_y_position(self.as_raw()) }
    }
    pub fn set_position(&self, x: i32, y: i32) {
        unsafe { FXScrollWindow_set_position(self.as_raw(), x, y) }
    }
    pub fn scroll_style(&self) -> u32 {
        unsafe { FXScrollWindow_get_scroll_style(self.as_raw()) }
    }
    pub fn set_scroll_style(&self, style: u32) {
        unsafe { FXScrollWindow_set_scroll_style(self.as_raw(), style) }
    }
}
