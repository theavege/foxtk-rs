#![doc = include_str!("../README.md")]

pub mod enums;
pub mod prelude;

#[macro_use]
mod macros;

mod support;
pub(crate) use support::{
    HEIGHT, cmouse_callback, cstr_lossy, cstring_args, ctimer_callback, drop_callback_context,
    to_cstring,
};

mod app;
pub use app::*;

mod widgets;
pub use widgets::*;

use {foxtk_sys::*, prelude::*};

impl_textable!(Button, Label, Text, TextField, RadioButton);

impl_selector!(ComboBox, List, ListBox);

impl_ranger!(Dial, Knob, Slider, Spinner);

impl_editable!(Text, TextField);

impl_help!(ColorBar, ColorRing, ColorWell, ColorWheel, ShutterItem);

impl_selector!(ColorList);
