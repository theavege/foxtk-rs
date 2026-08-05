#![doc = include_str!("../README.md")]

pub mod enums;
pub mod prelude;
use {
    foxtk_sys::*,
    prelude::*,
    std::{
        ffi::{CString, c_long, c_void},
        ptr::NonNull,
    },
};

pub(crate) const HEIGHT: i32 = 30;

unsafe extern "C" fn ctimer_callback<T: ObjectExt>(ptr: *mut T::T, context: *mut c_void) -> c_long {
    unsafe {
        let func: &mut Box<dyn FnMut(T) -> bool> =
            &mut *(context as *mut Box<dyn FnMut(T) -> bool>);
        func(T::from_raw(ptr)) as c_long
    }
}

unsafe extern "C" fn cmouse_callback<T: ObjectExt>(
    ptr: *mut T::T,
    selector: i32,
    x: i32,
    y: i32,
    context: *mut c_void,
) -> c_long {
    unsafe {
        let func: &mut Box<dyn FnMut(T, i32, i32, i32) -> bool> =
            &mut *(context as *mut Box<dyn FnMut(T, i32, i32, i32) -> bool>);
        func(T::from_raw(ptr), selector, x, y) as c_long
    }
}

macro_rules! impl_widget {
    ($name:ident, $($dep:ident),*) => {
        paste::paste! {
            #[derive(Default, Clone, Copy)]
            pub struct $name(Option<std::ptr::NonNull<[<FX $name >]>>);

            impl ObjectExt for $name {
                type T = [<FX $name >];
                fn as_raw(&self) -> *mut Self::T {
                    self.0
                        .expect(concat!("Empty ", stringify!($name), "!"))
                        .as_ptr()
                }
                fn from_raw(obj: *mut Self::T) -> Self {
                    Self(NonNull::new(obj))
                }
            }

            $(impl $dep for $name {})*
        }
    };
}

macro_rules! impl_ranger {
    ($($name:ident),+) => {
        paste::paste! {$(
            impl RangerExt for $name {
                fn value(&self) -> i32 {
                    unsafe { [<FX $name _get_value>](self.as_raw()) }
                }
                fn set_value(&self, value: i32) {
                    unsafe { [<FX $name _set_value>](self.as_raw(), value) }
                }
                fn range(&self) -> (i32, i32) {
                    let mut lo = 0;
                    let mut hi = 0;
                    unsafe { [<FX $name _get_range>](self.as_raw(), &mut lo, &mut hi) };
                    (lo, hi)
                }
                fn set_range(&self, low: i32, high: i32) {
                    unsafe { [<FX $name _set_range>](self.as_raw(), low, high) }
                }
            }
        )+}
    };
}

macro_rules! impl_selector {
    ($($name:ident),+) => {
        paste::paste! {$(
            impl SelectorExt for $name {
                fn append_item(&self, text: &str) {
                    unsafe {
                        [<FX $name _append_item>](self.as_raw(), CString::new(text).unwrap().as_ptr());
                    }
                }
                fn clear_items(&self) {
                    unsafe {
                        [<FX $name _clear_items>](self.as_raw());
                    }
                }

                fn current_item(&self) -> i32 {
                    unsafe { [<FX $name _get_current_item>](self.as_raw()) }
                }

                fn set_current_item(&self, index: i32) {
                    unsafe {
                        [<FX $name _set_current_item>](self.as_raw(), index);
                    }
                }

                fn item_text(&self, index: i32) -> String {
                    unsafe {
                        let ptr = [<FX $name _get_item_text>](self.as_raw(), index);
                        std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
                    }
                }
                fn num_items(&self) -> i32 {
                    unsafe { [<FX $name _get_num_items>](self.as_raw()) }
                }
                fn set_num_visible(&self, num_visible: i32) {
                    unsafe {
                        [<FX $name _set_num_visible>](self.as_raw(), num_visible);
                    }
                }
            }
        )+}
    };
}

macro_rules! impl_textable {
    ($($name:ident),+) => {
        paste::paste! {$(
            impl TextableExt for $name {
                fn text(&self) -> String {
                    unsafe {
                        let ptr = [<FX $name _get_text>](self.as_raw());
                        std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
                    }
                }
                fn set_text(&self, text: &str) {
                    unsafe {
                        [<FX $name _set_text>](self.as_raw(), CString::new(text).unwrap().as_ptr());
                    }
                }
                fn set_text_color(&self, color: Color) {
                    unsafe {
                        [<FX $name _set_text_color>](self.as_raw(), color.bits());
                    }
                }
                fn set_font(&self, family: &str, size: i32) {
                    unsafe {
                        [<FX $name _set_font>](self.as_raw(), CString::new(family).unwrap().as_ptr(), size);
                    }
                }
            }
        )+}
    };
}

