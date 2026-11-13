use crate::prelude::*;
use crate::{cstr_lossy, to_cstring};
use foxtk_sys::*;
use std::ptr::NonNull;

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
        unsafe { Self::from_raw(FXComboBox_new(parent.as_raw() as *mut FXComposite, cols)) }
            .with_layout(Layout::FillX)
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
        unsafe { Self::from_raw(FXListBox_new(parent.as_raw() as *mut FXComposite)) }
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
        unsafe { Self::from_raw(FXList_new(parent.as_raw() as *mut FXComposite)) }
            .with_layout(Layout::Fill)
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
            FXTable_set_item_text(self.as_raw(), row, col, to_cstring(text).as_ptr());
        }
    }

    pub fn item_text(&self, row: i32, col: i32) -> String {
        unsafe {
            let ptr = FXTable_get_item_text(self.as_raw(), row, col);
            cstr_lossy(ptr)
        }
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
    pub fn add_item_first<'tree>(
        &'tree self,
        prt: &TreeItem<'tree>,
        text: &str,
    ) -> TreeItem<'tree> {
        unsafe {
            TreeItem::from_raw(FXTreeList_append_item(
                self.as_raw(),
                prt.as_raw(),
                to_cstring(text).as_ptr(),
            ))
        }
    }
    pub fn clear_items(&mut self) {
        unsafe {
            FXTreeList_clear_items(self.as_raw());
        }
    }
}

pub struct TreeItem<'tree>(
    std::option::Option<NonNull<FXTreeItem>>,
    std::marker::PhantomData<&'tree TreeList>,
);

impl ObjectExt for TreeItem<'_> {
    type T = FXTreeItem;
    fn as_raw(&self) -> *mut Self::T {
        self.0.expect("Empty FXObject!").as_ptr()
    }
    unsafe fn from_raw(ptr: *mut Self::T) -> Self {
        Self(NonNull::new(ptr), std::marker::PhantomData)
    }
}

impl_widget!(
    DriveBox,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl DriveBox {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXDriveBox_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    DirBox,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl DirBox {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXDirBox_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    FileSelector,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl FileSelector {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXFileSelector_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    TreeListBox,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl TreeListBox {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXTreeListBox_new(parent.as_raw() as *mut FXComposite)) }
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
        unsafe { Self::from_raw(FXFoldingList_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(Header, IdExt, FrameExt, DrawableExt, WindowExt);
impl Header {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXHeader_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(TableItem, IdExt);
impl TableItem {
    pub fn new(table: &Table, text: &str) -> Self {
        unsafe { Self::from_raw(FXTableItem_new(table.as_raw(), to_cstring(text).as_ptr())) }
    }
}

impl_widget!(
    DirList,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt
);
impl DirList {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXDirList_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    DirSelector,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl DirSelector {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXDirSelector_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    FileList,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl FileList {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXFileList_new(parent.as_raw() as *mut FXComposite)) }
    }
}

impl_widget!(
    IconList,
    IdExt,
    FrameExt,
    DrawableExt,
    WindowExt,
    CompositeExt,
    PackerExt
);
impl IconList {
    pub fn new(parent: &impl CompositeExt) -> Self {
        unsafe { Self::from_raw(FXIconList_new(parent.as_raw() as *mut FXComposite)) }
    }
    pub fn num_items(&self) -> i32 {
        unsafe { FXIconList_get_num_items(self.as_raw()) }
    }
    pub fn current_item(&self) -> i32 {
        unsafe { FXIconList_get_current_item(self.as_raw()) }
    }
    pub fn set_current_item(&self, index: i32) {
        unsafe { FXIconList_set_current_item(self.as_raw(), index) }
    }
    pub fn item_text(&self, index: i32) -> String {
        unsafe {
            let ptr = FXIconList_get_item_text(self.as_raw(), index);
            cstr_lossy(ptr)
        }
    }
    pub fn set_item_text(&self, index: i32, text: &str) {
        unsafe { FXIconList_set_item_text(self.as_raw(), index, to_cstring(text).as_ptr()) }
    }
    pub fn append_item(&self, text: &str) {
        unsafe { FXIconList_append_item(self.as_raw(), to_cstring(text).as_ptr()) }
    }
    pub fn clear_items(&self) {
        unsafe { FXIconList_clear_items(self.as_raw()) }
    }
}
