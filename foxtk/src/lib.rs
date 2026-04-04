pub mod app;
pub mod button;
pub mod canvas;
pub mod checkbutton;
pub mod choicebox;
pub mod frame;
pub mod label;
pub mod listbox;
pub mod menubar;
pub mod prelude;
pub mod progressbar;
pub mod radiobutton;
pub mod scrollbar;
pub mod slider;
pub mod spinner;
pub mod tabbook;
pub mod table;
pub mod text;
pub mod textfield;
pub mod treelist;
pub mod window;
use std::{
    ffi::CString,
    os::raw::{c_char, c_int},
    sync::mpsc::channel,
};
pub use {
    app::App,
    button::Button,
    canvas::Canvas,
    checkbutton::CheckButton,
    choicebox::ComboBox,
    frame::HorizontalFrame,
    frame::VerticalFrame,
    label::Label,
    listbox::ListBox,
    menubar::{MenuBar, MenuCommand, MenuPane, MenuTitle},
    progressbar::ProgressBar,
    radiobutton::RadioButton,
    scrollbar::ScrollBar,
    slider::RangeSlider,
    spinner::Spinner,
    std::sync::mpsc::Sender,
    tabbook::{TabBook, TabItem},
    table::Table,
    text::Text,
    textfield::TextField,
    treelist::TreeList,
    window::MainWindow,
};

unsafe extern "C" fn ccallback<T: ObjectExt>(
    ptr: foxtk_sys::ObjectPtr,
    context: *mut std::os::raw::c_void,
) -> std::os::raw::c_long {
    unsafe {
        let func: &mut Box<dyn FnMut(T) -> bool> =
            &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr)) as std::os::raw::c_long
    }
}

unsafe extern "C" fn ctimer<T: AppExt>(
    ptr: foxtk_sys::ObjectPtr,
    context: *mut std::os::raw::c_void,
) -> std::os::raw::c_long {
    unsafe {
        let func: &mut Box<dyn FnMut(T) -> bool> =
            &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr)) as std::os::raw::c_long
    }
}

pub trait ObjectExt: Sized {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr;
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self;
}

pub trait TreeListExt: ObjectExt {
    fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self::from_raw(foxtk_sys::fx_tree_list_new(
                parent.as_raw(),
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ))
        }
    }
    fn add_item_first(
        &self,
        parent_item: Option<&treelist::TreeItem>,
        text: &str,
    ) -> treelist::TreeItem {
        let c_text = CString::new(text).unwrap();
        unsafe {
            treelist::TreeItem::from_raw(foxtk_sys::fx_tree_list_append_item(
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
            foxtk_sys::fx_tree_list_clear_items(self.as_raw());
        }
    }
}

pub trait TextExt: ObjectExt {
    fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self::from_raw(foxtk_sys::fx_text_new(
                parent.as_raw(),
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ))
        }
    }
    fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            foxtk_sys::fx_text_set_text(self.as_raw(), c_text.as_ptr());
        }
    }

    fn get_text(&self) -> String {
        unsafe {
            let ptr = foxtk_sys::fx_text_get_text(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

pub trait TabItemExt: ObjectExt {
    fn new(parent: &impl WindowExt, text: &str) -> Self {
        let c_text = CString::new(text).unwrap();
        unsafe {
            Self::from_raw(foxtk_sys::fx_tab_item_new(
                parent.as_raw(),
                c_text.as_ptr(),
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ))
        }
    }
}

pub trait TabBookExt: ObjectExt {
    fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self::from_raw(foxtk_sys::fx_tab_book_new(
                parent.as_raw(),
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ))
        }
    }
}
pub trait TableExt: ObjectExt {
    fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self::from_raw(foxtk_sys::fx_table_new(
                parent.as_raw(),
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ))
        }
    }
    fn set_table_size(&self, rows: i32, cols: i32) {
        unsafe {
            foxtk_sys::fx_table_set_table_size(self.as_raw(), rows, cols);
        }
    }

    fn set_item_text(&self, row: i32, col: i32, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            foxtk_sys::fx_table_set_item_text(self.as_raw(), row, col, c_text.as_ptr());
        }
    }

    fn get_item_text(&self, row: i32, col: i32) -> String {
        unsafe {
            let ptr = foxtk_sys::fx_table_get_item_text(self.as_raw(), row, col);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

pub trait ScrollBarExt: ObjectExt {
    fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self::from_raw(foxtk_sys::fx_scroll_bar_new(
                parent.as_raw(),
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ))
        }
    }
    fn get_position(&self) -> i32 {
        unsafe { foxtk_sys::fx_scroll_bar_get_position(self.as_raw()) }
    }

    fn set_position(&self, pos: i32) {
        unsafe {
            foxtk_sys::fx_scroll_bar_set_position(self.as_raw(), pos);
        }
    }

    fn set_range(&self, lo: i32, hi: i32) {
        unsafe {
            foxtk_sys::fx_scroll_bar_set_range(self.as_raw(), lo, hi);
        }
    }
}

