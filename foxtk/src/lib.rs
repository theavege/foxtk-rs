#![doc = include_str!("../README.md")]

pub mod enums;
pub mod prelude;
use {
    foxtk_sys::*,
    prelude::*,
    std::{ffi::CString, ptr::NonNull},
};

pub(crate) const HEIGHT: i32 = 30;

pub struct App(Option<NonNull<ObjectPtr>>);

impl App {
    pub fn new(name_: &str, vendor_: &str) -> Self {
        let args = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const i8>>();
        Self::from_raw(unsafe {
            fx_app_new(
                CString::new(name_).unwrap().as_ptr(),
                CString::new(vendor_).unwrap().as_ptr(),
                args.len() as i32,
                args.as_ptr() as *mut *mut i8,
            )
        })
    }
}
impl ObjectExt for App {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl AppExt for App {}

#[derive(Default)]
pub struct Window(Option<NonNull<ObjectPtr>>);
impl Window {
    pub fn new(parent: &impl ObjectExt, opts: u32, x: i32, y: i32, w: i32, h: i32) -> Self {
        Self::from_raw(unsafe { fx_shell_new(parent.as_raw(), opts, x, y, w, h) })
    }
}
impl ObjectExt for Window {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for Window {}
impl FrameExt for Window {}
impl DrawableExt for Window {}
impl WindowExt for Window {}

#[derive(Default)]
pub struct Shell(Option<NonNull<ObjectPtr>>);
impl Shell {
    pub fn new(parent: &impl ObjectExt, opts: u32, x: i32, y: i32, w: i32, h: i32) -> Self {
        Self::from_raw(unsafe { fx_shell_new(parent.as_raw(), opts, x, y, w, h) })
    }
}
impl ObjectExt for Shell {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for Shell {}
impl FrameExt for Shell {}
impl DrawableExt for Shell {}
impl WindowExt for Shell {}

#[derive(Default)]
pub struct RootWindow(Option<NonNull<ObjectPtr>>);
impl RootWindow {
    pub fn new(app: &impl AppExt) -> Self {
        Self::from_raw(unsafe { fx_root_window_new(app.as_raw()) })
    }
}
impl ObjectExt for RootWindow {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for RootWindow {}
impl FrameExt for RootWindow {}
impl DrawableExt for RootWindow {}
impl WindowExt for RootWindow {}

#[derive(Default)]
pub struct ToolBarShell(Option<NonNull<ObjectPtr>>);
impl ToolBarShell {
    pub fn new(owner: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_tool_bar_shell_new(owner.as_raw()) })
    }
}
impl ObjectExt for ToolBarShell {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ToolBarShell {}
impl FrameExt for ToolBarShell {}
impl DrawableExt for ToolBarShell {}
impl WindowExt for ToolBarShell {}

#[derive(Default)]
pub struct TopWindow(Option<NonNull<ObjectPtr>>);
impl TopWindow {
    pub fn new(app: &impl AppExt, title: &str, width: i32, height: i32) -> Self {
        Self::from_raw(unsafe {
            fx_top_window_new(
                app.as_raw(),
                CString::new(title).unwrap().as_ptr(),
                width,
                height,
            )
        })
    }
}
impl ObjectExt for TopWindow {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for TopWindow {}
impl FrameExt for TopWindow {}
impl DrawableExt for TopWindow {}
impl WindowExt for TopWindow {}

#[derive(Default)]
pub struct SplashWindow(Option<NonNull<ObjectPtr>>);
impl SplashWindow {
    pub fn new(app: &impl AppExt) -> Self {
        Self::from_raw(unsafe { fx_splash_window_new(app.as_raw()) })
    }
}
impl ObjectExt for SplashWindow {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for SplashWindow {}
impl FrameExt for SplashWindow {}
impl DrawableExt for SplashWindow {}
impl WindowExt for SplashWindow {}

#[derive(Default)]
pub struct Button(Option<NonNull<ObjectPtr>>);
impl Button {
    pub fn new(parent: &impl ObjectExt, title: &str) -> Self {
        Self::from_raw(unsafe {
            fx_button_new(
                parent.as_raw(),
                CString::new(format!("&{title}").as_str()).unwrap().as_ptr(),
            )
        })
        .with_layout(Layout::FillX)
    }
    pub fn set_state(&self, state: ButtonState) {
        unsafe {
            fx_button_set_state(self.as_raw(), state as u32);
        }
    }
    pub fn with_style(self, style: ButtonStyle) -> Self {
        unsafe {
            fx_button_set_style(self.as_raw(), style as u32);
        }
        self
    }
}
impl ObjectExt for Button {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for Button {}
impl FrameExt for Button {}
impl DrawableExt for Button {}
impl WindowExt for Button {}
impl LabelExt for Button {}

#[derive(Default)]
pub struct ArrowButton(Option<NonNull<ObjectPtr>>);
impl ArrowButton {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_arrow_button_new(parent.as_raw()) })
    }
    pub fn set_size(&self, size: i32) {
        unsafe {
            fx_arrow_button_set_arrow_size(self.as_raw(), size);
        }
    }
    pub fn set_color(&self, color: Color) {
        unsafe {
            fx_arrow_button_set_arrow_color(self.as_raw(), color.bits());
        }
    }
}
impl ObjectExt for ArrowButton {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ArrowButton {}
impl FrameExt for ArrowButton {}
impl DrawableExt for ArrowButton {}
impl WindowExt for ArrowButton {}

#[derive(Default)]
pub struct Canvas(Option<NonNull<ObjectPtr>>);
impl Canvas {
    pub fn new(parent: &impl WindowExt) -> Self {
        Self::from_raw(unsafe { fx_canvas_new(parent.as_raw()) })
    }
}
impl ObjectExt for Canvas {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for Canvas {}
impl FrameExt for Canvas {}
impl DrawableExt for Canvas {}
impl WindowExt for Canvas {}
impl CanvasExt for Canvas {}
impl DCWindowExt for Canvas {}

#[derive(Default)]
pub struct CheckButton(Option<NonNull<ObjectPtr>>);
impl CheckButton {
    pub fn new(parent: &impl ObjectExt, title: &str) -> Self {
        Self::from_raw(unsafe {
            fx_check_button_new(parent.as_raw(), CString::new(title).unwrap().as_ptr())
        })
        .with_layout(Layout::FillX)
    }
}
impl ObjectExt for CheckButton {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for CheckButton {}
impl FrameExt for CheckButton {}
impl DrawableExt for CheckButton {}
impl WindowExt for CheckButton {}
impl LabelExt for CheckButton {}
impl CheckButtonExt for CheckButton {}

#[derive(Default)]
pub struct ComboBox(Option<NonNull<ObjectPtr>>);
impl ComboBox {
    pub fn new(parent: &impl WindowExt, cols: i32) -> Self {
        Self::from_raw(unsafe { fx_combo_box_new(parent.as_raw(), cols) })
            .with_layout(Layout::FillX)
    }
}
impl ObjectExt for ComboBox {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ComboBox {}
impl FrameExt for ComboBox {}
impl DrawableExt for ComboBox {}
impl WindowExt for ComboBox {}
impl PackerExt for ComboBox {}
impl CompositeExt for ComboBox {}

pub struct GroupBox(Option<NonNull<ObjectPtr>>);
impl GroupBox {
    pub fn new(parent: &impl ObjectExt, title_: &str) -> Self {
        Self::from_raw(unsafe {
            fx_groupbox_new(parent.as_raw(), CString::new(title_).unwrap().as_ptr())
        })
        .with_frame(FrameStyle::Thick)
        .with_layout(Layout::FillX)
    }
}
impl ObjectExt for GroupBox {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for GroupBox {}
impl IdExt for GroupBox {}
impl FrameExt for GroupBox {}
impl WindowExt for GroupBox {}
impl DrawableExt for GroupBox {}
impl PackerExt for GroupBox {}
impl GroupBoxExt for GroupBox {}

pub struct Spring(Option<NonNull<ObjectPtr>>);
impl Spring {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_spring_new(parent.as_raw()) })
    }
}
impl ObjectExt for Spring {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for Spring {}
impl PackerExt for Spring {}
impl IdExt for Spring {}
impl FrameExt for Spring {}
impl DrawableExt for Spring {}
impl WindowExt for Spring {}

pub struct VerticalFrame(Option<NonNull<ObjectPtr>>);
impl VerticalFrame {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_vertical_frame_new(parent.as_raw()) })
            .with_layout(Layout::FillX)
            .with_frame(FrameStyle::Thick)
    }
}
impl ObjectExt for VerticalFrame {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for VerticalFrame {}
impl IdExt for VerticalFrame {}
impl FrameExt for VerticalFrame {}
impl DrawableExt for VerticalFrame {}
impl PackerExt for VerticalFrame {}
impl WindowExt for VerticalFrame {}

pub struct HorizontalFrame(Option<NonNull<ObjectPtr>>);
impl HorizontalFrame {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_horizontal_frame_new(parent.as_raw()) })
            .with_height(HEIGHT)
            .with_frame(FrameStyle::Thick)
    }
}
impl ObjectExt for HorizontalFrame {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for HorizontalFrame {}
impl PackerExt for HorizontalFrame {}
impl IdExt for HorizontalFrame {}
impl FrameExt for HorizontalFrame {}
impl DrawableExt for HorizontalFrame {}
impl WindowExt for HorizontalFrame {}

#[derive(Default)]
pub struct Matrix(Option<NonNull<ObjectPtr>>);
impl Matrix {
    pub fn new(parent: &impl ObjectExt, rows: i32) -> Self {
        Self::from_raw(unsafe { fx_matrix_new(parent.as_raw(), rows, MatrixStyle::ByRows as u32) })
            .with_layout(Layout::Fill)
    }
    pub fn set_style(&self, style: MatrixStyle) {
        unsafe {
            fx_matrix_set_matrix_style(self.as_raw(), style as u32);
        }
    }
    pub fn set_num_rows(&self, rows: i32) {
        unsafe {
            fx_matrix_set_num_rows(self.as_raw(), rows);
        }
    }
    pub fn set_num_columns(&self, cols: i32) {
        unsafe {
            fx_matrix_set_num_columns(self.as_raw(), cols);
        }
    }
    pub fn style(&self) -> MatrixStyle {
        unsafe {
            std::mem::transmute::<u32, MatrixStyle>(fx_matrix_get_matrix_style(self.as_raw()) as u32)
        }
    }
    pub fn num_rows(&self) -> i32 {
        unsafe { fx_matrix_get_num_rows(self.as_raw()) }
    }
    pub fn num_columns(&self) -> i32 {
        unsafe { fx_matrix_get_num_columns(self.as_raw()) }
    }
}
impl ObjectExt for Matrix {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for Matrix {}
impl PackerExt for Matrix {}
impl IdExt for Matrix {}
impl FrameExt for Matrix {}
impl DrawableExt for Matrix {}
impl WindowExt for Matrix {}

#[derive(Default)]
pub struct Splitter(Option<NonNull<ObjectPtr>>);
impl Splitter {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_splitter_new(parent.as_raw(), SplitterStyle::Normal as u32) })
            .with_layout(Layout::Fill)
    }
    pub fn with_style(self, style: SplitterStyle) -> Self {
        unsafe {
            fx_splitter_set_splitter_style(self.as_raw(), style as u32);
        }
        self
    }
    pub fn set_style(&self, style: SplitterStyle) {
        unsafe {
            fx_splitter_set_splitter_style(self.as_raw(), style as u32);
        }
    }
    pub fn style(&self) -> SplitterStyle {
        unsafe {
            std::mem::transmute::<u32, SplitterStyle>(
                fx_splitter_get_splitter_style(self.as_raw()) as u32
            )
        }
    }
    pub fn set_split(&self, index: i32, size: i32) {
        unsafe {
            fx_splitter_set_split(self.as_raw(), index, size);
        }
    }
    pub fn split(&self, index: i32) -> i32 {
        unsafe { fx_splitter_get_split(self.as_raw(), index) }
    }
    pub fn set_bar_size(&self, size: i32) {
        unsafe {
            fx_splitter_set_bar_size(self.as_raw(), size);
        }
    }
    pub fn bar_size(&self) -> i32 {
        unsafe { fx_splitter_get_bar_size(self.as_raw()) }
    }
}
impl ObjectExt for Splitter {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for Splitter {}
impl IdExt for Splitter {}
impl FrameExt for Splitter {}
impl DrawableExt for Splitter {}
impl WindowExt for Splitter {}

#[derive(Default)]
pub struct FourSplitter(Option<NonNull<ObjectPtr>>);
impl FourSplitter {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe {
            fx_four_splitter_new(parent.as_raw(), SplitterStyle::Normal as u32)
        })
        .with_layout(Layout::Fill)
    }
}
impl ObjectExt for FourSplitter {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for FourSplitter {}
impl PackerExt for FourSplitter {}
impl IdExt for FourSplitter {}
impl FrameExt for FourSplitter {}
impl DrawableExt for FourSplitter {}
impl WindowExt for FourSplitter {}

#[derive(Default)]
pub struct ScrollArea(Option<NonNull<ObjectPtr>>);
impl ScrollArea {
    pub fn new(parent: &impl ObjectExt, opts: u32, x: i32, y: i32, w: i32, h: i32) -> Self {
        Self::from_raw(unsafe { fx_scroll_area_new(parent.as_raw(), opts, x, y, w, h) })
            .with_layout(Layout::Fill)
    }
}
impl ObjectExt for ScrollArea {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for ScrollArea {}
impl PackerExt for ScrollArea {}
impl IdExt for ScrollArea {}
impl FrameExt for ScrollArea {}
impl DrawableExt for ScrollArea {}
impl WindowExt for ScrollArea {}

#[derive(Default)]
pub struct ScrollWindow(Option<NonNull<ObjectPtr>>);
impl ScrollWindow {
    pub fn new(parent: &impl ObjectExt, opts: u32, x: i32, y: i32, w: i32, h: i32) -> Self {
        Self::from_raw(unsafe { fx_scroll_window_new(parent.as_raw(), opts, x, y, w, h) })
            .with_layout(Layout::Fill)
    }
}
impl ObjectExt for ScrollWindow {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for ScrollWindow {}
impl PackerExt for ScrollWindow {}
impl IdExt for ScrollWindow {}
impl FrameExt for ScrollWindow {}
impl DrawableExt for ScrollWindow {}
impl WindowExt for ScrollWindow {}

#[derive(Default)]
pub struct Switcher(Option<NonNull<ObjectPtr>>);
impl Switcher {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_switcher_new(parent.as_raw()) })
            .with_layout(Layout::Fill)
            .with_frame(FrameStyle::Thick)
    }
}
impl ObjectExt for Switcher {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for Switcher {}
impl SwitcherExt for Switcher {}
impl FrameExt for Switcher {}
impl IdExt for Switcher {}
impl DrawableExt for Switcher {}
impl WindowExt for Switcher {}
impl PackerExt for Switcher {}

#[derive(Default)]
pub struct Label(Option<NonNull<ObjectPtr>>);
impl Label {
    pub fn new(parent: &impl ObjectExt, title: &str) -> Self {
        Self::from_raw(unsafe {
            fx_label_new(
                parent.as_raw(),
                CString::new(format!("&{title}").as_str()).unwrap().as_ptr(),
            )
        })
        .with_height(HEIGHT)
    }
}
impl ObjectExt for Label {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for Label {}
impl DrawableExt for Label {}
impl WindowExt for Label {}
impl FrameExt for Label {}
impl LabelExt for Label {}

#[derive(Default)]
pub struct Dial(Option<NonNull<ObjectPtr>>);
impl Dial {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_dial_new(parent.as_raw()) })
    }
}
impl ObjectExt for Dial {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for Dial {}
impl DrawableExt for Dial {}
impl WindowExt for Dial {}
impl FrameExt for Dial {}

#[derive(Default)]
pub struct Knob(Option<NonNull<ObjectPtr>>);
impl Knob {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_knob_new(parent.as_raw()) })
    }
}
impl ObjectExt for Knob {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for Knob {}
impl DrawableExt for Knob {}
impl WindowExt for Knob {}
impl FrameExt for Knob {}

#[derive(Default)]
pub struct RealSpinner(Option<NonNull<ObjectPtr>>);
impl RealSpinner {
    pub fn new(parent: &impl ObjectExt, cols: i32) -> Self {
        Self::from_raw(unsafe { fx_real_spinner_new(parent.as_raw(), cols) })
    }
}
impl ObjectExt for RealSpinner {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for RealSpinner {}
impl DrawableExt for RealSpinner {}
impl WindowExt for RealSpinner {}
impl FrameExt for RealSpinner {}

#[derive(Default)]
pub struct RealSlider(Option<NonNull<ObjectPtr>>);
impl RealSlider {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_real_slider_new(parent.as_raw()) })
    }
}
impl ObjectExt for RealSlider {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for RealSlider {}
impl DrawableExt for RealSlider {}
impl WindowExt for RealSlider {}
impl FrameExt for RealSlider {}

#[derive(Default)]
pub struct ColorWell(Option<NonNull<ObjectPtr>>);
impl ColorWell {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_color_well_new(parent.as_raw()) })
    }
}
impl ObjectExt for ColorWell {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ColorWell {}
impl DrawableExt for ColorWell {}
impl WindowExt for ColorWell {}
impl FrameExt for ColorWell {}

#[derive(Default)]
pub struct ColorWheel(Option<NonNull<ObjectPtr>>);
impl ColorWheel {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_color_wheel_new(parent.as_raw()) })
    }
}
impl ObjectExt for ColorWheel {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ColorWheel {}
impl DrawableExt for ColorWheel {}
impl WindowExt for ColorWheel {}
impl FrameExt for ColorWheel {}

#[derive(Default)]
pub struct ColorRing(Option<NonNull<ObjectPtr>>);
impl ColorRing {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_color_ring_new(parent.as_raw()) })
    }
}
impl ObjectExt for ColorRing {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ColorRing {}
impl DrawableExt for ColorRing {}
impl WindowExt for ColorRing {}
impl FrameExt for ColorRing {}

#[derive(Default)]
pub struct ColorBar(Option<NonNull<ObjectPtr>>);
impl ColorBar {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_color_bar_new(parent.as_raw()) })
    }
}
impl ObjectExt for ColorBar {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ColorBar {}
impl DrawableExt for ColorBar {}
impl WindowExt for ColorBar {}
impl FrameExt for ColorBar {}

#[derive(Default)]
pub struct GradientBar(Option<NonNull<ObjectPtr>>);
impl GradientBar {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_gradient_bar_new(parent.as_raw()) })
    }
}
impl ObjectExt for GradientBar {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for GradientBar {}
impl DrawableExt for GradientBar {}
impl WindowExt for GradientBar {}
impl FrameExt for GradientBar {}

#[derive(Default)]
pub struct SevenSegment(Option<NonNull<ObjectPtr>>);
impl SevenSegment {
    pub fn new(parent: &impl ObjectExt, text: &str) -> Self {
        Self::from_raw(unsafe {
            fx_7segment_new(parent.as_raw(), CString::new(text).unwrap().as_ptr())
        })
    }
}
impl ObjectExt for SevenSegment {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for SevenSegment {}
impl DrawableExt for SevenSegment {}
impl WindowExt for SevenSegment {}
impl FrameExt for SevenSegment {}

#[derive(Default)]
pub struct ColorDialog(Option<NonNull<ObjectPtr>>);
impl ColorDialog {
    pub fn new(owner: &impl ObjectExt, title: &str) -> Self {
        Self::from_raw(unsafe {
            fx_color_dialog_new(owner.as_raw(), CString::new(title).unwrap().as_ptr())
        })
    }
}
impl ObjectExt for ColorDialog {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ColorDialog {}
impl DrawableExt for ColorDialog {}
impl WindowExt for ColorDialog {}
impl FrameExt for ColorDialog {}

#[derive(Default)]
pub struct ListBox(Option<NonNull<ObjectPtr>>);
impl ListBox {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { fx_list_box_new(parent.as_raw()) })
            .with_layout(Layout::FillX)
            .with_num_visible(3)
    }
}
impl ObjectExt for ListBox {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ListBox {}
impl DrawableExt for ListBox {}
impl WindowExt for ListBox {}
impl FrameExt for ListBox {}
impl CompositeExt for ListBox {}
impl PackerExt for ListBox {}

#[derive(Default)]
pub struct List(Option<NonNull<ObjectPtr>>);
impl List {
    pub fn new(parent: &impl CompositeExt) -> Self {
        Self::from_raw(unsafe { fx_list_new(parent.as_raw()) }).with_layout(Layout::Fill)
    }
}
impl ObjectExt for List {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for List {}
impl FrameExt for List {}
impl WindowExt for List {}
impl DrawableExt for List {}
impl PackerExt for List {}
impl CompositeExt for List {}

#[derive(Default)]
pub struct ProgressBar(Option<NonNull<ObjectPtr>>);
impl ProgressBar {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_progressbar_new(parent.as_raw()) }).with_height(HEIGHT)
    }
}
impl ObjectExt for ProgressBar {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ProgressBar {}
impl FrameExt for ProgressBar {}
impl WindowExt for ProgressBar {}
impl DrawableExt for ProgressBar {}
impl ProgressBarExt for ProgressBar {}

#[derive(Default)]
pub struct TriStateButton(Option<NonNull<ObjectPtr>>);
impl TriStateButton {
    pub fn new(parent: &impl ObjectExt, text1: &str, text2: &str, text3: &str) -> Self {
        Self::from_raw(unsafe {
            fx_tri_state_button_new(
                parent.as_raw(),
                CString::new(text1).unwrap().as_ptr(),
                CString::new(text2).unwrap().as_ptr(),
                CString::new(text3).unwrap().as_ptr(),
            )
        })
        .with_layout(Layout::FillX)
    }
}
impl ObjectExt for TriStateButton {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for TriStateButton {}
impl FrameExt for TriStateButton {}
impl DrawableExt for TriStateButton {}
impl WindowExt for TriStateButton {}
impl LabelExt for TriStateButton {}

#[derive(Default)]
pub struct ToggleButton(Option<NonNull<ObjectPtr>>);
impl ToggleButton {
    pub fn new(parent: &impl ObjectExt, title: &str, title_: &str) -> Self {
        Self::from_raw(unsafe {
            fx_toggle_button_new(
                parent.as_raw(),
                CString::new(title).unwrap().as_ptr(),
                CString::new(title_).unwrap().as_ptr(),
            )
        })
        .with_layout(Layout::FillX)
    }
}
impl ObjectExt for ToggleButton {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ToggleButton {}
impl FrameExt for ToggleButton {}
impl DrawableExt for ToggleButton {}
impl WindowExt for ToggleButton {}
impl LabelExt for ToggleButton {}
impl RadioButtonExt for ToggleButton {}

#[derive(Default)]
pub struct RadioButton(Option<NonNull<ObjectPtr>>);
impl RadioButton {
    pub fn new(parent: &impl ObjectExt, title: &str) -> Self {
        Self::from_raw(unsafe {
            fx_radio_button_new(parent.as_raw(), CString::new(title).unwrap().as_ptr())
        })
        .with_layout(Layout::FillX)
    }
}
impl ObjectExt for RadioButton {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for RadioButton {}
impl FrameExt for RadioButton {}
impl DrawableExt for RadioButton {}
impl WindowExt for RadioButton {}
impl LabelExt for RadioButton {}
impl RadioButtonExt for RadioButton {}

pub struct ScrollBar(Option<NonNull<ObjectPtr>>);
impl ScrollBar {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(fx_scroll_bar_new(parent.as_raw())) }
    }
}
impl ObjectExt for ScrollBar {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ScrollBar {}
impl FrameExt for ScrollBar {}
impl DrawableExt for ScrollBar {}
impl WindowExt for ScrollBar {}
impl ScrollBarExt for ScrollBar {}

#[derive(Default)]
pub struct Slider(Option<NonNull<ObjectPtr>>);
impl Slider {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_slider_new(parent.as_raw()) })
            .with_selector(Selector::CHANGED)
            .with_layout(Layout::FillX)
    }
}
impl ObjectExt for Slider {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for Slider {}
impl FrameExt for Slider {}
impl WindowExt for Slider {}
impl DrawableExt for Slider {}

#[derive(Default)]
pub struct Spinner(Option<NonNull<ObjectPtr>>);
impl Spinner {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_spinner_new(parent.as_raw()) }).with_layout(Layout::FillX)
    }
}
impl ObjectExt for Spinner {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for Spinner {}
impl FrameExt for Spinner {}
impl WindowExt for Spinner {}
impl CompositeExt for Spinner {}
impl DrawableExt for Spinner {}
impl PackerExt for Spinner {}
impl SpinnerExt for Spinner {}

pub struct TabBar(Option<NonNull<ObjectPtr>>);
impl TabBar {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(fx_tab_bar_new(parent.as_raw())) }
    }
}
impl ObjectExt for TabBar {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for TabBar {}
impl FrameExt for TabBar {}
impl DrawableExt for TabBar {}
impl WindowExt for TabBar {}
impl CompositeExt for TabBar {}
impl PackerExt for TabBar {}

pub struct TabBook(Option<NonNull<ObjectPtr>>);
impl TabBook {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(fx_tab_book_new(parent.as_raw())) }
    }
}
impl ObjectExt for TabBook {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for TabBook {}
impl FrameExt for TabBook {}
impl DrawableExt for TabBook {}
impl WindowExt for TabBook {}
impl CompositeExt for TabBook {}
impl PackerExt for TabBook {}

pub struct TabItem(Option<NonNull<ObjectPtr>>);
impl TabItem {
    pub fn new(parent: &impl WindowExt, text: &str) -> Self {
        let c_text = CString::new(text).unwrap();
        unsafe { Self::from_raw(fx_tab_item_new(parent.as_raw(), c_text.as_ptr())) }
    }
}
impl ObjectExt for TabItem {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for TabItem {}
impl FrameExt for TabItem {}
impl DrawableExt for TabItem {}
impl WindowExt for TabItem {}
impl CompositeExt for TabItem {}
impl PackerExt for TabItem {}
impl TabItemExt for TabItem {}

pub struct Table(Option<NonNull<ObjectPtr>>);
impl Table {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(fx_table_new(parent.as_raw())) }
    }
}
impl ObjectExt for Table {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for Table {}
impl FrameExt for Table {}
impl WindowExt for Table {}
impl DrawableExt for Table {}
impl TableExt for Table {}

#[derive(Default)]
pub struct Text(Option<NonNull<ObjectPtr>>);
impl Text {
    pub fn new(parent: &impl WindowExt) -> Self {
        Self::from_raw(unsafe { fx_text_new(parent.as_raw()) })
            .with_selector(Selector::CHANGED)
            .with_layout(Layout::Fill)
    }
}
impl ObjectExt for Text {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for Text {}
impl FrameExt for Text {}
impl DrawableExt for Text {}
impl WindowExt for Text {}
impl TextExt for Text {}

#[derive(Default)]
pub struct TextField(Option<NonNull<ObjectPtr>>);
impl TextField {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_textfield_new(parent.as_raw()) })
            .with_selector(Selector::CHANGED)
            .with_layout(Layout::FillX)
    }
}
impl ObjectExt for TextField {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for TextField {}
impl WindowExt for TextField {}
impl FrameExt for TextField {}
impl DrawableExt for TextField {}
impl TextFieldExt for TextField {}

#[derive(Default)]
pub struct StatusBar(std::option::Option<NonNull<ObjectPtr>>);
impl StatusBar {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_status_bar_new(parent.as_raw()) })
    }
}
impl ObjectExt for StatusBar {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for StatusBar {}
impl FrameExt for StatusBar {}
impl DrawableExt for StatusBar {}
impl WindowExt for StatusBar {}

#[derive(Default)]
pub struct OptionWidget(std::option::Option<NonNull<ObjectPtr>>);
impl OptionWidget {
    pub fn new(parent: &impl ObjectExt, title: &str) -> Self {
        Self::from_raw(unsafe {
            fx_option_new(parent.as_raw(), CString::new(title).unwrap().as_ptr())
        })
        .with_layout(Layout::FillX)
    }
}
impl ObjectExt for OptionWidget {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for OptionWidget {}
impl FrameExt for OptionWidget {}
impl DrawableExt for OptionWidget {}
impl WindowExt for OptionWidget {}

#[derive(Default)]
pub struct OptionMenu(std::option::Option<NonNull<ObjectPtr>>);
impl OptionMenu {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_option_menu_new(parent.as_raw()) }).with_layout(Layout::FillX)
    }
}
impl ObjectExt for OptionMenu {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for OptionMenu {}
impl FrameExt for OptionMenu {}
impl DrawableExt for OptionMenu {}
impl WindowExt for OptionMenu {}

#[derive(Default)]
pub struct DriveBox(std::option::Option<NonNull<ObjectPtr>>);
impl DriveBox {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_drive_box_new(parent.as_raw()) }).with_layout(Layout::Fill)
    }
}
impl ObjectExt for DriveBox {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for DriveBox {}
impl FrameExt for DriveBox {}
impl DrawableExt for DriveBox {}
impl WindowExt for DriveBox {}

#[derive(Default)]
pub struct DirBox(std::option::Option<NonNull<ObjectPtr>>);
impl DirBox {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_dir_box_new(parent.as_raw()) }).with_layout(Layout::Fill)
    }
}
impl ObjectExt for DirBox {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for DirBox {}
impl FrameExt for DirBox {}
impl DrawableExt for DirBox {}
impl WindowExt for DirBox {}

#[derive(Default)]
pub struct DirList(std::option::Option<NonNull<ObjectPtr>>);
impl DirList {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_dir_list_new(parent.as_raw()) }).with_layout(Layout::Fill)
    }
}
impl ObjectExt for DirList {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for DirList {}
impl FrameExt for DirList {}
impl DrawableExt for DirList {}
impl WindowExt for DirList {}
impl CompositeExt for DirList {}

#[derive(Default)]
pub struct DirSelector(std::option::Option<NonNull<ObjectPtr>>);
impl DirSelector {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_dir_selector_new(parent.as_raw()) }).with_layout(Layout::Fill)
    }
}
impl ObjectExt for DirSelector {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for DirSelector {}
impl FrameExt for DirSelector {}
impl DrawableExt for DirSelector {}
impl WindowExt for DirSelector {}
impl CompositeExt for DirSelector {}
impl PackerExt for DirSelector {}

#[derive(Default)]
pub struct FileSelector(std::option::Option<NonNull<ObjectPtr>>);
impl FileSelector {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_file_selector_new(parent.as_raw()) }).with_layout(Layout::Fill)
    }
}
impl ObjectExt for FileSelector {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for FileSelector {}
impl FrameExt for FileSelector {}
impl DrawableExt for FileSelector {}
impl WindowExt for FileSelector {}
impl CompositeExt for FileSelector {}
impl PackerExt for FileSelector {}

#[derive(Default)]
pub struct FileList(std::option::Option<NonNull<ObjectPtr>>);
impl FileList {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_file_list_new(parent.as_raw()) }).with_layout(Layout::Fill)
    }
}
impl ObjectExt for FileList {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for FileList {}
impl FrameExt for FileList {}
impl DrawableExt for FileList {}
impl WindowExt for FileList {}
impl CompositeExt for FileList {}

#[derive(Default)]
pub struct TreeListBox(std::option::Option<NonNull<ObjectPtr>>);
impl TreeListBox {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_tree_list_box_new(parent.as_raw()) }).with_layout(Layout::Fill)
    }
}
impl ObjectExt for TreeListBox {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for TreeListBox {}
impl FrameExt for TreeListBox {}
impl DrawableExt for TreeListBox {}
impl WindowExt for TreeListBox {}
impl CompositeExt for TreeListBox {}
impl PackerExt for TreeListBox {}

#[derive(Default)]
pub struct FontSelector(std::option::Option<NonNull<ObjectPtr>>);
impl FontSelector {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_font_selector_new(parent.as_raw()) }).with_layout(Layout::Fill)
    }
}
impl ObjectExt for FontSelector {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for FontSelector {}
impl FrameExt for FontSelector {}
impl DrawableExt for FontSelector {}
impl WindowExt for FontSelector {}
impl CompositeExt for FontSelector {}
impl PackerExt for FontSelector {}

#[derive(Default)]
pub struct ColorSelector(std::option::Option<NonNull<ObjectPtr>>);
impl ColorSelector {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_color_selector_new(parent.as_raw()) }).with_layout(Layout::Fill)
    }
}
impl ObjectExt for ColorSelector {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for ColorSelector {}
impl FrameExt for ColorSelector {}
impl DrawableExt for ColorSelector {}
impl WindowExt for ColorSelector {}
impl CompositeExt for ColorSelector {}
impl PackerExt for ColorSelector {}

pub struct TreeList(Option<NonNull<ObjectPtr>>);
impl TreeList {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(fx_tree_list_new(parent.as_raw())) }
    }
}