macro_rules! impl_editable {
    ($($name:ident),+) => {
        paste::paste! {$(
            impl EditableExt for $name {
                fn set_editable(&self, val: bool) {
                    unsafe {
                        [<FX $name _set_editable>](self.as_raw(), val as u8);
                    }
                }
            }
        )+}
    };
}

impl_widget!(App,);
/// The application object manages the message queue, timers, chores, signal handling, GUI updating, and other system facilities. Each FOX application will have exactly one application instance. Every FOX application will start by constructing one FXApp object prior to building the GUI.  Usually, the FXApp object is the last object to be deleted as well.
///
/// Using the code below, the application object will be constructed on the stack and hence is automatically destroyed when the program terminates.  Also, when the application object is destroyed, all the windows and other resources it knows about are destroyed as well.
impl App {
    pub fn new(name: &str, vendor: &str) -> Self {
        let args = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const i8>>();
        Self::from_raw(unsafe {
            FXApp_new(
                CString::new(name).unwrap().as_ptr(),
                CString::new(vendor).unwrap().as_ptr(),
                args.len() as i32,
                args.as_ptr() as *mut *mut i8,
            )
        })
    }
    pub fn add_timeout<F: FnMut(Self) -> bool + 'static>(&self, ms: u32, func: F) {
        let context: *mut Box<dyn FnMut(Self) -> bool> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            FXApp_add_timeout(
                self.as_raw(),
                Some(ctimer_callback::<Self>),
                ms,
                context as *mut c_void,
            );
        }
    }
    pub fn run(&self) -> i32 {
        unsafe { FXApp_run(self.as_raw()) }
    }
}

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
        Self::from_raw(unsafe { FXSplashWindow_new(app.as_raw()) })
    }
}

impl_widget!(Button, IdExt, DrawableExt, WindowExt, FrameExt);
impl Button {
    pub fn new(parent: &impl CompositeExt, title: &str) -> Self {
        Self::from_raw(unsafe {
            FXButton_new(
                parent.as_raw() as *mut FXComposite,
                CString::new(format!("&{title}").as_str()).unwrap().as_ptr(),
            )
        })
        .with_layout(Layout::FillX)
    }
    pub fn set_state(&self, state: ButtonState) {
        unsafe {
            FXButton_set_state(self.as_raw(), state as u32);
        }
    }
    pub fn with_style(self, style: ButtonStyle) -> Self {
        unsafe {
            FXButton_set_style(self.as_raw(), style as u32);
        }
        self
    }
}

impl_widget!(ArrowButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl ArrowButton {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXArrowButton_new(parent.as_raw() as *mut FXComposite) })
    }
    pub fn set_size(&self, size: i32) {
        unsafe {
            FXArrowButton_set_arrow_size(self.as_raw(), size);
        }
    }
    pub fn set_color(&self, color: Color) {
        unsafe {
            FXArrowButton_set_arrow_color(self.as_raw(), color.bits());
        }
    }
}

impl_widget!(Canvas, IdExt, FrameExt, DrawableExt, WindowExt);
impl Canvas {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXCanvas_new(parent.as_raw() as *mut FXComposite) })
    }
    pub fn set_mouse_callback<F: FnMut(Self, i32, i32, i32) -> bool + 'static>(&self, func: F) {
        let context: *mut Box<dyn FnMut(Self, i32, i32, i32) -> bool> =
            Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            FXCanvas_set_mouse_callback(
                self.as_raw(),
                Some(cmouse_callback::<Self>),
                context as *mut c_void,
            );
        }
    }
}

