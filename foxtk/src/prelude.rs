pub use {super::enums::*, std::sync::mpsc::Sender};
use {
    foxtk_sys::*,
    std::{
        ffi::{CString, c_long, c_void},
        sync::mpsc::channel,
    },
};

/// Helper function to convert a string to CString, handling null bytes gracefully
/// by replacing them with a placeholder character.
#[allow(dead_code)]
pub(crate) fn to_cstring(s: &str) -> CString {
    CString::new(s).unwrap_or_else(|_| {
        // Replace null bytes with underscore
        CString::new(s.replace('\0', "_")).unwrap()
    })
}

unsafe extern "C" fn ccallback<T: WindowExt>(ptr: *mut FXWindow, context: *mut c_void) -> c_long {
    unsafe {
        let func: &mut Box<dyn FnMut(T) -> bool> =
            &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr as *mut T::T)) as c_long
    }
}

pub trait ObjectExt: Sized {
    type T: 'static;
    fn as_raw(&self) -> *mut Self::T;
    fn from_raw(ptr: *mut Self::T) -> Self;
}

pub trait IdExt: ObjectExt {
    fn app(&self) -> super::App {
        super::App::from_raw(unsafe { FXId_get_app(self.as_raw() as *const FXId) })
    }
    #[cfg(target_os = "windows")]
    fn id(&self) -> *mut c_void {
        unsafe { FXId_get_id(self.as_raw() as *const FXId) }
    }
    #[cfg(target_os = "linux")]
    fn id(&self) -> u64 {
        unsafe { FXId_get_id(self.as_raw() as *const FXId) }
    }
}

pub trait DrawableExt: IdExt {
    fn height(&self) -> i32 {
        unsafe { FXDrawable_get_height(self.as_raw() as *const FXDrawable) }
    }
    fn width(&self) -> i32 {
        unsafe { FXDrawable_get_width(self.as_raw() as *const FXDrawable) }
    }
}

