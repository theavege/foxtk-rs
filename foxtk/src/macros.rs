macro_rules! impl_widget {
    (@as $name:ident, $raw:ident, $($dep:ident),*) => {
        #[derive(Default)]
        pub struct $name(std::option::Option<std::ptr::NonNull<$raw>>);

        impl ObjectExt for $name {
            type T = $raw;
            fn as_raw(&self) -> *mut Self::T {
                self.0
                    .expect(concat!("Empty ", stringify!($name), "!"))
                    .as_ptr()
            }
            unsafe fn from_raw(obj: *mut Self::T) -> Self {
                Self(std::ptr::NonNull::new(obj))
            }
        }

        $(impl $dep for $name {})*
    };
    ($name:ident, $($dep:ident),*) => {
        paste::paste! {
            impl_widget!(@as $name, [<FX $name>], $($dep),*);
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
                        [<FX $name _append_item>](self.as_raw(), to_cstring(text).as_ptr());
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
                        cstr_lossy(ptr)
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
                        cstr_lossy(ptr)
                    }
                }
                fn set_text(&self, text: &str) {
                    unsafe {
                        [<FX $name _set_text>](self.as_raw(), to_cstring(text).as_ptr());
                    }
                }
                fn set_text_color(&self, color: Color) {
                    unsafe {
                        [<FX $name _set_text_color>](self.as_raw(), color.bits());
                    }
                }
                fn set_font(&self, family: &str, size: i32) {
                    unsafe {
                        [<FX $name _set_font>](self.as_raw(), to_cstring(family).as_ptr(), size);
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

macro_rules! impl_help {
    ($($name:ident),+ $(,)?) => {
        paste::paste! {$ (
            impl $name {
                pub fn help_text(&self) -> String {
                    unsafe {
                        let ptr = [<FX $name _get_help_text>](self.as_raw());
                        cstr_lossy(ptr)
                    }
                }
                pub fn set_help_text(&self, text: &str) {
                    unsafe { [<FX $name _set_help_text>](self.as_raw(), to_cstring(text).as_ptr()) }
                }
                pub fn tip_text(&self) -> String {
                    unsafe {
                        let ptr = [<FX $name _get_tip_text>](self.as_raw());
                        cstr_lossy(ptr)
                    }
                }
                pub fn set_tip_text(&self, text: &str) {
                    unsafe { [<FX $name _set_tip_text>](self.as_raw(), to_cstring(text).as_ptr()) }
                }
            }
        )+}
    };
}