impl_widget!(CheckButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl CheckButton {
    pub fn new(parent: &impl CompositeExt, title: &str) -> Self {
        Self::from_raw(unsafe {
            FXCheckButton_new(
                parent.as_raw() as *mut FXComposite,
                CString::new(title).unwrap().as_ptr(),
            )
        })
        .with_layout(Layout::FillX)
    }
    pub fn check(&self) -> bool {
        unsafe { FXCheckButton_get_check(self.as_raw()) != 0 }
    }
    pub fn set_check(&self, check: bool) {
        unsafe { FXCheckButton_set_check(self.as_raw(), check as u8) }
    }
    pub fn with_check(self, check: bool) -> Self {
        self.set_check(check);
        self
    }
}

impl_widget!(
    ComboBox,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    PackerExt,
    CompositeExt
);

impl ComboBox {
    pub fn new(parent: &impl WindowExt, cols: i32) -> Self {
        Self::from_raw(unsafe { FXComboBox_new(parent.as_raw() as *mut FXComposite, cols) })
            .with_layout(Layout::FillX)
    }
}

impl_widget!(
    GroupBox,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    PackerExt,
    CompositeExt
);

impl GroupBox {
    pub fn new(parent: &impl CompositeExt, title_: &str) -> Self {
        Self::from_raw(unsafe {
            FXGroupBox_new(
                parent.as_raw() as *mut FXComposite,
                CString::new(title_).unwrap().as_ptr(),
            )
        })
        .with_frame(FrameStyle::Thick)
        .with_layout(Layout::FillX)
    }
    pub fn with_style(self, style: GroupBoxStyle) -> Self {
        unsafe {
            FXGroupBox_set_style(self.as_raw(), style as u32);
        }
        self
    }
}

impl_widget!(
    VerticalFrame,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl VerticalFrame {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXVerticalFrame_new(parent.as_raw() as *mut FXComposite) })
            .with_layout(Layout::FillX)
            .with_frame(FrameStyle::Thick)
    }
}

impl_widget!(
    HorizontalFrame,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl HorizontalFrame {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXHorizontalFrame_new(parent.as_raw() as *mut FXComposite) })
            .with_height(HEIGHT)
            .with_frame(FrameStyle::Thick)
    }
}

impl_widget!(
    Matrix,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl Matrix {
    pub fn new(parent: &impl CompositeExt, rows: i32) -> Self {
        Self::from_raw(unsafe {
            FXMatrix_new(
                parent.as_raw() as *mut FXComposite,
                rows,
                MatrixStyle::ByRows as u32,
            )
        })
        .with_layout(Layout::Fill)
    }
    pub fn set_style(&self, style: MatrixStyle) {
        unsafe {
            FXMatrix_set_style(self.as_raw(), style as u32);
        }
    }
    pub fn set_num_rows(&self, rows: i32) {
        unsafe {
            FXMatrix_set_num_rows(self.as_raw(), rows);
        }
    }
    pub fn set_num_columns(&self, cols: i32) {
        unsafe {
            FXMatrix_set_num_columns(self.as_raw(), cols);
        }
    }
    pub fn style(&self) -> MatrixStyle {
        unsafe { std::mem::transmute::<u32, MatrixStyle>(FXMatrix_get_style(self.as_raw()) as u32) }
    }
    pub fn num_rows(&self) -> i32 {
        unsafe { FXMatrix_get_num_rows(self.as_raw()) }
    }
    pub fn num_columns(&self) -> i32 {
        unsafe { FXMatrix_get_num_columns(self.as_raw()) }
    }
}

impl_widget!(
    Splitter,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl Splitter {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe {
            FXSplitter_new(
                parent.as_raw() as *mut FXComposite,
                SplitterStyle::Normal as u32,
            )
        })
        .with_layout(Layout::Fill)
    }
    pub fn with_style(self, style: SplitterStyle) -> Self {
        self.set_style(style);
        self
    }
    pub fn set_style(&self, style: SplitterStyle) {
        unsafe {
            FXSplitter_set_style(self.as_raw(), style as u32);
        }
    }
    pub fn style(&self) -> SplitterStyle {
        unsafe {
            std::mem::transmute::<u32, SplitterStyle>(FXSplitter_get_style(self.as_raw()) as u32)
        }
    }
    pub fn set_split(&self, index: i32, size: i32) {
        unsafe {
            FXSplitter_set_split(self.as_raw(), index, size);
        }
    }
    pub fn split(&self, index: i32) -> i32 {
        unsafe { FXSplitter_get_split(self.as_raw(), index) }
    }
    pub fn set_bar_size(&self, size: i32) {
        unsafe {
            FXSplitter_set_bar_size(self.as_raw(), size);
        }
    }
    pub fn bar_size(&self) -> i32 {
        unsafe { FXSplitter_get_bar_size(self.as_raw()) }
    }
}

impl_widget!(
    StatusBar,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);