impl ObjectExt for TreeList {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}

impl IdExt for TreeList {}
impl FrameExt for TreeList {}
impl WindowExt for TreeList {}
impl DrawableExt for TreeList {}
impl CompositeExt for TreeList {}
impl TreeListExt for TreeList {}

pub struct TreeItem(Option<NonNull<ObjectPtr>>);

impl ObjectExt for TreeItem {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}

#[derive(Default)]
pub struct MainWindow(Option<NonNull<ObjectPtr>>);
impl MainWindow {
    pub fn new(app: &impl AppExt, title_: &str, w: i32, h: i32) -> Self {
        Self::from_raw(unsafe {
            fx_main_window_new(app.as_raw(), CString::new(title_).unwrap().as_ptr(), w, h)
        })
        .with_pad(0)
    }
    pub fn show(&self) {
        unsafe {
            fx_main_window_show(self.as_raw());
        }
    }
}
impl ObjectExt for MainWindow {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}

impl IdExt for MainWindow {}
impl FrameExt for MainWindow {}
impl WindowExt for MainWindow {}
impl DrawableExt for MainWindow {}
impl CompositeExt for MainWindow {}
impl MainWindowExt for MainWindow {}

pub struct MenuBar(Option<NonNull<ObjectPtr>>);
impl MenuBar {
    pub fn new(parent: &impl WindowExt) -> Self {
        Self::from_raw(unsafe { fx_menu_bar_new(parent.as_raw()) })
    }
}