pub trait ListBoxExt: ObjectExt {
    fn new(parent: &impl WindowExt) -> Self {
        unsafe {
            Self::from_raw(foxtk_sys::fx_list_box_new(
                parent.as_raw(),
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ))
        }
    }
    fn append_item(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            foxtk_sys::fx_list_box_append_item(
                self.as_raw(),
                c_text.as_ptr(),
                std::ptr::null_mut(),
            );
        }
    }

    fn clear_items(&self) {
        unsafe {
            foxtk_sys::fx_list_box_clear_items(self.as_raw());
        }
    }

    fn get_current_item(&self) -> i32 {
        unsafe { foxtk_sys::fx_list_box_get_current_item(self.as_raw()) }
    }

    fn set_current_item(&self, index: i32) {
        unsafe {
            foxtk_sys::fx_list_box_set_current_item(self.as_raw(), index);
        }
    }

    fn get_item_text(&self, index: i32) -> String {
        unsafe {
            let ptr = foxtk_sys::fx_list_box_get_item_text(self.as_raw(), index);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }

    fn get_num_items(&self) -> i32 {
        unsafe { foxtk_sys::fx_list_box_get_num_items(self.as_raw()) }
    }
}

pub trait ComboBoxExt: ObjectExt {
    fn new(parent: &impl WindowExt, cols: i32) -> Self {
        unsafe {
            Self::from_raw(foxtk_sys::fx_combo_box_new(
                parent.as_raw(),
                cols,
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ))
        }
    }
    fn append_item(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            foxtk_sys::fx_combo_box_append_item(
                self.as_raw(),
                c_text.as_ptr(),
                std::ptr::null_mut(),
            );
        }
    }

    fn clear_items(&self) {
        unsafe {
            foxtk_sys::fx_combo_box_clear_items(self.as_raw());
        }
    }

    fn get_current_item(&self) -> i32 {
        unsafe { foxtk_sys::fx_combo_box_get_current_item(self.as_raw()) }
    }

    fn set_current_item(&self, index: i32) {
        unsafe {
            foxtk_sys::fx_combo_box_set_current_item(self.as_raw(), index);
        }
    }

    fn get_item_text(&self, index: i32) -> String {
        unsafe {
            let ptr = foxtk_sys::fx_combo_box_get_item_text(self.as_raw(), index);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }

    fn get_num_items(&self) -> i32 {
        unsafe { foxtk_sys::fx_combo_box_get_num_items(self.as_raw()) }
    }
}

pub trait AppExt: ObjectExt {
    fn new(name_: &str, vendor_: &str) -> Self {
        let name = std::ffi::CString::new(name_).unwrap();
        let vendor = std::ffi::CString::new(vendor_).unwrap();
        let args = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();
        Self::from_raw(unsafe {
            foxtk_sys::fx_app_new(
                name.as_ptr(),
                vendor.as_ptr(),
                args.len() as c_int,
                args.as_ptr() as *mut *mut c_char,
            )
        })
    }
    fn add_timeout<F: FnMut(Self) -> bool + 'static>(&self, ms: u32, func: F) {
        let raw_ptr: *mut Box<dyn FnMut(Self) -> bool> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            foxtk_sys::fx_app_add_timeout(
                self.as_raw(),
                Some(ctimer::<Self>),
                ms,
                raw_ptr as *mut std::os::raw::c_void,
            );
        }
    }
    fn run(&self) -> i32 {
        unsafe { foxtk_sys::fx_app_run(self.as_raw()) }
    }
}

