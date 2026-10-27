pub use super::enums::*;
use {
    super::{cstr_lossy, to_cstring},
    foxtk_sys::*,
    std::{
        cell::RefCell,
        ffi::{c_long, c_void},
        marker::PhantomData,
        ptr::NonNull,
        rc::Rc,
        sync::{
            Arc, Mutex, Weak,
            mpsc::{self, Sender as ChannelSender},
        },
        thread::{self, ThreadId},
    },
};

pub struct ComponentSender<E> {
    channel: ChannelSender<E>,
    scheduler: Weak<Mutex<ComponentScheduler>>,
}

impl<E> Clone for ComponentSender<E> {
    fn clone(&self) -> Self {
        Self {
            channel: self.channel.clone(),
            scheduler: self.scheduler.clone(),
        }
    }
}

impl<E> ComponentSender<E> {
    pub fn send(&self, message: E) -> Result<(), mpsc::SendError<E>> {
        let Some(scheduler) = self.scheduler.upgrade() else {
            return Err(mpsc::SendError(message));
        };
        let mut scheduler = scheduler.lock().unwrap();
        if !scheduler.active {
            return Err(mpsc::SendError(message));
        }
        self.channel.send(message)?;
        if thread::current().id() == scheduler.ui_thread && !scheduler.scheduled {
            scheduler.scheduled = true;
            unsafe {
                FXApp_schedule_chore(scheduler.app as *mut FXApp, scheduler.chore as *mut FXChore);
            }
        }
        Ok(())
    }
}

pub type Sender<E> = ComponentSender<E>;

struct ComponentScheduler {
    app: usize,
    chore: usize,
    ui_thread: ThreadId,
    active: bool,
    scheduled: bool,
}

#[must_use = "keep the mount handle alive while the component is mounted"]
pub struct ComponentMount {
    app: NonNull<FXApp>,
    chore: NonNull<FXChore>,
    timeout: NonNull<FXTimeout>,
    scheduler: Arc<Mutex<ComponentScheduler>>,
    _not_send: PhantomData<Rc<()>>,
}

impl Drop for ComponentMount {
    fn drop(&mut self) {
        let mut scheduler = self.scheduler.lock().unwrap();
        scheduler.active = false;
        unsafe {
            FXApp_remove_chore(self.app.as_ptr(), self.chore.as_ptr());
            FXApp_remove_timeout(self.app.as_ptr(), self.timeout.as_ptr());
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentOutcome {
    Unchanged,
    Changed,
    Stop,
}

unsafe extern "C" fn ccallback<T: WindowExt>(ptr: *mut FXWindow, context: *mut c_void) -> c_long {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let func = &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr as *mut T::T)) as c_long
    }));
    result.unwrap_or(0)
}

pub trait ObjectExt: Sized {
    type T: 'static;
    fn as_raw(&self) -> *mut Self::T;
    /// Construct a wrapper around a native pointer.
    ///
    /// # Safety
    /// `ptr` must be a live pointer to the native type represented by `Self`
    /// and must remain valid for every use of the returned wrapper.
    unsafe fn from_raw(ptr: *mut Self::T) -> Self;
}

