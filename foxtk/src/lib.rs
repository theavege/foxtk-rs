pub mod app;
pub mod button;
pub mod frame;
pub mod prelude;
pub mod radiobutton;
pub mod textfield;
pub mod window;
use std::{
    ffi::CString,
    os::raw::{c_char, c_int},
    sync::mpsc::channel,
};
pub use {
    app::App, button::Button, frame::HorizontalFrame, frame::VerticalFrame,
    radiobutton::RadioButton, std::sync::mpsc::Sender, textfield::TextField, window::MainWindow,
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
    fn set_check(&self, check: bool) {
        unsafe { foxtk_sys::fx_radio_button_set_check(self.as_raw(), if check { 1 } else { 0 }) }
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
