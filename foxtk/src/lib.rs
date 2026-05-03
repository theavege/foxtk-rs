pub mod prelude;
use {foxtk_sys::*, prelude::*, std::ffi::CString};

pub struct App(ObjectPtr);
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
//~ impl Drop for App {
//~ fn drop(&mut self) {
//~ unsafe { fx_object_delete(self.0) }
//~ }
//~ }
impl ObjectExt for App {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl AppExt for App {}

#[derive(Default)]
pub struct Button(ObjectPtr);
impl ObjectExt for Button {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl IdExt for Button {}
impl WindowExt for Button {}
impl LabelExt for Button {}
impl ButtonExt for Button {}

pub struct Canvas(ObjectPtr);
impl Canvas {
    pub fn new(parent: &impl WindowExt) -> Self {
        Self::from_raw(unsafe { fx_canvas_new(parent.as_raw()) })
    }
}
impl ObjectExt for Canvas {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl WindowExt for Canvas {}
impl IdExt for Canvas {}

#[derive(Default)]
pub struct CheckButton(ObjectPtr);
impl CheckButton {
    pub fn new(parent: &impl ObjectExt, title_: &str) -> Self {
        let title = CString::new(title_).unwrap();
        Self::from_raw(unsafe { fx_check_button_new(parent.as_raw(), title.as_ptr()) })
    }
}
impl ObjectExt for CheckButton {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl IdExt for CheckButton {}
impl WindowExt for CheckButton {}
impl LabelExt for CheckButton {}
impl CheckButtonExt for CheckButton {}

#[derive(Default)]
pub struct ComboBox(ObjectPtr);
impl ComboBox {
    pub fn new(parent: &impl WindowExt, cols: i32) -> Self {
        unsafe { Self::from_raw(fx_combo_box_new(parent.as_raw(), cols)) }
    }
}
impl ObjectExt for ComboBox {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl IdExt for ComboBox {}
impl WindowExt for ComboBox {}
impl PackerExt for ComboBox {}
impl CompositeExt for ComboBox {}

pub struct GroupBox(ObjectPtr);
impl GroupBox {
    pub fn new(parent: &impl ObjectExt, title_: &str) -> Self {
        let title = CString::new(title_).unwrap();
        Self::from_raw(unsafe { fx_groupbox_new(parent.as_raw(), title.as_ptr()) })
    }
}
impl ObjectExt for GroupBox {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl CompositeExt for GroupBox {}
impl IdExt for GroupBox {}
impl WindowExt for GroupBox {}

pub struct Spring(ObjectPtr);
impl Spring {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_spring_new(parent.as_raw()) })
    }
}
impl ObjectExt for Spring {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl CompositeExt for Spring {}
impl PackerExt for Spring {}
impl IdExt for Spring {}
impl WindowExt for Spring {}

pub struct VerticalFrame(ObjectPtr);
impl VerticalFrame {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_vertical_frame_new(parent.as_raw()) })
    }
}
impl ObjectExt for VerticalFrame {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl CompositeExt for VerticalFrame {}
impl IdExt for VerticalFrame {}
impl WindowExt for VerticalFrame {}

pub struct HorizontalFrame(ObjectPtr);
impl HorizontalFrame {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_horizontal_frame_new(parent.as_raw()) })
    }
}
impl ObjectExt for HorizontalFrame {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl CompositeExt for HorizontalFrame {}
impl IdExt for HorizontalFrame {}
impl WindowExt for HorizontalFrame {}

#[derive(Default)]
pub struct Switcher(ObjectPtr);
impl Switcher {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_switcher_new(parent.as_raw()) })
    }
}
impl ObjectExt for Switcher {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl CompositeExt for Switcher {}
impl SwitcherExt for Switcher {}
impl IdExt for Switcher {}
impl WindowExt for Switcher {}
impl PackerExt for Switcher {}

#[derive(Default)]
pub struct Label(ObjectPtr);
impl Label {
    pub fn new(parent: &impl ObjectExt, title_: &str) -> Self {
        let title = CString::new(format!("&{title_}").as_str()).unwrap();
        Self::from_raw(unsafe { fx_label_new(parent.as_raw(), title.as_ptr()) })
    }
}
impl ObjectExt for Label {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl WindowExt for Label {}
impl IdExt for Label {}
impl LabelExt for Label {}

#[derive(Default)]
pub struct ListBox(ObjectPtr);
impl ListBox {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(fx_list_box_new(parent.as_raw())) }
    }
}
impl ObjectExt for ListBox {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl IdExt for ListBox {}
impl WindowExt for ListBox {}
impl CompositeExt for ListBox {}
impl PackerExt for ListBox {}

#[derive(Default)]
pub struct List(ObjectPtr);
impl List {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(fx_list_new(parent.as_raw())) }
    }
}
impl ObjectExt for List {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl IdExt for List {}
impl WindowExt for List {}
impl PackerExt for List {}
impl CompositeExt for List {}

#[derive(Default)]
pub struct ProgressBar(ObjectPtr);
impl ProgressBar {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_progressbar_new(parent.as_raw()) })
    }
}

