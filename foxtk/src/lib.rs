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

pub struct MainWindow(Option<NonNull<ObjectPtr>>);
impl MainWindow {
    pub fn new(app: &impl AppExt, title_: &str, w: i32, h: i32) -> Self {
        Self::from_raw(unsafe {
            fx_main_window_new(app.as_raw(), CString::new(title_).unwrap().as_ptr(), w, h)
        })
        .with_pad(0)
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
