pub use std::sync::mpsc::Sender;
use {
    foxtk_sys::*,
    std::{
        ffi::{CString, c_long, c_void},
        sync::mpsc::channel,
    },
};

pub struct Color(u32);
impl Color {
    pub fn from_hex(hex: u32) -> Self {
        Self::from_rgb((hex >> 16) & 0xff, (hex >> 8) & 0xff, hex & 0xff)
    }
    pub fn from_rgb(r: u32, g: u32, b: u32) -> Self {
        Self(unsafe { fx_rgb(r, g, b) })
    }
    pub fn from_rgba(r: u32, g: u32, b: u32, a: u32) -> Self {
        Self(unsafe { fx_rgba(r, g, b, a) })
    }
    pub fn bits(&self) -> u32 {
        self.0
    }
}

pub enum Message {
    Error = 0,
    Warning,
    Question,
    Information,
}

pub enum MessageBox {
    Ok = 0x10000000,
    OkCancel = 0x20000000,
}

pub enum Justify {
    Normal = 0,
    Left = 0x00008000,
    Right = 0x00010000,
    Top = 0x00020000,
    Bottom = 0x00040000,
    HzApart = 0x00008000 | 0x00010000,
    VtApart = 0x00020000 | 0x00040000,
}

pub enum Layout {
    Normal = 0,
    FillX = 0x00000400,
    FillY = 0x00000800,
    Fill = 0x00000400 | 0x00000800,
    FixWidth = 0x00000100,
    FixHeight = 0x00000200,
}

pub enum Frame {
    None = 0,
    Sunken = 0x00001000,
    Raised = 0x00002000,
    Thick = 0x00004000,
    Ridge = 0x00004000 | 0x00002000 | 0x00001000,
    Line = 0x00002000 | 0x00001000,
    Normal = 0x00000800 | 0x00004000,
}

pub enum Trigger {
    COMMAND = 0,
    CHANGED,
}

unsafe extern "C" fn ccallback<T: ObjectExt>(ptr: *mut ObjectPtr, context: *mut c_void) -> c_long {
    unsafe {
        let func: &mut Box<dyn FnMut(T) -> bool> =
            &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr)) as c_long
    }
}

unsafe extern "C" fn ctimer<T: AppExt>(ptr: *mut ObjectPtr, context: *mut c_void) -> c_long {
    unsafe {
        let func: &mut Box<dyn FnMut(T) -> bool> =
            &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr)) as c_long
    }
}

pub trait SwitcherExt: PackerExt {
    fn set_curent(&self, idx: i32) {
        unsafe { fx_switcher_set_current(self.as_raw(), idx) }
    }
}

pub trait ObjectExt: Sized {
    fn as_raw(&self) -> *mut ObjectPtr;
    fn from_raw(ptr: *mut ObjectPtr) -> Self;
}