impl ObjectExt for ProgressBar {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl IdExt for ProgressBar {}
impl WindowExt for ProgressBar {}
impl ProgressBarExt for ProgressBar {}

#[derive(Default)]
pub struct RadioButton(ObjectPtr);
impl RadioButton {
    pub fn new(parent: &impl ObjectExt, title_: &str) -> Self {
        let title = CString::new(title_).unwrap();
        Self::from_raw(unsafe { fx_radio_button_new(parent.as_raw(), title.as_ptr()) })
    }
}
impl ObjectExt for RadioButton {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl IdExt for RadioButton {}
impl WindowExt for RadioButton {}
impl LabelExt for RadioButton {}
impl RadioButtonExt for RadioButton {}

pub struct ScrollBar(ObjectPtr);
impl ScrollBar {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(fx_scroll_bar_new(parent.as_raw())) }
    }
}
impl ObjectExt for ScrollBar {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl WindowExt for ScrollBar {}
impl IdExt for ScrollBar {}
impl ScrollBarExt for ScrollBar {}

#[derive(Default)]
pub struct Slider(ObjectPtr);
impl Slider {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_slider_new(parent.as_raw()) })
    }
}
impl ObjectExt for Slider {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl IdExt for Slider {}
impl WindowExt for Slider {}
impl SliderExt for Slider {}

#[derive(Default)]
pub struct Spinner(ObjectPtr);
impl Spinner {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_spinner_new(parent.as_raw()) })
    }
}
impl ObjectExt for Spinner {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl IdExt for Spinner {}
impl WindowExt for Spinner {}
impl SpinnerExt for Spinner {}

pub struct TabBook(ObjectPtr);
impl TabBook {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(fx_tab_book_new(parent.as_raw())) }
    }
}

impl ObjectExt for TabBook {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for TabBook {}
impl IdExt for TabBook {}

pub struct TabItem(ObjectPtr);
impl TabItem {
    pub fn new(parent: &impl WindowExt, text: &str) -> Self {
        let c_text = CString::new(text).unwrap();
        unsafe { Self::from_raw(fx_tab_item_new(parent.as_raw(), c_text.as_ptr())) }
    }
}
impl ObjectExt for TabItem {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl WindowExt for TabItem {}
impl IdExt for TabItem {}

pub struct Table(ObjectPtr);
impl Table {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(fx_table_new(parent.as_raw())) }
    }
}

impl TableExt for Table {}
impl ObjectExt for Table {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl WindowExt for Table {}
impl IdExt for Table {}

pub struct Text(ObjectPtr);
impl Text {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(fx_text_new(parent.as_raw())) }
    }
}
impl ObjectExt for Text {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl IdExt for Text {}
impl WindowExt for Text {}
impl TextExt for Text {}

#[derive(Default)]
pub struct TextField(ObjectPtr);
impl TextField {
    pub fn new(parent: &impl ObjectExt) -> Self {
        Self::from_raw(unsafe { fx_textfield_new(parent.as_raw()) })
    }
}

impl ObjectExt for TextField {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl IdExt for TextField {}
impl WindowExt for TextField {}
impl TextFieldExt for TextField {}

pub struct TreeList(ObjectPtr);
impl TreeList {
    pub fn new(parent: &impl WindowExt) -> Self {
        unsafe { Self::from_raw(fx_tree_list_new(parent.as_raw())) }
    }
}
impl TreeListExt for TreeList {}

impl ObjectExt for TreeList {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl IdExt for TreeList {}
impl WindowExt for TreeList {}

pub struct TreeItem(ObjectPtr);

impl ObjectExt for TreeItem {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

pub struct MainWindow(ObjectPtr);
impl MainWindow {
    pub fn new(app: &impl AppExt, title_: &str, w: i32, h: i32) -> Self {
        let title = CString::new(title_).unwrap();
        Self::from_raw(unsafe { fx_main_window_new(app.as_raw(), title.as_ptr(), w, h) })
    }
}
impl ObjectExt for MainWindow {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl IdExt for MainWindow {}
impl WindowExt for MainWindow {}
impl CompositeExt for MainWindow {}
impl MainWindowExt for MainWindow {}

pub struct MenuBar(ObjectPtr);
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

impl MenuTitle {
    pub fn new(prt: &impl WindowExt, text_: &str, pane: &MenuPane) -> Self {
        unsafe {
            Self::from_raw(foxtk_sys::fx_menu_title_new(
                prt.as_raw(),
                CString::new(text_).unwrap().as_ptr(),
                pane.as_raw(),
            ))
        }
    }
}
impl ObjectExt for MenuBar {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl CompositeExt for MenuBar {}
impl WindowExt for MenuBar {}
impl IdExt for MenuBar {}

pub struct MenuPane(ObjectPtr);

impl ObjectExt for MenuPane {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }
    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl CompositeExt for MenuPane {}
impl WindowExt for MenuPane {}
impl IdExt for MenuPane {}

pub struct MenuTitle(ObjectPtr);

impl ObjectExt for MenuTitle {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for MenuTitle {}
impl IdExt for MenuTitle {}

pub struct MenuCommand(ObjectPtr);

impl MenuCommand {
    pub fn new(parent: &impl WindowExt, text_: &str) -> Self {
        let text = CString::new(text_).unwrap();
        Self::from_raw(unsafe { fx_menu_command_new(parent.as_raw(), text.as_ptr()) })
    }
}

impl ObjectExt for MenuCommand {
    fn as_raw(&self) -> ObjectPtr {
        self.0
    }

    fn from_raw(ptr: ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl WindowExt for MenuCommand {}
impl IdExt for MenuCommand {}