pub trait IdExt: ObjectExt {
    fn get_app(&self) -> impl AppExt {
        app::App::from_raw(unsafe { foxtk_sys::fx_id_get_app(self.as_raw()) })
    }
}
pub trait WindowExt: IdExt {
    fn set_callback<F: FnMut(Self) -> bool + 'static>(&self, func: F) {
        let raw_ptr: *mut Box<dyn FnMut(Self) -> bool> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            foxtk_sys::fx_window_set_target(
                self.as_raw(),
                Some(ccallback::<Self>),
                raw_ptr as *mut std::os::raw::c_void,
            );
        }
    }
}
pub trait TextFieldExt: WindowExt {
    fn new(parent: &impl ObjectExt, ncols: i32) -> Self {
        Self::from_raw(unsafe { foxtk_sys::fx_textfield_new(parent.as_raw(), ncols) })
    }
    fn text(&self) -> String {
        unsafe {
            let ptr = foxtk_sys::fx_textfield_get_text(self.as_raw());
            if !ptr.is_null() {
                std::ffi::CStr::from_ptr(ptr).to_string_lossy().to_string()
            } else {
                String::new()
            }
        }
    }
    fn set_text(&self, text_: &str) {
        let text = std::ffi::CString::new(text_).unwrap();
        unsafe { foxtk_sys::fx_textfield_set_text(self.as_raw(), text.as_ptr()) };
    }
}
pub trait SpinnerExt: WindowExt {
    fn new(parent: &impl ObjectExt, cols: i32) -> Self {
        Self::from_raw(unsafe {
            foxtk_sys::fx_spinner_new(
                parent.as_raw(),
                cols,
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            )
        })
    }
    fn get_value(&self) -> i32 {
        unsafe { foxtk_sys::fx_spinner_get_value(self.as_raw()) }
    }
    fn set_value(&self, value: i32) {
        unsafe { foxtk_sys::fx_spinner_set_value(self.as_raw(), value) }
    }
    fn get_range(&self) -> (i32, i32) {
        let mut lo = 0;
        let mut hi = 0;
        unsafe { foxtk_sys::fx_spinner_get_range(self.as_raw(), &mut lo, &mut hi) };
        (lo, hi)
    }
    fn set_range(&self, lo: i32, hi: i32) {
        unsafe { foxtk_sys::fx_spinner_set_range(self.as_raw(), lo, hi) }
    }
    fn set_increment(&self, inc: i32) {
        unsafe { foxtk_sys::fx_spinner_set_increment(self.as_raw(), inc) }
    }
    fn increment(&self) {
        unsafe { foxtk_sys::fx_spinner_increment(self.as_raw()) }
    }
    fn decrement(&self) {
        unsafe { foxtk_sys::fx_spinner_decrement(self.as_raw()) }
    }
}
pub trait RangeSliderExt: WindowExt {
    fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe {
            foxtk_sys::fx_slider_new(
                parent.as_raw(),
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            )
        })
    }
    fn get_value(&self) -> i32 {
        unsafe { foxtk_sys::fx_slider_get_value(self.as_raw()) }
    }
    fn set_value(&self, value: i32) {
        unsafe { foxtk_sys::fx_slider_set_value(self.as_raw(), value) }
    }
    fn get_range(&self) -> (i32, i32) {
        let mut lo = 0;
        let mut hi = 0;
        unsafe { foxtk_sys::fx_slider_get_range(self.as_raw(), &mut lo, &mut hi) };
        (lo, hi)
    }
    fn set_range(&self, lo: i32, hi: i32) {
        unsafe { foxtk_sys::fx_slider_set_range(self.as_raw(), lo, hi) }
    }
    fn get_increment(&self) -> i32 {
        unsafe { foxtk_sys::fx_slider_get_increment(self.as_raw()) }
    }
    fn set_increment(&self, inc: i32) {
        unsafe { foxtk_sys::fx_slider_set_increment(self.as_raw(), inc) }
    }
}
pub trait ProgressBarExt: WindowExt {
    fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe {
            foxtk_sys::fx_progressbar_new(
                parent.as_raw(),
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            )
        })
    }
    fn set_progress(&self, value: u32) {
        unsafe { foxtk_sys::fx_progressbar_set_progress(self.as_raw(), value) }
    }
    fn progress(&self) -> u32 {
        unsafe { foxtk_sys::fx_progressbar_get_progress(self.as_raw()) }
    }
    fn set_total(&self, value: u32) {
        unsafe { foxtk_sys::fx_progressbar_set_total(self.as_raw(), value) }
    }
    fn total(&self) -> u32 {
        unsafe { foxtk_sys::fx_progressbar_get_total(self.as_raw()) }
    }
    fn increment(&self, value: u32) {
        unsafe { foxtk_sys::fx_progressbar_increment(self.as_raw(), value) }
    }
    fn show_number(&self) {
        unsafe { foxtk_sys::fx_progressbar_show_number(self.as_raw()) }
    }
    fn hide_number(&self) {
        unsafe { foxtk_sys::fx_progressbar_hide_number(self.as_raw()) }
    }
    fn set_bar_size(&self, size: i32) {
        unsafe { foxtk_sys::fx_progressbar_set_bar_size(self.as_raw(), size) }
    }
    fn bar_size(&self) -> i32 {
        unsafe { foxtk_sys::fx_progressbar_get_bar_size(self.as_raw()) }
    }
}
pub trait LabelExt: WindowExt {
    fn text(&self) -> String {
        unsafe {
            let ptr = foxtk_sys::fx_label_get_text(self.as_raw());
            if !ptr.is_null() {
                std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }
    fn set_text(&self, text_: &str) {
        let text = std::ffi::CString::new(text_).unwrap();
        unsafe { foxtk_sys::fx_label_set_text(self.as_raw(), text.as_ptr()) };
    }
}
pub trait ButtonExt: LabelExt {
    fn new(parent: &impl ObjectExt, title_: &str) -> Self {
        let title = std::ffi::CString::new(format!("&{title_}").as_str()).unwrap();
        ObjectExt::from_raw(unsafe { foxtk_sys::fx_button_new(parent.as_raw(), title.as_ptr()) })
    }
}
pub trait RadioButtonExt: LabelExt {
    fn new(parent: &impl ObjectExt, title_: &str) -> Self {
        let title = std::ffi::CString::new(title_).unwrap();
        Self::from_raw(unsafe { foxtk_sys::fx_radio_button_new(parent.as_raw(), title.as_ptr()) })
    }
    fn check(&self) -> bool {
        unsafe { foxtk_sys::fx_radio_button_get_check(self.as_raw()) != 0 }
    }
    fn set_check(&self) {
        unsafe { foxtk_sys::fx_radio_button_set_check(self.as_raw()) }
    }
}
pub trait CheckButtonExt: LabelExt {
    fn new(parent: &impl ObjectExt, title_: &str) -> Self {
        let title = std::ffi::CString::new(title_).unwrap();
        Self::from_raw(unsafe { foxtk_sys::fx_check_button_new(parent.as_raw(), title.as_ptr()) })
    }
    fn check(&self) -> bool {
        unsafe { foxtk_sys::fx_check_button_get_check(self.as_raw()) != 0 }
    }
    fn set_check(&self, check: bool) {
        unsafe { foxtk_sys::fx_check_button_set_check(self.as_raw(), check as u8) }
    }
}
pub trait VerticalFrameExt: WindowExt {
    fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { foxtk_sys::fx_vertical_frame_new(ObjectExt::as_raw(parent)) })
    }
}
pub trait HorizontalFrameExt: WindowExt {
    fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { foxtk_sys::fx_horizontal_frame_new(ObjectExt::as_raw(parent)) })
    }
}
pub trait MainWindowExt: WindowExt {
    fn new(app: &impl AppExt, title_: &str, w: i32, h: i32) -> Self {
        let title = std::ffi::CString::new(title_).unwrap();
        let wgt = Self::from_raw(unsafe {
            foxtk_sys::fx_main_window_new(app.as_raw(), title.as_ptr(), w, h)
        });
        wgt.show();
        wgt
    }
    fn show(&self) {
        unsafe { foxtk_sys::fx_main_window_show(self.as_raw()) }
    }
}
pub trait Component: Default + 'static {
    type Event: 'static;
    type State: Default + 'static;
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool;
    fn update(&self, model: &Self::State);
    fn view(&mut self, parent: &impl WindowExt, sender: Sender<Self::Event>);
    fn mount(parent: &impl WindowExt) {
        let (sender, receiver) = channel::<Self::Event>();
        let mut page = Self::default();
        let mut model = Self::State::default();
        page.view(parent, sender.clone());
        page.update(&model);
        const TICK: u32 = 200;
        parent.get_app().add_timeout(TICK, move |_| {
            if let Ok(msg) = receiver.try_recv()
                && Self::handle(msg, &mut model, sender.clone())
            {
                page.update(&model);
            }
            true
        });
    }
    fn run(name: &str, vendor: &str, title: &str) {
        let app = App::new(name, vendor);
        Self::mount(&MainWindow::new(&app, title, 480, 270));
        app.run();
    }
}