pub trait IdExt: ObjectExt {
    fn app(&self) -> super::App {
        unsafe { super::App::from_raw(FXId_get_app(self.as_raw() as *const FXId)) }
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
            cstr_lossy(ptr)
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
            cstr_lossy(ptr)
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
                Some(super::drop_callback_context::<Box<dyn FnMut(Self) -> bool>>),
            );
        }
        self
    }
    fn parent(&self) -> Self {
        unsafe {
            Self::from_raw(FXWindow_get_parent(self.as_raw() as *const FXWindow) as *mut Self::T)
        }
    }
    fn root(&self) -> Self {
        unsafe {
            Self::from_raw(FXWindow_get_root(self.as_raw() as *const FXWindow) as *mut Self::T)
        }
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

//~ pub trait DCWindowExt: ObjectExt {
//~ fn new_dc(&self) -> Self {
//~ unsafe { Self::from_raw(FXDCWindow_new(self.as_raw() as *mut FXDrawable)) }
//~ }
//~ fn dc_set_foreground(&self, color: Color) {
//~ unsafe { FXDC_set_foreground(self.as_raw() as *mut FXDC, color.bits()) }
//~ }
//~ fn dc_set_line_width(&self, width: i32) {
//~ unsafe { FXDC_set_line_width(self.as_raw() as *mut FXDC, width) }
//~ }
//~ fn dc_draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
//~ unsafe { FXDC_draw_line(self.as_raw() as *mut FXDC, x1, y1, x2, y2) }
//~ }
//~ fn dc_draw_point(&self, x: i32, y: i32) {
//~ unsafe { FXDC_draw_point(self.as_raw() as *mut FXDC, x, y) }
//~ }
//~ fn dc_draw_rect(&self, x: i32, y: i32, w: i32, h: i32) {
//~ unsafe { FXDC_draw_rect(self.as_raw() as *mut FXDC, x, y, w, h) }
//~ }
//~ fn dc_fill_rect(&self, x: i32, y: i32, w: i32, h: i32) {
//~ unsafe { FXDC_fill_rect(self.as_raw() as *mut FXDC, x, y, w, h) }
//~ }
//~ }

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

/// FOX 1.6 `FXApp::addTimeout` takes milliseconds. This polls for a `send`
/// from a thread other than the UI thread, which must not call `addChore`.
const COMPONENT_POLL_MS: u32 = 50;

pub trait Component: Default + 'static {
    type Event: 'static;
    type State: Default + 'static;
    fn handle(
        msg: Self::Event,
        model: &mut Self::State,
        sender: Sender<Self::Event>,
    ) -> ComponentOutcome;
    fn update(&self, model: &Self::State);
    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>);
    fn mount(parent: &impl CompositeExt) -> ComponentMount {
        let (channel, receiver) = mpsc::channel::<Self::Event>();
        let dispatch_slot: Rc<RefCell<Option<Box<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let app = parent.app();
        let app_ptr = NonNull::new(app.as_raw()).expect("FOX application pointer was null");
        let scheduler = Arc::new(Mutex::new(ComponentScheduler {
            app: app_ptr.as_ptr() as usize,
            chore: 0,
            ui_thread: thread::current().id(),
            active: true,
            scheduled: false,
        }));
        let chore_context =
            component_callback_context(dispatch_slot.clone(), Some(scheduler.clone()), true);
        let chore = NonNull::new(unsafe {
            FXApp_add_chore(
                app_ptr.as_ptr(),
                Some(super::ctimer_callback::<super::App>),
                chore_context,
                Some(super::drop_callback_context::<Box<dyn FnMut(super::App) -> bool>>),
            )
        })
        .unwrap_or_else(|| {
            unsafe {
                super::drop_callback_context::<Box<dyn FnMut(super::App) -> bool>>(chore_context)
            };
            panic!("FOX failed to allocate a component chore");
        });
        scheduler.lock().unwrap().chore = chore.as_ptr() as usize;
        let timeout_context =
            component_callback_context(dispatch_slot.clone(), Some(scheduler.clone()), false);
        let timeout = NonNull::new(unsafe {
            FXApp_add_timeout(
                app_ptr.as_ptr(),
                Some(super::ctimer_callback::<super::App>),
                COMPONENT_POLL_MS,
                timeout_context,
                Some(super::drop_callback_context::<Box<dyn FnMut(super::App) -> bool>>),
            )
        });
        let Some(timeout) = timeout else {
            unsafe {
                super::drop_callback_context::<Box<dyn FnMut(super::App) -> bool>>(timeout_context);
                FXApp_remove_chore(app_ptr.as_ptr(), chore.as_ptr());
            }
            panic!("FOX failed to allocate a component event pump");
        };
        let mount = ComponentMount {
            app: app_ptr,
            chore,
            timeout,
            scheduler: scheduler.clone(),
            _not_send: PhantomData,
        };
        let sender = ComponentSender {
            channel,
            scheduler: Arc::downgrade(&scheduler),
        };
        let mut page = Self::default();
        let mut model = Self::State::default();
        page.view(parent, sender.clone());
        page.update(&model);
        *dispatch_slot.borrow_mut() = Some(Box::new(move || {
            let mut changed = false;
            while let Ok(msg) = receiver.try_recv() {
                match Self::handle(msg, &mut model, sender.clone()) {
                    ComponentOutcome::Unchanged => {}
                    ComponentOutcome::Changed => changed = true,
                    ComponentOutcome::Stop => {
                        scheduler.lock().unwrap().active = false;
                        break;
                    }
                }
            }
            if changed {
                page.update(&model);
            }
        }));
        mount
    }
    fn run(name: &str, vendor: &str, title: &str, width: i32, height: i32) -> i32 {
        let app = super::App::new(name, vendor);
        let win = super::MainWindow::new(&app, title, width, height);
        let _mount = Self::mount(&win);
        win.show();
        app.run()
    }
}

fn component_callback_context(
    dispatch_slot: Rc<RefCell<Option<Box<dyn FnMut()>>>>,
    scheduler: Option<Arc<Mutex<ComponentScheduler>>>,
    clear_scheduled: bool,
) -> *mut c_void {
    let callback: Box<dyn FnMut(super::App) -> bool> = Box::new(move |_| {
        if let Some(scheduler) = &scheduler {
            let mut scheduler = scheduler.lock().unwrap();
            if !scheduler.active {
                return true;
            }
            if clear_scheduled {
                scheduler.scheduled = false;
            }
        }
        // A modal loop inside handle() re-enters this callback. The outer
        // dispatch is already draining the channel, so skip instead of panicking.
        if let Ok(mut slot) = dispatch_slot.try_borrow_mut()
            && let Some(dispatch) = slot.as_mut()
        {
            dispatch();
        }
        true
    });
    Box::into_raw(Box::new(callback)) as *mut c_void
}
