use crate::app::App;
use crate::prelude::*;
use foxtk_sys::*;

impl_widget!(BitmapFrame, IdExt, FrameExt, DrawableExt, WindowExt);
impl BitmapFrame {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXBitmapFrame_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn set_justify(&self, justify: Justify) {
        unsafe {
            FXBitmapFrame_set_justify(self.as_raw(), justify as u32);
        }
    }
    pub fn justify(&self) -> Justify {
        crate::enums::justify_from_raw(unsafe { FXBitmapFrame_get_justify(self.as_raw()) as u32 })
    }
    pub fn with_justify(self, justify: Justify) -> Self {
        self.set_justify(justify);
        self
    }
}

impl_widget!(BitmapView, IdExt, FrameExt, DrawableExt, WindowExt);
impl BitmapView {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXBitmapView_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(ImageFrame, IdExt, FrameExt, DrawableExt, WindowExt);
impl ImageFrame {
    pub fn new(parent: &impl CompositeExt, img: &Image) -> Self {
        unsafe {
            Self::from_raw(FXImageFrame_new(
                parent.as_raw() as *mut FXComposite,
                img.as_raw(),
            ))
        }
    }
    pub fn set_justify(&self, justify: Justify) {
        unsafe {
            FXImageFrame_set_justify(self.as_raw(), justify as u32);
        }
    }
    pub fn justify(&self) -> Justify {
        crate::enums::justify_from_raw(unsafe { FXImageFrame_get_justify(self.as_raw()) as u32 })
    }
    pub fn with_justify(self, justify: Justify) -> Self {
        self.set_justify(justify);
        self
    }
}

impl_widget!(Image, IdExt);
impl Image {
    pub fn new(app: &App) -> Self {
        unsafe { Self::from_raw(FXImage_new(app.as_raw())) }
    }
}

impl_widget!(ImageView, IdExt, FrameExt, DrawableExt, WindowExt);
impl ImageView {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXImageView_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(Icon, IdExt);
impl Icon {
    pub fn new(app: &App) -> Self {
        unsafe { Self::from_raw(FXIcon_new(app.as_raw())) }
    }
    /// GIF file bytes. FOX copies the pixels during this call.
    pub fn from_gif(app: &App, bytes: &[u8]) -> Self {
        unsafe {
            Self::from_raw(FXGIFIcon_new(
                app.as_raw(),
                bytes.as_ptr(),
                bytes.len() as u32,
            ))
        }
    }
    /// BMP file bytes. FOX copies the pixels during this call.
    pub fn from_bmp(app: &App, bytes: &[u8]) -> Self {
        unsafe {
            Self::from_raw(FXBMPIcon_new(
                app.as_raw(),
                bytes.as_ptr(),
                bytes.len() as u32,
            ))
        }
    }
}

impl_widget!(DCWindow, IdExt);
impl DCWindow {
    pub fn new(drawable: &impl DrawableExt) -> Self {
        unsafe { Self::from_raw(FXDCWindow_new(drawable.as_raw() as *mut FXDrawable)) }
    }
    pub fn set_foreground(&self, color: Color) {
        unsafe { FXDCWindow_set_foreground(self.as_raw(), color.bits()) }
    }
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe { FXDCWindow_draw_line(self.as_raw(), x1, y1, x2, y2) }
    }
    pub fn fill_rectangle(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { FXDCWindow_fill_rect(self.as_raw(), x, y, width, height) }
    }
    /// Ends the drawing context. The C++ object is not owned by a parent.
    pub fn finish(self) {
        unsafe { FXDCWindow_free(self.as_raw()) }
    }
}

impl_widget!(Bitmap, IdExt);
impl Bitmap {
    pub fn new(app: &App) -> Self {
        unsafe { Self::from_raw(FXBitmap_new(app.as_raw())) }
    }
}

impl_widget!(Cursor, IdExt);
impl Cursor {
    pub fn new(app: &App, stock_cursor_id: i32) -> Self {
        unsafe { Self::from_raw(FXCursor_new(app.as_raw(), stock_cursor_id)) }
    }
}

impl_widget!(DCPrint, IdExt);
impl DCPrint {
    pub fn new(app: &App) -> Self {
        unsafe { Self::from_raw(FXDCPrint_new(app.as_raw())) }
    }
}
