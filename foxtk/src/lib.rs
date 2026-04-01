pub mod application;
pub mod button;
pub mod frame;
pub mod textfield;
pub mod window;

pub use {
    application::Application,
    button::Button,
    frame::Frame,
    std::sync::mpsc::{Sender, channel},
    textfield::TextField,
    window::MainWindow,
};

pub trait Parent {
    fn as_raw(&self) -> foxtk_sys::FXParentPtr;
}

unsafe extern "C" fn ccallback<T: crate::Widget>(
    ptr: foxtk_sys::FXWidgetPtr,
    context: *mut std::os::raw::c_void,
) -> std::os::raw::c_long {
    unsafe {
        let func: &mut Box<dyn FnMut(T) -> bool> =
            &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr)) as std::os::raw::c_long
    }
}

pub trait Widget: Sized {
    fn as_raw(&self) -> foxtk_sys::FXWidgetPtr;
    fn from_raw(ptr: foxtk_sys::FXWidgetPtr) -> Self;
    fn set_callback<F: FnMut(Self) -> bool + 'static>(&self, func: F) {
        let raw_ptr: *mut Box<dyn FnMut(Self) -> bool> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            foxtk_sys::fox_button_set_target(
                self.as_raw(),
                Some(ccallback::<Self>),
                raw_ptr as *mut std::os::raw::c_void,
            );
        }
    }
}

pub trait Component: Default + 'static {
    type Event: 'static;
    type State: Default + 'static;
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool;
    fn update(&self, model: &Self::State);
    fn view(&mut self, parent: &MainWindow, sender: Sender<Self::Event>);
    fn run() {
        let (sender, receiver) = channel::<Self::Event>();
        let mut page = Self::default();
        let mut model = Self::State::default();
        let app = Application::new("Name", "Vendor");
        let parent = MainWindow::new(&app, "Title", 480, 270);
        page.view(&parent, sender.clone());
        page.update(&model);
        const TICK: u32 = 200;
        app.add_timeout(TICK, move |_| {
            if let Ok(msg) = receiver.try_recv()
                && Self::handle(msg, &mut model, sender.clone())
            {
                page.update(&model);
            }
            true
        });
        app.run();
    }
}