pub trait LabelExt: FrameExt {
    fn set_text(&self, text: &str) {
        unsafe {
            fx_label_set_text(self.as_raw(), CString::new(text).unwrap().as_ptr());
        }
    }
    fn set_tip(&self, tip: &str) {
        unsafe {
            fx_label_set_tip_text(self.as_raw(), CString::new(tip).unwrap().as_ptr());
        }
    }
    fn with_tip(self, tip: &str) -> Self {
        self.set_tip(tip);
        self
    }
    fn set_help(&self, help: &str) {
        unsafe {
            fx_label_set_help_text(self.as_raw(), CString::new(help).unwrap().as_ptr());
        }
    }
    fn text(&self) -> String {
        unsafe {
            let ptr = fx_label_get_text(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }

    fn set_justify(&self, justify: Justify) {
        unsafe {
            fx_label_set_justify(self.as_raw(), justify as u32);
        }
    }

    fn with_justify(self, justify: Justify) -> Self {
        self.set_justify(justify);
        self
    }

    fn justify(&self) -> u32 {
        unsafe { fx_label_get_justify(self.as_raw()) }
    }
    fn set_text_color(&self, color: Color) {
        unsafe {
            fx_label_set_text_color(self.as_raw(), color.bits());
        }
    }
}
pub trait TreeListExt: ObjectExt {
    fn add_item_first(&self, prt: &super::TreeItem, text: &str) -> super::TreeItem {
        unsafe {
            super::TreeItem::from_raw(fx_tree_list_append_item(
                self.as_raw(),
                prt.as_raw(),
                CString::new(text).unwrap().as_ptr(),
            ))
        }
    }

    fn clear_items(&self) {
        unsafe {
            fx_tree_list_clear_items(self.as_raw());
        }
    }
}

pub trait TextExt: ObjectExt {
    fn set_text(&self, text: &str) {
        unsafe {
            fx_text_set_text(self.as_raw(), CString::new(text).unwrap().as_ptr());
        }
    }
    fn text(&self) -> String {
        unsafe {
            let ptr = fx_text_get_text(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    fn set_editable(&self, val: bool) {
        unsafe {
            fx_text_set_editable(self.as_raw(), val as c_long);
        }
    }
    fn with_editable(self, val: bool) -> Self {
        self.set_editable(val);
        self
    }
}

pub trait TableExt: ObjectExt {
    fn set_table_size(&self, rows: i32, cols: i32) {
        unsafe {
            fx_table_set_table_size(self.as_raw(), rows, cols);
        }
    }

    fn set_item_text(&self, row: i32, col: i32, text: &str) {
        unsafe {
            fx_table_set_item_text(
                self.as_raw(),
                row,
                col,
                CString::new(text).unwrap().as_ptr(),
            );
        }
    }

    fn item_text(&self, row: i32, col: i32) -> String {
        unsafe {
            let ptr = fx_table_get_item_text(self.as_raw(), row, col);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

pub trait ScrollBarExt: WindowExt {
    fn position(&self) -> i32 {
        unsafe { fx_scroll_bar_get_position(self.as_raw()) }
    }
    fn set_position(&self, pos: i32) {
        unsafe {
            fx_scroll_bar_set_position(self.as_raw(), pos);
        }
    }
    fn set_range(&self, hi: i32) {
        unsafe {
            fx_scroll_bar_set_range(self.as_raw(), hi);
        }
    }
}

pub trait CompositeExt: WindowExt {
    fn inside(self, mut func: impl FnMut(&Self)) -> Self {
        func(&self);
        self
    }
}

impl SelectorExt for super::ListBox {
    fn append_item(&self, text: &str) {
        unsafe {
            fx_list_box_append_item(self.as_raw(), CString::new(text).unwrap().as_ptr());
        }
    }
    fn clear_items(&self) {
        unsafe {
            fx_list_box_clear_items(self.as_raw());
        }
    }

    fn current_item(&self) -> i32 {
        unsafe { fx_list_box_get_current_item(self.as_raw()) }
    }

    fn set_current_item(&self, index: i32) {
        unsafe {
            fx_list_box_set_current_item(self.as_raw(), index);
        }
    }

    fn item_text(&self, index: i32) -> String {
        unsafe {
            let ptr = fx_list_box_get_item_text(self.as_raw(), index);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }

    fn num_items(&self) -> i32 {
        unsafe { fx_list_box_get_num_items(self.as_raw()) }
    }
}

impl SelectorExt for super::List {
    fn append_item(&self, text: &str) {
        unsafe {
            fx_list_append_item(self.as_raw(), CString::new(text).unwrap().as_ptr());
        }
    }
    fn clear_items(&self) {
        unsafe {
            fx_list_clear_items(self.as_raw());
        }
    }

    fn current_item(&self) -> i32 {
        unsafe { fx_list_get_current_item(self.as_raw()) }
    }

    fn set_current_item(&self, index: i32) {
        unsafe {
            fx_list_box_set_current_item(self.as_raw(), index);
        }
    }

    fn item_text(&self, index: i32) -> String {
        unsafe {
            let ptr = fx_list_get_item_text(self.as_raw(), index);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    fn num_items(&self) -> i32 {
        unsafe { fx_list_get_num_items(self.as_raw()) }
    }
}

pub trait PackerExt: CompositeExt {
    fn set_hspacing(&self, val: i32) {
        unsafe {
            fx_packer_set_hspacing(self.as_raw(), val);
        }
    }
    fn set_vspacing(&self, val: i32) {
        unsafe {
            fx_packer_set_vspacing(self.as_raw(), val);
        }
    }
    fn with_spacing(self, spacing: i32) -> Self {
        self.set_vspacing(spacing);
        self.set_hspacing(spacing);
        self
    }
}

pub trait GroupBoxExt: PackerExt {
    fn set_style(&self, val: Frame) {
        unsafe {
            fx_groupbox_set_style(self.as_raw(), val as u32);
        }
    }
    fn with_style(self, val: Frame) -> Self {
        self.set_style(val);
        self
    }
}

pub trait SelectorExt: PackerExt {
    fn append_item(&self, text: &str);
    fn clear_items(&self);
    fn current_item(&self) -> i32;
    fn set_current_item(&self, index: i32);
    fn num_items(&self) -> i32;
    fn item_text(&self, index: i32) -> String;
    fn append_items(&self, items: &[&str]) {
        for text in items {
            self.append_item(text);
        }
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

impl SelectorExt for super::ComboBox {
    fn append_item(&self, text: &str) {
        unsafe {
            fx_combo_box_append_item(self.as_raw(), CString::new(text).unwrap().as_ptr());
        }
    }

    fn clear_items(&self) {
        unsafe {
            fx_combo_box_clear_items(self.as_raw());
        }
    }

    fn current_item(&self) -> i32 {
        unsafe { fx_combo_box_get_current_item(self.as_raw()) }
    }

    fn set_current_item(&self, index: i32) {
        unsafe {
            fx_combo_box_set_current_item(self.as_raw(), index);
        }
    }

    fn item_text(&self, index: i32) -> String {
        unsafe {
            let ptr = fx_combo_box_get_item_text(self.as_raw(), index);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }

    fn num_items(&self) -> i32 {
        unsafe { fx_combo_box_get_num_items(self.as_raw()) }
    }
}

pub trait AppExt: ObjectExt {
    fn add_timeout<F: FnMut(Self) -> bool + 'static>(&self, ms: u32, func: F) {
        let raw_ptr: *mut Box<dyn FnMut(Self) -> bool> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            fx_app_add_timeout(
                self.as_raw(),
                Some(ctimer::<Self>),
                ms,
                raw_ptr as *mut c_void,
            );
        }
    }
    fn add_chore<F: FnMut(Self) -> bool + 'static>(&self, func: F) {
        let raw_ptr: *mut Box<dyn FnMut(Self) -> bool> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            fx_app_add_chore(self.as_raw(), Some(ctimer::<Self>), raw_ptr as *mut c_void);
        }
    }
    fn run(&self) -> i32 {
        unsafe { fx_app_run(self.as_raw()) }
    }
}

pub trait IdExt: ObjectExt {
    fn app(&self) -> impl AppExt {
        super::App::from_raw(unsafe { fx_id_get_app(self.as_raw()) })
    }
    #[cfg(target_os = "windows")]
    fn id(&self) -> u32 {
        unsafe { fx_id_get_id(self.as_raw()) }
    }
    #[cfg(target_os = "linux")]
    fn id(&self) -> u64 {
        unsafe { fx_id_get_id(self.as_raw()) }
    }
}

pub trait WindowExt: IdExt {
    fn set_callback<F: FnMut(Self) -> bool + 'static>(&self, func: F) {
        let raw_ptr: *mut Box<dyn FnMut(Self) -> bool> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            fx_window_set_target(
                self.as_raw(),
                Some(ccallback::<Self>),
                raw_ptr as *mut c_void,
            );
        }
    }
    fn set_layout(&self, layout: Layout) {
        unsafe {
            fx_window_set_layout_hints(self.as_raw(), layout as u32);
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
            fx_window_disable(self.as_raw());
        }
    }
    fn enable(&self) {
        unsafe {
            fx_window_enable(self.as_raw());
        }
    }
    fn with_layout(self, layout: Layout) -> Self {
        self.set_layout(layout);
        self
    }
    fn set_trigger(&self, trigger: Trigger) {
        unsafe {
            fx_window_set_selector(self.as_raw(), trigger as i32);
        }
    }
    fn set_height(&self, height: i32) {
        unsafe {
            fx_window_set_height(self.as_raw(), height);
        };
    }
    fn has_focus(&self) -> bool {
        unsafe { fx_window_has_focus(self.as_raw()) != 0 }
    }
    fn set_width(&self, width: i32) {
        unsafe {
            fx_window_set_width(self.as_raw(), width);
        };
    }
    fn set_size(&self, width: i32, height: i32) {
        self.set_height(height);
        self.set_width(width);
    }
    fn with_size(self, width: i32, height: i32) -> Self {
        self.set_size(width, height);
        self.set_layout(Layout::Normal);
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
        self.set_layout(match width {
            0 => Layout::Fill,
            _ => Layout::FixWidth,
        });
        self
    }
    fn with_trigger(self, val: Trigger) -> Self {
        self.set_trigger(val);
        self
    }
    fn with_callback<F: FnMut(Self) -> bool + 'static>(self, func: F) -> Self {
        self.set_callback(func);
        self
    }
    fn parent(&self) -> Self {
        Self::from_raw(unsafe { fx_window_get_parent(self.as_raw()) })
    }
    fn root(&self) -> super::MainWindow {
        super::MainWindow::from_raw(unsafe { fx_window_get_root(self.as_raw()) })
    }
    fn message(&self, opts: MessageBox, message: &str, kind: Message) -> u32 {
        unsafe {
            match kind {
                Message::Error => fx_message_box_error(
                    self.root().as_raw(),
                    opts as u32,
                    CString::new("Error").unwrap().as_ptr(),
                    CString::new(message).unwrap().as_ptr(),
                ),
                Message::Information => fx_message_box_information(
                    self.root().as_raw(),
                    opts as u32,
                    CString::new("Information").unwrap().as_ptr(),
                    CString::new(message).unwrap().as_ptr(),
                ),
                Message::Question => fx_message_box_question(
                    self.root().as_raw(),
                    opts as u32,
                    CString::new("Question").unwrap().as_ptr(),
                    CString::new(message).unwrap().as_ptr(),
                ),
                Message::Warning => fx_message_box_warning(
                    self.root().as_raw(),
                    opts as u32,
                    CString::new("Warning").unwrap().as_ptr(),
                    CString::new(message).unwrap().as_ptr(),
                ),
            }
        }
    }
}

pub trait FrameExt: WindowExt {
    fn set_pad_top(&self, pad: i32) {
        unsafe {
            fx_frame_set_pad_top(self.as_raw(), pad);
        }
    }
    fn set_pad_left(&self, pad: i32) {
        unsafe {
            fx_frame_set_pad_left(self.as_raw(), pad);
        }
    }
    fn set_pad_right(&self, pad: i32) {
        unsafe {
            fx_frame_set_pad_right(self.as_raw(), pad);
        }
    }
    fn set_pad_bottom(&self, pad: i32) {
        unsafe {
            fx_frame_set_pad_bottom(self.as_raw(), pad);
        }
    }
    fn set_pad(&self, pad: i32) {
        self.set_pad_bottom(pad);
        self.set_pad_left(pad);
        self.set_pad_right(pad);
        self.set_pad_top(pad);
    }
    fn with_pad(self, pad: i32) -> Self {
        self.set_pad(pad);
        self
    }
    fn set_frame(&self, frame: Frame) {
        unsafe {
            fx_frame_set_frame_style(self.as_raw(), frame as u32);
        }
    }
    fn with_frame(self, frame: Frame) -> Self {
        self.set_frame(frame);
        self
    }
}

pub trait TextFieldExt: FrameExt {
    fn set_editable(&self, val: bool) {
        unsafe {
            fx_textfield_set_editable(self.as_raw(), val as c_long);
        }
    }
    fn set_text(&self, text_: &str) {
        let text = std::ffi::CString::new(text_).unwrap();
        unsafe { fx_textfield_set_text(self.as_raw(), text.as_ptr()) };
    }
    fn set_tip(&self, text: &str) {
        unsafe {
            fx_textfield_set_tip_text(self.as_raw(), CString::new(text).unwrap().as_ptr());
        }
    }
    fn set_help(&self, text: &str) {
        unsafe {
            fx_textfield_set_help_text(self.as_raw(), CString::new(text).unwrap().as_ptr());
        }
    }
    fn set_text_color(&self, color: Color) {
        unsafe {
            fx_textfield_set_text_color(self.as_raw(), color.bits());
        }
    }
    fn text(&self) -> String {
        unsafe {
            let ptr = fx_textfield_get_text(self.as_raw());
            if !ptr.is_null() {
                std::ffi::CStr::from_ptr(ptr).to_string_lossy().to_string()
            } else {
                String::new()
            }
        }
    }
    fn with_help(self, text: &str) -> Self {
        self.set_help(text);
        self
    }
    fn with_tip(self, text: &str) -> Self {
        self.set_tip(text);
        self
    }
    fn with_editable(self, val: bool) -> Self {
        self.set_editable(val);
        self
    }
}

pub trait Update<T>
where
    Self: 'static,
{
    fn update(&self, value: T);
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

pub trait SpinnerExt: PackerExt {
    fn value(&self) -> i32 {
        unsafe { fx_spinner_get_value(self.as_raw()) }
    }
    fn range(&self) -> (i32, i32) {
        let mut lo = 0;
        let mut hi = 0;
        unsafe { fx_spinner_get_range(self.as_raw(), &mut lo, &mut hi) };
        (lo, hi)
    }
    fn increment(&self) {
        unsafe { fx_spinner_increment(self.as_raw()) }
    }
    fn decrement(&self) {
        unsafe { fx_spinner_decrement(self.as_raw()) }
    }
    fn set_value(&self, value: i32) {
        unsafe { fx_spinner_set_value(self.as_raw(), value) }
    }
    fn set_range(&self, low: i32, high: i32) {
        unsafe { fx_spinner_set_range(self.as_raw(), low, high) }
    }
    fn set_increment(&self, inc: i32) {
        unsafe { fx_spinner_set_increment(self.as_raw(), inc) }
    }
    fn with_range(self, low: i32, high: i32) -> Self {
        self.set_range(low, high);
        self
    }
    fn with_increment(self, inc: i32) -> Self {
        self.set_increment(inc);
        self
    }
    fn with_value(self, value: i32) -> Self {
        self.set_value(value);
        self
    }
}
pub trait SliderExt: WindowExt {
    fn value(&self) -> i32 {
        unsafe { fx_slider_get_value(self.as_raw()) }
    }
    fn set_value(&self, value: i32) {
        unsafe { fx_slider_set_value(self.as_raw(), value) }
    }
    fn range(&self) -> (i32, i32) {
        let mut lo = 0;
        let mut hi = 0;
        unsafe { fx_slider_get_range(self.as_raw(), &mut lo, &mut hi) };
        (lo, hi)
    }
    fn set_range(&self, low: i32, high: i32) {
        unsafe { fx_slider_set_range(self.as_raw(), low, high) }
    }
    fn with_range(self, low: i32, high: i32) -> Self {
        self.set_range(low, high);
        self
    }
    fn increment(&self) -> i32 {
        unsafe { fx_slider_get_increment(self.as_raw()) }
    }
    fn set_increment(&self, inc: i32) {
        unsafe { fx_slider_set_increment(self.as_raw(), inc) }
    }
    fn with_increment(self, inc: i32) -> Self {
        self.set_increment(inc);
        self
    }
}
pub trait ProgressBarExt: FrameExt {
    fn progress(&self) -> u32 {
        unsafe { fx_progressbar_get_progress(self.as_raw()) }
    }
    fn total(&self) -> u32 {
        unsafe { fx_progressbar_get_total(self.as_raw()) }
    }
    fn increment(&self, value: u32) {
        unsafe { fx_progressbar_increment(self.as_raw(), value) }
    }
    fn show_number(&self) {
        unsafe { fx_progressbar_show_number(self.as_raw()) }
    }
    fn hide_number(&self) {
        unsafe { fx_progressbar_hide_number(self.as_raw()) }
    }
    fn bar_size(&self) -> i32 {
        unsafe { fx_progressbar_get_bar_size(self.as_raw()) }
    }
    fn set_progress(&self, progress: u32) {
        unsafe { fx_progressbar_set_progress(self.as_raw(), progress) }
    }
    fn set_value(&self, value: u32) {
        self.set_progress(value);
    }
    fn set_total(&self, total: u32) {
        unsafe { fx_progressbar_set_total(self.as_raw(), total) }
    }
    fn set_bar_size(&self, size: i32) {
        unsafe { fx_progressbar_set_bar_size(self.as_raw(), size) }
    }
    fn with_total(self, value: u32) -> Self {
        self.set_total(value);
        self
    }
}

pub trait RadioButtonExt: LabelExt {
    fn check(&self) -> bool {
        unsafe { fx_radio_button_get_check(self.as_raw()) != 0 }
    }
    fn set_check(&self) {
        unsafe { fx_radio_button_set_check(self.as_raw()) }
    }
}

pub trait CheckButtonExt: LabelExt {
    fn check(&self) -> bool {
        unsafe { fx_check_button_get_check(self.as_raw()) != 0 }
    }
    fn set_check(&self, check: bool) {
        unsafe { fx_check_button_set_check(self.as_raw(), check as u8) }
    }
}

pub trait MainWindowExt: CompositeExt {
    fn show(&self) {
        unsafe { fx_main_window_show(self.as_raw()) }
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
        parent.app().add_timeout(200, move |_| {
            if let Ok(msg) = receiver.try_recv()
                && Self::handle(msg, &mut model, sender.clone())
            {
                page.update(&model);
            }
            true
        });
    }
    fn run(name: &str, vendor: &str, title: &str) -> i32 {
        let app = super::App::new(name, vendor);
        let win = super::MainWindow::new(&app, title, 400, 640);
        Self::mount(&win);
        win.show();
        app.run()
    }
}