pub trait WindowExt: DrawableExt {
    fn set_layout(&self, layout: Layout) {
        unsafe {
            FXWindow_set_layout_hints(self.as_raw() as *mut FXWindow, layout as u32);
        }
    }
    fn set_enable(&self, enable: bool) {
        match enable {
            true => self.enable(),
            false => self.disable(),
        }
    }
    fn disable(&self) {
        unsafe {
            FXWindow_disable(self.as_raw() as *mut FXWindow);
        }
    }
    fn enable(&self) {
        unsafe {
            FXWindow_enable(self.as_raw() as *mut FXWindow);
        }
    }
    fn with_layout(self, layout: Layout) -> Self {
        self.set_layout(layout);
        self
    }
    fn set_selector(&self, selector: Selector) {
        unsafe {
            FXWindow_set_selector(self.as_raw() as *mut FXWindow, selector as i32);
        }
    }
    fn set_height(&self, height: i32) {
        unsafe {
            FXWindow_set_height(self.as_raw() as *mut FXWindow, height);
        };
        self.set_layout(match height {
            0 => Layout::Fill,
            _ => Layout::FixHeight,
        });
    }
    fn has_focus(&self) -> bool {
        unsafe { FXWindow_has_focus(self.as_raw() as *const FXWindow) != 0 }
    }
    fn open_file_dialog(&self, caption: &str, path: &str, patterns: &str, initial: i32) -> String {
        unsafe {
            let ptr = FXFileDialog_get_open_filename(
                self.root().as_raw() as *mut FXWindow,
                to_cstring(caption).as_ptr(),
                to_cstring(path).as_ptr(),
                to_cstring(patterns).as_ptr(),
                initial,
            );
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    fn save_file_dialog(&self, caption: &str, path: &str, patterns: &str, initial: i32) -> String {
        unsafe {
            let ptr = FXFileDialog_get_save_filename(
                self.root().as_raw() as *mut FXWindow,
                to_cstring(caption).as_ptr(),
                to_cstring(path).as_ptr(),
                to_cstring(patterns).as_ptr(),
                initial,
            );
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    fn set_width(&self, width: i32) {
        unsafe {
            FXWindow_set_width(self.as_raw() as *mut FXWindow, width);
        };
        self.set_layout(match width {
            0 => Layout::Fill,
            _ => Layout::FixWidth,
        });
    }
    fn set_size(&self, width: i32, height: i32) {
        self.set_height(height);
        self.set_width(width);
        self.set_layout(Layout::Normal);
    }
    fn with_size(self, width: i32, height: i32) -> Self {
        self.set_size(width, height);
        self
    }
    fn with_height(self, height: i32) -> Self {
        self.set_height(height);
        self.set_layout(match height {
            0 => Layout::Fill,
            _ => Layout::FillX,
        });
        self
    }
    fn with_width(self, width: i32) -> Self {
        self.set_width(width);
        self
    }
    fn with_selector(self, selector: Selector) -> Self {
        self.set_selector(selector);
        self
    }
    fn with_callback<F: FnMut(Self) -> bool + 'static>(self, func: F) -> Self {
        let raw_ptr: *mut Box<dyn FnMut(Self) -> bool> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            FXWindow_set_target(
                self.as_raw() as *mut FXWindow,
                Some(ccallback::<Self>),
                raw_ptr as *mut c_void,
            );
        }
        self
    }
    fn parent(&self) -> Self {
        Self::from_raw(unsafe {
            FXWindow_get_parent(self.as_raw() as *const FXWindow) as *mut Self::T
        })
    }
    fn root(&self) -> Self {
        Self::from_raw(unsafe {
            FXWindow_get_root(self.as_raw() as *const FXWindow) as *mut Self::T
        })
    }
    fn message(&self, opts: MessageBox, message: &str, kind: Message) -> u32 {
        unsafe {
            match kind {
                Message::Error => FXMessageBox_error(
                    self.root().as_raw() as *mut FXWindow,
                    opts as u32,
                    to_cstring("Error").as_ptr(),
                    to_cstring(message).as_ptr(),
                ),
                Message::Information => FXMessageBox_information(
                    self.root().as_raw() as *mut FXWindow,
                    opts as u32,
                    to_cstring("Information").as_ptr(),
                    to_cstring(message).as_ptr(),
                ),
                Message::Question => FXMessageBox_question(
                    self.root().as_raw() as *mut FXWindow,
                    opts as u32,
                    to_cstring("Question").as_ptr(),
                    to_cstring(message).as_ptr(),
                ),
                Message::Warning => FXMessageBox_warning(
                    self.root().as_raw() as *mut FXWindow,
                    opts as u32,
                    to_cstring("Warning").as_ptr(),
                    to_cstring(message).as_ptr(),
                ),
            }
        }
    }
}

pub trait DCWindowExt: ObjectExt {
    //~ fn new_dc(&self) -> Self {
    //~ unsafe { Self::from_raw(FXDCWindow_new(self.as_raw() as *mut FXDrawable)) }
    //~ }
    fn dc_set_foreground(&self, color: Color) {
        unsafe { FXDC_set_foreground(self.as_raw() as *mut FXDC, color.bits()) }
    }
    fn dc_set_line_width(&self, width: i32) {
        unsafe { FXDC_set_line_width(self.as_raw() as *mut FXDC, width) }
    }
    fn dc_draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe { FXDC_draw_line(self.as_raw() as *mut FXDC, x1, y1, x2, y2) }
    }
    fn dc_draw_point(&self, x: i32, y: i32) {
        unsafe { FXDC_draw_point(self.as_raw() as *mut FXDC, x, y) }
    }
    fn dc_draw_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { FXDC_draw_rect(self.as_raw() as *mut FXDC, x, y, w, h) }
    }
    fn dc_fill_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { FXDC_fill_rect(self.as_raw() as *mut FXDC, x, y, w, h) }
    }
}

pub trait FrameExt: WindowExt {
    fn with_pad(self, pad: i32) -> Self {
        unsafe {
            FXFrame_set_pad_bottom(self.as_raw() as *mut FXFrame, pad);
            FXFrame_set_pad_right(self.as_raw() as *mut FXFrame, pad);
            FXFrame_set_pad_left(self.as_raw() as *mut FXFrame, pad);
            FXFrame_set_pad_top(self.as_raw() as *mut FXFrame, pad);
        }
        self
    }
    fn with_frame(self, frame: FrameStyle) -> Self {
        unsafe {
            FXFrame_set_style(self.as_raw() as *mut FXFrame, frame as u32);
        }
        self
    }
}

pub trait TextableExt: FrameExt {
    fn text(&self) -> String;
    fn set_font(&self, family: &str, size: i32);
    fn set_text(&self, text: &str);
    fn set_text_color(&self, color: Color);
    fn with_font(self, family: &str, size: i32) -> Self {
        self.set_font(family, size);
        self
    }
}

pub trait EditableExt: ObjectExt {
    fn set_editable(&self, editable: bool);
    fn with_editable(self, editable: bool) -> Self {
        self.set_editable(editable);
        self
    }
}

pub trait CompositeExt: WindowExt {
    fn inside(&self, mut func: impl FnMut(&Self)) {
        func(self);
    }
    fn child_width(&self) -> i32 {
        unsafe { FXComposite_child_width(self.as_raw() as *const FXComposite) }
    }
    fn child_height(&self) -> i32 {
        unsafe { FXComposite_child_height(self.as_raw() as *const FXComposite) }
    }
}