impl StatusBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXStatusBar_new(parent.as_raw() as *mut FXComposite) })
            .with_layout(Layout::FillX)
    }
    pub fn set_text(&self, text: &str) {
        unsafe {
            FXStatusBar_set_text(self.as_raw(), CString::new(text).unwrap().as_ptr());
        }
    }
    pub fn text(&self) -> String {
        unsafe {
            let ptr = FXStatusBar_get_text(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    pub fn set_help_text(&self, text: &str) {
        unsafe {
            FXStatusBar_set_help_text(self.as_raw(), CString::new(text).unwrap().as_ptr());
        }
    }
    pub fn help_text(&self) -> String {
        unsafe {
            let ptr = FXStatusBar_get_help_text(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
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
        Self::from_raw(unsafe {
            FXDialogBox_new(
                parent.as_raw() as *mut FXWindow,
                CString::new(title).unwrap().as_ptr(),
            )
        })
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
    pub fn with_style(self, style: DialogBoxStyle) -> Self {
        unsafe {
            FXTopWindow_set_decorations(self.as_raw() as *mut FXTopWindow, style as u32);
        }
        self
    }
}

impl_widget!(
    FileDialog,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);

impl FileDialog {
    pub fn new(parent: &impl WindowExt, title: &str) -> Self {
        Self::from_raw(unsafe {
            FXFileDialog_new(
                parent.as_raw() as *mut FXWindow,
                CString::new(title).unwrap().as_ptr(),
            )
        })
    }
    pub fn show(&self) {
        unsafe {
            FXDialogBox_show(self.as_raw() as *mut FXDialogBox);
        }
    }
    pub fn set_directory(&self, directory: &str) {
        unsafe {
            FXFileDialog_set_directory(self.as_raw(), CString::new(directory).unwrap().as_ptr());
        }
    }
    pub fn directory(&self) -> String {
        unsafe {
            let ptr = FXFileDialog_get_directory(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    pub fn set_filename(&self, filename: &str) {
        unsafe {
            FXFileDialog_set_filename(self.as_raw(), CString::new(filename).unwrap().as_ptr());
        }
    }
    pub fn filename(&self) -> String {
        unsafe {
            let ptr = FXFileDialog_get_filename(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    pub fn set_pattern(&self, pattern: &str) {
        unsafe {
            FXFileDialog_set_pattern(self.as_raw(), CString::new(pattern).unwrap().as_ptr());
        }
    }
    pub fn pattern(&self) -> String {
        unsafe {
            let ptr = FXFileDialog_get_pattern(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

impl_widget!(
    Switcher,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl Switcher {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXSwitcher_new(parent.as_raw() as *mut FXComposite) })
            .with_layout(Layout::Fill)
            .with_frame(FrameStyle::Thick)
    }
    pub fn set_curent(&self, idx: i32) {
        unsafe { FXSwitcher_set_current(self.as_raw(), idx) }
    }
}

impl_widget!(Label, IdExt, FrameExt, DrawableExt, WindowExt);
impl Label {
    pub fn new(parent: &impl CompositeExt, title: &str) -> Self {
        Self::from_raw(unsafe {
            FXLabel_new(
                parent.as_raw() as *mut FXComposite,
                CString::new(format!("&{title}").as_str()).unwrap().as_ptr(),
            )
        })
        .with_height(HEIGHT)
    }
    pub fn with_justify(self, justify: Justify) -> Self {
        unsafe {
            FXLabel_set_justify(self.as_raw(), justify as u32);
        }
        self
    }
}

impl_widget!(Knob, IdExt, FrameExt, DrawableExt, WindowExt);
impl Knob {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXKnob_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(Dial, IdExt, FrameExt, DrawableExt, WindowExt);
impl Dial {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXDial_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(
    ListBox,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    PackerExt,
    CompositeExt
);

impl ListBox {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXListBox_new(parent.as_raw() as *mut FXComposite) })
            .with_layout(Layout::FillX)
            .with_num_visible(3)
    }
}

impl_widget!(
    List,
    IdExt,
    DrawableExt,
    WindowExt,
    FrameExt,
    CompositeExt,
    PackerExt
);

impl List {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXList_new(parent.as_raw() as *mut FXComposite) })
            .with_layout(Layout::Fill)
    }
}

impl_widget!(ProgressBar, IdExt, DrawableExt, WindowExt, FrameExt);
impl ProgressBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXProgressBar_new(parent.as_raw() as *mut FXComposite) })
            .with_height(HEIGHT)
    }
    pub fn progress(&self) -> u32 {
        unsafe { FXProgressBar_get_progress(self.as_raw()) }
    }
    pub fn total(&self) -> u32 {
        unsafe { FXProgressBar_get_total(self.as_raw()) }
    }
    pub fn increment(&self, value: u32) {
        unsafe { FXProgressBar_increment(self.as_raw(), value) }
    }
    pub fn show_number(&self) {
        unsafe { FXProgressBar_show_number(self.as_raw()) }
    }
    pub fn hide_number(&self) {
        unsafe { FXProgressBar_hide_number(self.as_raw()) }
    }
    pub fn bar_size(&self) -> i32 {
        unsafe { FXProgressBar_get_bar_size(self.as_raw()) }
    }
    pub fn set_progress(&self, progress: u32) {
        unsafe { FXProgressBar_set_progress(self.as_raw(), progress) }
    }
    pub fn set_value(&self, value: u32) {
        self.set_progress(value);
    }
    pub fn set_total(&self, total: u32) {
        unsafe { FXProgressBar_set_total(self.as_raw(), total) }
    }
    pub fn set_bar_size(&self, size: i32) {
        unsafe { FXProgressBar_set_bar_size(self.as_raw(), size) }
    }
    pub fn with_total(self, value: u32) -> Self {
        self.set_total(value);
        self
    }
}

impl_widget!(TriStateButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl TriStateButton {
    pub fn new(parent: &impl CompositeExt, text1: &str, text2: &str, text3: &str) -> Self {
        Self::from_raw(unsafe {
            FXTriStateButton_new(
                parent.as_raw() as *mut FXComposite,
                CString::new(text1).unwrap().as_ptr(),
                CString::new(text2).unwrap().as_ptr(),
                CString::new(text3).unwrap().as_ptr(),
            )
        })
        .with_layout(Layout::FillX)
    }
}

impl_widget!(ToggleButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl ToggleButton {
    pub fn new(parent: &impl CompositeExt, title: &str, title_: &str) -> Self {
        Self::from_raw(unsafe {
            FXToggleButton_new(
                parent.as_raw() as *mut FXComposite,
                CString::new(title).unwrap().as_ptr(),
                CString::new(title_).unwrap().as_ptr(),
            )
        })
        .with_layout(Layout::FillX)
    }
}

impl_widget!(RadioButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl RadioButton {
    pub fn new(parent: &impl CompositeExt, title: &str) -> Self {
        Self::from_raw(unsafe {
            FXRadioButton_new(
                parent.as_raw() as *mut FXComposite,
                CString::new(title).unwrap().as_ptr(),
            )
        })
        .with_layout(Layout::FillX)
    }
    pub fn check(&self) -> bool {
        unsafe { FXRadioButton_get_check(self.as_raw()) != 0 }
    }
    pub fn set_check(&self, check: bool) {
        unsafe { FXRadioButton_set_check(self.as_raw(), check as u8) }
    }
}

impl_widget!(ScrollBar, IdExt, FrameExt, DrawableExt, WindowExt);
impl ScrollBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXScrollBar_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn position(&self) -> i32 {
        unsafe { FXScrollBar_get_position(self.as_raw()) }
    }
    pub fn set_position(&self, pos: i32) {
        unsafe {
            FXScrollBar_set_position(self.as_raw(), pos);
        }
    }
    pub fn set_range(&self, hi: i32) {
        unsafe {
            FXScrollBar_set_range(self.as_raw(), hi);
        }
    }
}

impl_widget!(Slider, IdExt, FrameExt, DrawableExt, WindowExt);
impl Slider {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXSlider_new(parent.as_raw() as *mut FXComposite) })
            .with_selector(Selector::CHANGED)
            .with_layout(Layout::FillX)
    }
}

impl_widget!(
    Spinner,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl Spinner {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXSpinner_new(parent.as_raw() as *mut FXComposite) })
            .with_layout(Layout::FillX)
    }
    pub fn decrement(&self) {
        unsafe { FXSpinner_decrement(self.as_raw()) }
    }
}

impl_widget!(
    TabBar,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    PackerExt,
    CompositeExt
);
impl TabBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXTabBar_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    TabBook,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    PackerExt,
    CompositeExt
);

impl TabBook {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXTabBook_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn set_current(&self, index: i32) {
        unsafe {
            FXTabBook_set_current(self.as_raw(), index);
        }
    }
    pub fn current(&self) -> i32 {
        unsafe { FXTabBook_get_current(self.as_raw()) }
    }
    pub fn num_children(&self) -> i32 {
        unsafe { FXTabBook_get_num_children(self.as_raw()) }
    }
    pub fn with_current(self, index: i32) -> Self {
        self.set_current(index);
        self
    }
}

impl_widget!(
    TabItem,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    PackerExt,
    CompositeExt
);

impl TabItem {
    pub fn new(parent: &TabBook, text: &str) -> Self {
        let c_text = CString::new(text).unwrap();
        unsafe { Self::from_raw(FXTabItem_new(parent.as_raw(), c_text.as_ptr())) }
    }
    pub fn set_text(&self, text: &str) {
        unsafe {
            FXTabItem_set_text(self.as_raw(), CString::new(text).unwrap().as_ptr());
        }
    }
    pub fn text(&self) -> String {
        unsafe {
            let ptr = FXTabItem_get_text(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

impl_widget!(Table, IdExt, FrameExt, DrawableExt, WindowExt);
impl Table {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXTable_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn set_table_size(&self, rows: i32, cols: i32) {
        unsafe {
            FXTable_set_table_size(self.as_raw(), rows, cols);
        }
    }

    pub fn set_item_text(&self, row: i32, col: i32, text: &str) {
        unsafe {
            FXTable_set_item_text(
                self.as_raw(),
                row,
                col,
                CString::new(text).unwrap().as_ptr(),
            );
        }
    }

    pub fn item_text(&self, row: i32, col: i32) -> String {
        unsafe {
            let ptr = FXTable_get_item_text(self.as_raw(), row, col);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

impl_widget!(Text, IdExt, FrameExt, DrawableExt, WindowExt);
impl Text {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXText_new(parent.as_raw() as *mut FXComposite) })
            .with_selector(Selector::CHANGED)
            .with_layout(Layout::Fill)
    }
}

impl_widget!(TextField, IdExt, FrameExt, DrawableExt, WindowExt);
impl TextField {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXTextField_new(parent.as_raw() as *mut FXComposite) })
            .with_selector(Selector::CHANGED)
            .with_layout(Layout::FillX)
    }
}

impl_widget!(
    TreeList,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl TreeList {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXTreeList_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn add_item_first(&self, prt: &TreeItem, text: &str) -> TreeItem {
        unsafe {
            TreeItem::from_raw(FXTreeList_append_item(
                self.as_raw(),
                prt.as_raw(),
                CString::new(text).unwrap().as_ptr(),
            ))
        }
    }
    pub fn clear_items(&self) {
        unsafe {
            FXTreeList_clear_items(self.as_raw());
        }
    }
}

pub struct TreeItem(Option<NonNull<FXTreeItem>>);

impl ObjectExt for TreeItem {
    type T = FXTreeItem;
    fn as_raw(&self) -> *mut Self::T {
        self.0.expect("Empty FXObject!").as_ptr()
    }
    fn from_raw(ptr: *mut Self::T) -> Self {
        Self(NonNull::new(ptr))
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
        Self::from_raw(unsafe {
            FXMainWindow_new(app.as_raw(), CString::new(title_).unwrap().as_ptr(), w, h)
        })
        .with_pad(0)
    }
    pub fn show(&self) {
        unsafe {
            FXMainWindow_show(self.as_raw());
        }
    }
}

impl_widget!(
    MenuBar,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl MenuBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXMenuBar_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(
    MenuPane,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl MenuPane {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(foxtk_sys::FXMenuPane_new(parent.as_raw() as *mut FXWindow)) }
    }
}

impl_widget!(MenuButton, IdExt, DrawableExt, WindowExt, FrameExt);
impl MenuButton {
    pub fn new(prt: &impl CompositeExt, text_: &str, pane: &MenuPane) -> Self {
        Self::from_raw(unsafe {
            foxtk_sys::FXMenuButton_new(
                prt.as_raw() as *mut FXComposite,
                CString::new(text_).unwrap().as_ptr(),
                pane.as_raw() as *mut FXPopup,
            )
        })
        .with_layout(Layout::FillX)
    }
}

impl_widget!(MenuTitle, IdExt, FrameExt, DrawableExt, WindowExt);
impl MenuTitle {
    pub fn new(prt: &impl CompositeExt, text_: &str, pane: &MenuPane) -> Self {
        Self::from_raw(unsafe {
            foxtk_sys::FXMenuTitle_new(
                prt.as_raw() as *mut FXComposite,
                CString::new(text_).unwrap().as_ptr(),
                pane.as_raw() as *mut FXPopup,
            )
        })
        .with_layout(Layout::FillX)
    }
}

impl_widget!(MenuRadio, IdExt, FrameExt, DrawableExt, WindowExt);
impl MenuRadio {
    pub fn new(parent: &impl CompositeExt, text: &str) -> Self {
        Self::from_raw(unsafe {
            FXMenuRadio_new(
                parent.as_raw() as *mut FXComposite,
                CString::new(text).unwrap().as_ptr(),
            )
        })
    }
}

impl_widget!(MenuCheck, IdExt, FrameExt, DrawableExt, WindowExt);
impl MenuCheck {
    pub fn new(parent: &impl CompositeExt, text: &str) -> Self {
        Self::from_raw(unsafe {
            FXMenuCheck_new(
                parent.as_raw() as *mut FXComposite,
                CString::new(text).unwrap().as_ptr(),
            )
        })
    }
}

impl_widget!(MenuCommand, IdExt, FrameExt, DrawableExt, WindowExt);
impl MenuCommand {
    pub fn new(parent: &impl CompositeExt, text: &str) -> Self {
        Self::from_raw(unsafe {
            FXMenuCommand_new(
                parent.as_raw() as *mut FXComposite,
                CString::new(text).unwrap().as_ptr(),
            )
        })
    }
}

impl_widget!(StatusLine, IdExt, FrameExt, DrawableExt, WindowExt);
impl StatusLine {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXStatusLine_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(BitmapFrame, IdExt, FrameExt, DrawableExt, WindowExt);
impl BitmapFrame {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXBitmapFrame_new(parent.as_raw() as *mut FXComposite) })
    }
    pub fn set_justify(&self, justify: Justify) {
        unsafe {
            FXBitmapFrame_set_justify(self.as_raw(), justify as u32);
        }
    }
    pub fn justify(&self) -> Justify {
        unsafe {
            std::mem::transmute::<u32, Justify>(FXBitmapFrame_get_justify(self.as_raw()) as u32)
        }
    }
    pub fn with_justify(self, justify: Justify) -> Self {
        self.set_justify(justify);
        self
    }
}

impl_widget!(BitmapView, IdExt, FrameExt, DrawableExt, WindowExt);
impl BitmapView {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXBitmapView_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(ImageFrame, IdExt, FrameExt, DrawableExt, WindowExt);
impl ImageFrame {
    pub fn new(parent: &impl CompositeExt, img: &Image) -> Self {
        Self::from_raw(unsafe {
            FXImageFrame_new(parent.as_raw() as *mut FXComposite, img.as_raw())
        })
    }
    pub fn set_justify(&self, justify: Justify) {
        unsafe {
            FXImageFrame_set_justify(self.as_raw(), justify as u32);
        }
    }
    pub fn justify(&self) -> Justify {
        unsafe {
            std::mem::transmute::<u32, Justify>(FXImageFrame_get_justify(self.as_raw()) as u32)
        }
    }
    pub fn with_justify(self, justify: Justify) -> Self {
        self.set_justify(justify);
        self
    }
}

impl_widget!(Image, IdExt);
impl Image {
    pub fn new(app: &App) -> Self {
        Self::from_raw(unsafe { FXImage_new(app.as_raw()) })
    }
}

impl_widget!(ImageView, IdExt, FrameExt, DrawableExt, WindowExt);
impl ImageView {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXImageView_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(GLVisual, IdExt);
impl GLVisual {
    pub fn new(app: &App) -> Self {
        Self::from_raw(unsafe { FXGLVisual_new(app.as_raw()) })
    }
}

impl_widget!(GLCanvas, IdExt, FrameExt, DrawableExt, WindowExt);
impl GLCanvas {
    pub fn new(parent: &impl CompositeExt, visual: &GLVisual) -> Self {
        Self::from_raw(unsafe {
            FXGLCanvas_new(parent.as_raw() as *mut FXComposite, visual.as_raw())
        })
    }
}

impl_widget!(GLViewer, IdExt, FrameExt, DrawableExt, WindowExt);
impl GLViewer {
    pub fn new(parent: &impl CompositeExt, visual: &GLVisual) -> Self {
        Self::from_raw(unsafe {
            FXGLViewer_new(parent.as_raw() as *mut FXComposite, visual.as_raw())
        })
    }
}

impl_widget!(Header, IdExt, FrameExt, DrawableExt, WindowExt);
impl Header {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXHeader_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(Ruler, IdExt, FrameExt, DrawableExt, WindowExt);
impl Ruler {
    pub fn new(parent: &impl CompositeExt, orientation: RulerOrientation) -> Self {
        Self::from_raw(unsafe {
            FXRuler_new(parent.as_raw() as *mut FXComposite, orientation as u32)
        })
    }
    pub fn with_orientation(self, _orientation: RulerOrientation) -> Self {
        // Note: orientation is set in constructor, this is just for consistency
        self
    }
}

impl_widget!(Spring, IdExt, FrameExt, DrawableExt, WindowExt);
impl Spring {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXSpring_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(
    DockBar,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl DockBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXDockBar_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(
    DockSite,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl DockSite {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXDockSite_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(DockHandler, IdExt);
impl DockHandler {
    pub fn new(docksite: &DockSite) -> Self {
        Self::from_raw(unsafe { FXDockHandler_new(docksite.as_raw()) })
    }
}

impl_widget!(DockTitle, IdExt, FrameExt, DrawableExt, WindowExt);
impl DockTitle {
    pub fn new(bar: &DockBar, title: &str) -> Self {
        Self::from_raw(unsafe {
            FXDockTitle_new(bar.as_raw(), CString::new(title).unwrap().as_ptr())
        })
    }
    pub fn set_justify(&self, justify: Justify) {
        unsafe {
            FXDockTitle_set_justify(self.as_raw(), justify as u32);
        }
    }
    pub fn justify(&self) -> Justify {
        unsafe {
            std::mem::transmute::<u32, Justify>(FXDockTitle_get_justify(self.as_raw()) as u32)
        }
    }
    pub fn with_justify(self, justify: Justify) -> Self {
        self.set_justify(justify);
        self
    }
}

impl_widget!(
    ToolBar,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl ToolBar {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXToolBar_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(ToolBarGrip, IdExt, FrameExt, DrawableExt, WindowExt);
impl ToolBarGrip {
    pub fn new(toolbar: &ToolBar) -> Self {
        Self::from_raw(unsafe { FXToolBarGrip_new(toolbar.as_raw()) })
    }
}

impl_widget!(ToolBarTab, IdExt, FrameExt, DrawableExt, WindowExt);
impl ToolBarTab {
    pub fn new(toolbar: &ToolBar) -> Self {
        Self::from_raw(unsafe { FXToolBarTab_new(toolbar.as_raw()) })
    }
}

impl_widget!(
    FoldingList,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl FoldingList {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXFoldingList_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(
    MDIClient,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl MDIClient {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXMDIClient_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(
    MDIChild,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    TopWindowExt
);
impl MDIChild {
    pub fn new(client: &MDIClient, title: &str) -> Self {
        Self::from_raw(unsafe {
            FXMDIChild_new(client.as_raw(), CString::new(title).unwrap().as_ptr())
        })
    }
}

impl_widget!(MDIDeleteButton, IdExt, FrameExt, DrawableExt, WindowExt);
impl MDIDeleteButton {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXMDIDeleteButton_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(MDIRestoreButton, IdExt, FrameExt, DrawableExt, WindowExt);
impl MDIRestoreButton {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXMDIRestoreButton_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(MDIMaximizeButton, IdExt, FrameExt, DrawableExt, WindowExt);
impl MDIMaximizeButton {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXMDIMaximizeButton_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(MDIMinimizeButton, IdExt, FrameExt, DrawableExt, WindowExt);
impl MDIMinimizeButton {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXMDIMinimizeButton_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(
    MDIMenu,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl MDIMenu {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { FXMDIMenu_new(parent.as_raw() as *mut FXComposite) })
    }
}

impl_widget!(Popup, IdExt, FrameExt, DrawableExt, WindowExt, CompositeExt);
impl Popup {
    pub fn new(owner: &impl WindowExt) -> Self {
        Self::from_raw(unsafe { FXPopup_new(owner.as_raw() as *mut FXWindow) })
    }
}

impl_widget!(MDIWindowButton, IdExt, FrameExt, DrawableExt, WindowExt);
impl MDIWindowButton {
    pub fn new(parent: &impl CompositeExt, pup: &Popup) -> Self {
        Self::from_raw(unsafe {
            FXMDIWindowButton_new(parent.as_raw() as *mut FXComposite, pup.as_raw())
        })
    }
}

impl_widget!(TableItem, IdExt, FrameExt, DrawableExt, WindowExt);
impl TableItem {
    pub fn new(table: &Table, text: &str) -> Self {
        Self::from_raw(unsafe {
            FXTableItem_new(table.as_raw(), CString::new(text).unwrap().as_ptr())
        })
    }
}

impl_textable!(
    Button,
    GroupBox,
    Label,
    StatusBar,
    Text,
    TextField,
    RadioButton,
    StatusLine
);
impl_selector!(ComboBox, List, ListBox);
impl_ranger!(Dial, Knob, Slider, Spinner);
impl_editable!(ComboBox, Spinner, Table, Text, TextField);
