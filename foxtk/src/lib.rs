pub mod prelude;

pub struct App(foxtk_sys::ObjectPtr);
impl prelude::ObjectExt for App {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::AppExt for App {}

pub struct Button(foxtk_sys::ObjectPtr);
impl prelude::ObjectExt for Button {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for Button {}
impl prelude::WindowExt for Button {}
impl prelude::LabelExt for Button {}
impl prelude::ButtonExt for Button {}

pub struct Canvas(foxtk_sys::ObjectPtr);
impl prelude::ObjectExt for Canvas {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::CanvasExt for Canvas {}
impl prelude::WindowExt for Canvas {}
impl prelude::IdExt for Canvas {}

#[derive(Default, Clone)]
pub struct CheckButton(foxtk_sys::ObjectPtr);
impl prelude::ObjectExt for CheckButton {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for CheckButton {}
impl prelude::WindowExt for CheckButton {}
impl prelude::LabelExt for CheckButton {}
impl prelude::CheckButtonExt for CheckButton {}

pub struct ComboBox(foxtk_sys::ObjectPtr);
impl prelude::ObjectExt for ComboBox {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for ComboBox {}
impl prelude::WindowExt for ComboBox {}
impl prelude::ComboBoxExt for ComboBox {}

pub struct VerticalFrame(foxtk_sys::ObjectPtr);
impl prelude::ObjectExt for VerticalFrame {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for VerticalFrame {}
impl prelude::WindowExt for VerticalFrame {}
impl prelude::VerticalFrameExt for VerticalFrame {}

pub struct HorizontalFrame(foxtk_sys::ObjectPtr);
impl prelude::ObjectExt for HorizontalFrame {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for HorizontalFrame {}
impl prelude::WindowExt for HorizontalFrame {}
impl prelude::VerticalFrameExt for HorizontalFrame {}

pub struct Label(foxtk_sys::ObjectPtr);
impl prelude::ObjectExt for Label {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::WindowExt for Label {}
impl prelude::IdExt for Label {}
impl prelude::LabelExt for Label {}

pub struct ListBox(foxtk_sys::ObjectPtr);
impl prelude::ObjectExt for ListBox {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for ListBox {}
impl prelude::WindowExt for ListBox {}
impl prelude::ListBoxExt for ListBox {}

pub struct ProgressBar(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for ProgressBar {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for ProgressBar {}
impl prelude::WindowExt for ProgressBar {}
impl prelude::ProgressBarExt for ProgressBar {}

#[derive(Default)]
pub struct RadioButton(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for RadioButton {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for RadioButton {}
impl prelude::WindowExt for RadioButton {}
impl prelude::LabelExt for RadioButton {}
impl prelude::RadioButtonExt for RadioButton {}

pub struct ScrollBar(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for ScrollBar {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::WindowExt for ScrollBar {}
impl prelude::IdExt for ScrollBar {}
impl prelude::ScrollBarExt for ScrollBar {}

#[derive(Default, Clone)]
pub struct RangeSlider(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for RangeSlider {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for RangeSlider {}
impl prelude::WindowExt for RangeSlider {}
impl prelude::RangeSliderExt for RangeSlider {}

#[derive(Default, Clone)]
pub struct Spinner(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for Spinner {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for Spinner {}
impl prelude::WindowExt for Spinner {}
impl prelude::SpinnerExt for Spinner {}

pub struct TabBook(foxtk_sys::ObjectPtr);

impl prelude::TabBookExt for TabBook {}

impl prelude::ObjectExt for TabBook {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl prelude::WindowExt for TabBook {}
impl prelude::IdExt for TabBook {}

pub struct TabItem(foxtk_sys::ObjectPtr);
impl prelude::TabItemExt for TabItem {}
impl prelude::ObjectExt for TabItem {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::WindowExt for TabItem {}
impl prelude::IdExt for TabItem {}

pub struct Table(foxtk_sys::ObjectPtr);

impl prelude::TableExt for Table {}
impl prelude::ObjectExt for Table {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::WindowExt for Table {}
impl prelude::IdExt for Table {}

pub struct Text(foxtk_sys::ObjectPtr);
impl prelude::ObjectExt for Text {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for Text {}
impl prelude::WindowExt for Text {}
impl prelude::TextExt for Text {}

#[derive(Default, Clone)]
pub struct TextField(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for TextField {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::IdExt for TextField {}
impl prelude::WindowExt for TextField {}
impl prelude::TextFieldExt for TextField {}

pub struct TreeList(foxtk_sys::ObjectPtr);

impl prelude::TreeListExt for TreeList {}

impl prelude::ObjectExt for TreeList {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl prelude::IdExt for TreeList {}

impl prelude::WindowExt for TreeList {}

pub struct TreeItem(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for TreeItem {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

pub struct MainWindow(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for MainWindow {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl prelude::IdExt for MainWindow {}
impl prelude::WindowExt for MainWindow {}
impl prelude::MainWindowExt for MainWindow {}

pub struct MenuBar(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for MenuBar {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}
impl prelude::WindowExt for MenuBar {}
impl prelude::MenuBarExt for MenuBar {}
impl prelude::IdExt for MenuBar {}

pub struct MenuPane(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for MenuPane {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }
    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl prelude::MenuPaneExt for MenuPane {}
impl prelude::WindowExt for MenuPane {}
impl prelude::IdExt for MenuPane {}

pub struct MenuTitle(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for MenuTitle {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl prelude::WindowExt for MenuTitle {}
impl prelude::MenuTitleExt for MenuTitle {}
impl prelude::IdExt for MenuTitle {}

pub struct MenuCommand(foxtk_sys::ObjectPtr);

impl prelude::ObjectExt for MenuCommand {
    fn as_raw(&self) -> foxtk_sys::ObjectPtr {
        self.0
    }

    fn from_raw(ptr: foxtk_sys::ObjectPtr) -> Self {
        Self(ptr)
    }
}

impl prelude::MenuCommandExt for MenuCommand {}
impl prelude::WindowExt for MenuCommand {}
impl prelude::IdExt for MenuCommand {}
