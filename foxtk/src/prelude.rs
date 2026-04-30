pub use std::sync::mpsc::Sender;
use {
    foxtk_sys::*,
    std::{
        ffi::{CString, c_void},
        sync::mpsc::channel,
    },
};

unsafe extern "C" fn ccallback<T: ObjectExt>(ptr: ObjectPtr, context: *mut c_void) -> i64 {
    unsafe {
        let func: &mut Box<dyn FnMut(T) -> bool> =
            &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr)) as i64
    }
}

unsafe extern "C" fn ctimer<T: AppExt>(ptr: ObjectPtr, context: *mut c_void) -> i64 {
    unsafe {
        let func: &mut Box<dyn FnMut(T) -> bool> =
            &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr)) as i64
    }
}

pub trait SwitcherExt: PackerExt {
    fn set_curent(&self, idx: i32) {
        unsafe { fx_switcher_set_current(self.as_raw(), idx) }
    }
}

pub trait ObjectExt: Sized {
    fn as_raw(&self) -> ObjectPtr;
    fn from_raw(ptr: ObjectPtr) -> Self;
}

pub trait LabelExt: ObjectExt {
    fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            fx_label_set_text(self.as_raw(), c_text.as_ptr());
        }
    }

    fn text(&self) -> String {
        unsafe {
            let ptr = fx_label_get_text(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }

    fn set_justify(&self, justify: u32) {
        unsafe {
            fx_label_set_justify(self.as_raw(), justify);
        }
    }

    fn justify(&self) -> u32 {
        unsafe { fx_label_get_justify(self.as_raw()) }
    }
}
pub trait TreeListExt: ObjectExt {
    fn add_item_first(&self, parent_item: Option<&super::TreeItem>, text: &str) -> super::TreeItem {
        let c_text = CString::new(text).unwrap();
        unsafe {
            super::TreeItem::from_raw(fx_tree_list_append_item(
                self.as_raw(),
                parent_item
                    .map(|i| i.as_raw())
                    .unwrap_or(std::ptr::null_mut()),
                c_text.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
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
        let c_text = CString::new(text).unwrap();
        unsafe {
            fx_text_set_text(self.as_raw(), c_text.as_ptr());
        }
    }

    fn text(&self) -> String {
        unsafe {
            let ptr = fx_text_get_text(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

pub trait TableExt: ObjectExt {
    fn set_table_size(&self, rows: i32, cols: i32) {
        unsafe {
            fx_table_set_table_size(self.as_raw(), rows, cols);
        }
    }

    fn set_item_text(&self, row: i32, col: i32, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            fx_table_set_item_text(self.as_raw(), row, col, c_text.as_ptr());
        }
    }

    fn item_text(&self, row: i32, col: i32) -> String {
        unsafe {
            let ptr = fx_table_get_item_text(self.as_raw(), row, col);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

pub trait ScrollBarExt: ObjectExt {
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
pub trait CompositeExt: IdExt {
    fn inside(self, mut func: impl FnMut(&Self)) -> Self {
        func(&self);
        self
    }
}
pub trait GroupBoxExt: CompositeExt {
    fn set_style(&self, val: usize) {
        unsafe {
            fx_groupbox_set_style(self.as_raw(), val as u32);
        }
    }
}

impl SelectorExt for super::ListBox {
    fn append_item(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            fx_list_box_append_item(self.as_raw(), c_text.as_ptr());
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
        let c_text = CString::new(text).unwrap();
        unsafe {
            fx_list_append_item(self.as_raw(), c_text.as_ptr());
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
        let c_text = CString::new(text).unwrap();
        unsafe {
            fx_combo_box_append_item(self.as_raw(), c_text.as_ptr());
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
}

pub enum Trigger {
    COMMAND = 0,
    CHANGED,
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
    fn set_trigger(&self, val: Trigger) {
        unsafe {
            fx_window_set_selector(self.as_raw(), val as i32);
        }
    }
    fn set_layout_hints(&self, val: u32) {
        unsafe {
            fx_window_set_layout_hints(self.as_raw(), val);
        }
    }
    fn set_height(&self, val: i32) {
        unsafe {
            fx_window_set_height(self.as_raw(), val);
        }
    }
    fn has_focus(&self) -> bool {
        unsafe { fx_window_has_focus(self.as_raw()) != 0 }
    }
    fn set_width(&self, val: i32) {
        unsafe {
            fx_window_set_width(self.as_raw(), val);
        }
    }
    fn with_height(self, val: i32) -> Self {
        self.set_height(val);
        self
    }
    fn with_width(self, val: i32) -> Self {
        self.set_width(val);
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
}
pub trait TextFieldExt: WindowExt {
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
    fn set_editable(&self, val: bool) {
        unsafe {
            fx_textfield_set_editable(self.as_raw(), val as i64);
        }
    }
    fn with_editable(self, val: bool) -> Self {
        self.set_editable(val);
        self
    }
    fn set_text(&self, text_: &str) {
        let text = std::ffi::CString::new(text_).unwrap();
        unsafe { fx_textfield_set_text(self.as_raw(), text.as_ptr()) };
    }
}
pub trait SpinnerExt: WindowExt {
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
pub trait ProgressBarExt: WindowExt {
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
    fn set_progress(&self, value: u32) {
        unsafe { fx_progressbar_set_progress(self.as_raw(), value) }
    }
    fn set_value(&self, value: u32) {
        self.set_progress(value);
    }
    fn set_total(&self, value: u32) {
        unsafe { fx_progressbar_set_total(self.as_raw(), value) }
    }
    fn set_bar_size(&self, size: i32) {
        unsafe { fx_progressbar_set_bar_size(self.as_raw(), size) }
    }
    fn with_total(self, value: u32) -> Self {
        self.set_total(value);
        self
    }
}
pub trait ButtonExt: LabelExt {
    fn new(parent: &impl ObjectExt, title_: &str) -> Self {
        let title = std::ffi::CString::new(format!("&{title_}").as_str()).unwrap();
        Self::from_raw(unsafe { fx_button_new(parent.as_raw(), title.as_ptr()) })
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

pub trait MainWindowExt: WindowExt {
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
        let win = super::MainWindow::new(&app, title, 360, 640);
        Self::mount(&win);
        win.show();
        app.run()
    }
}