pub trait PackerExt: CompositeExt {
    fn set_hspacing(&self, val: i32) {
        unsafe {
            FXPacker_set_hspacing(self.as_raw() as *mut FXPacker, val);
        }
    }
    fn set_vspacing(&self, val: i32) {
        unsafe {
            FXPacker_set_vspacing(self.as_raw() as *mut FXPacker, val);
        }
    }
    fn with_spacing(self, spacing: i32) -> Self {
        self.set_vspacing(spacing);
        self.set_hspacing(spacing);
        self
    }
}

pub trait SelectorExt: PackerExt {
    fn append_item(&self, text: &str);
    fn clear_items(&self);
    fn set_current_item(&self, index: i32);
    fn set_num_visible(&self, num_visible: i32);
    fn current_item(&self) -> i32;
    fn num_items(&self) -> i32;
    fn item_text(&self, index: i32) -> String;
    fn append_items(&self, items: &[&str]) {
        for text in items {
            self.append_item(text);
        }
    }
    fn with_num_visible(self, num_visible: i32) -> Self {
        self.set_num_visible(num_visible);
        self
    }
    fn with_item(self, text: &str) -> Self {
        self.append_item(text);
        self
    }
    fn with_items(self, items: &[&str]) -> Self {
        self.append_items(items);
        self
    }
}

pub trait TopWindowExt: WindowExt {
    fn set_hspacing(&self, spacing: i32) {
        unsafe {
            FXTopWindow_set_hspacing(self.as_raw() as *mut FXTopWindow, spacing);
        }
    }
    fn set_vspacing(&self, spacing: i32) {
        unsafe {
            FXTopWindow_set_vspacing(self.as_raw() as *mut FXTopWindow, spacing);
        }
    }
    fn with_hspacing(self, spacing: i32) -> Self {
        self.set_hspacing(spacing);
        self
    }
    fn with_vspacing(self, spacing: i32) -> Self {
        self.set_vspacing(spacing);
        self
    }
}

pub trait Update<T>
where
    Self: 'static,
{
    fn update(&self, value: T);
}

impl Update<i32> for super::Slider {
    fn update(&self, value: i32) {
        if !self.has_focus() && self.value() != value {
            self.set_value(value);
        };
    }
}

impl Update<i32> for super::Spinner {
    fn update(&self, value: i32) {
        if !self.has_focus() && self.value() != value {
            self.set_value(value);
        };
    }
}

impl Update<&String> for super::TextField {
    fn update(&self, value: &String) {
        if !self.has_focus() && self.text() != *value {
            self.set_text(value);
        };
    }
}

impl Update<&String> for super::Text {
    fn update(&self, value: &String) {
        if !self.has_focus() && self.text() != *value {
            self.set_text(value);
        };
    }
}

impl<T: SelectorExt + 'static> Update<i32> for T {
    fn update(&self, value: i32) {
        if !self.has_focus() && self.current_item() != value {
            self.set_current_item(value);
        };
    }
}

impl<T: SelectorExt + 'static> Update<(Vec<String>, i32)> for T {
    fn update(&self, value: (Vec<String>, i32)) {
        if !self.has_focus() {
            if self.num_items() != value.0.len() as i32 {
                self.clear_items();
                if !value.0.is_empty() {
                    for item in &value.0 {
                        self.append_item(item);
                    }
                }
            };
            if self.num_items() > value.1 && self.current_item() != value.1 {
                self.set_current_item(value.1);
            };
        }
    }
}

pub trait RangerExt: WindowExt {
    fn value(&self) -> i32;
    fn range(&self) -> (i32, i32);
    fn set_value(&self, value: i32);
    fn set_range(&self, low: i32, high: i32);
    fn with_range(self, low: i32, high: i32) -> Self {
        self.set_range(low, high);
        self
    }
    fn with_value(self, value: i32) -> Self {
        self.set_value(value);
        self
    }
}

pub trait Component: Default + 'static {
    type Event: 'static;
    type State: Default + 'static;
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool;
    fn update(&self, model: &Self::State);
    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>);
    fn mount(parent: &impl CompositeExt) {
        let (sender, receiver) = channel::<Self::Event>();
        let mut page = Self::default();
        let mut model = Self::State::default();
        page.view(parent, sender.clone());
        page.update(&model);
        parent.app().add_timeout(400, move |_| {
            let mut update = false;
            while let Ok(msg) = receiver.try_recv() {
                update = Self::handle(msg, &mut model, sender.clone()) || update;
            }
            if update {
                page.update(&model);
            }
            true
        });
    }
    fn run(name: &str, vendor: &str, title: &str, width: i32, height: i32) -> i32 {
        let app = super::App::new(name, vendor);
        let win = super::MainWindow::new(&app, title, width, height);
        Self::mount(&win);
        win.show();
        app.run()
    }
}