impl MenuPane {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(foxtk_sys::fx_menu_pane_new(parent.as_raw())) }
    }
}

impl ObjectExt for MenuBar {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for MenuBar {}
impl IdExt for MenuBar {}
impl FrameExt for MenuBar {}
impl DrawableExt for MenuBar {}
impl WindowExt for MenuBar {}

pub struct MenuPane(Option<NonNull<ObjectPtr>>);

impl ObjectExt for MenuPane {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl CompositeExt for MenuPane {}
impl IdExt for MenuPane {}
impl WindowExt for MenuPane {}
impl DrawableExt for MenuPane {}
impl FrameExt for MenuPane {}

pub struct MenuButton(Option<NonNull<ObjectPtr>>);
impl MenuButton {
    pub fn new(prt: &impl WindowExt, text_: &str, pane: &MenuPane) -> Self {
        Self::from_raw(unsafe {
            foxtk_sys::fx_menu_button_new(
                prt.as_raw(),
                CString::new(text_).unwrap().as_ptr(),
                pane.as_raw(),
            )
        })
        .with_layout(Layout::FillX)
    }
}
impl ObjectExt for MenuButton {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for MenuButton {}
impl FrameExt for MenuButton {}
impl DrawableExt for MenuButton {}
impl WindowExt for MenuButton {}

pub struct MenuTitle(Option<NonNull<ObjectPtr>>);

impl MenuTitle {
    pub fn new(prt: &impl WindowExt, text_: &str, pane: &MenuPane) -> Self {
        Self::from_raw(unsafe {
            foxtk_sys::fx_menu_title_new(
                prt.as_raw(),
                CString::new(text_).unwrap().as_ptr(),
                pane.as_raw(),
            )
        })
        .with_layout(Layout::FillX)
    }
}

impl ObjectExt for MenuTitle {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}
impl IdExt for MenuTitle {}
impl FrameExt for MenuTitle {}
impl DrawableExt for MenuTitle {}
impl WindowExt for MenuTitle {}

pub struct MenuRadio(Option<NonNull<ObjectPtr>>);

impl MenuRadio {
    pub fn new(parent: &impl WindowExt, text: &str) -> Self {
        Self::from_raw(unsafe {
            fx_menu_radio_new(parent.as_raw(), CString::new(text).unwrap().as_ptr())
        })
    }
}

impl ObjectExt for MenuRadio {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}

impl IdExt for MenuRadio {}
impl FrameExt for MenuRadio {}
impl DrawableExt for MenuRadio {}
impl WindowExt for MenuRadio {}

pub struct MenuCheck(Option<NonNull<ObjectPtr>>);

impl MenuCheck {
    pub fn new(parent: &impl WindowExt, text: &str) -> Self {
        Self::from_raw(unsafe {
            fx_menu_check_new(parent.as_raw(), CString::new(text).unwrap().as_ptr())
        })
    }
}

impl ObjectExt for MenuCheck {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}

impl IdExt for MenuCheck {}
impl FrameExt for MenuCheck {}
impl DrawableExt for MenuCheck {}
impl WindowExt for MenuCheck {}

pub struct MenuCommand(Option<NonNull<ObjectPtr>>);

impl MenuCommand {
    pub fn new(parent: &impl WindowExt, text: &str) -> Self {
        Self::from_raw(unsafe {
            fx_menu_command_new(parent.as_raw(), CString::new(text).unwrap().as_ptr())
        })
    }
}

impl ObjectExt for MenuCommand {
    fn as_raw(&self) -> *mut ObjectPtr {
        self.0.expect("Empty ObjectPtr!").as_ptr()
    }
    fn from_raw(ptr: *mut ObjectPtr) -> Self {
        Self(NonNull::new(ptr))
    }
}

impl IdExt for MenuCommand {}
impl FrameExt for MenuCommand {}
impl DrawableExt for MenuCommand {}
impl WindowExt for MenuCommand {}
