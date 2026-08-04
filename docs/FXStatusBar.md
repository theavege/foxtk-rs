# FXStatusBar

Status bar widget for displaying status information and help text.

## Overview

The `FXStatusBar` widget provides a status bar typically displayed at the bottom of a window. It can display status text and help text, which is useful for showing application status, hints, or contextual help.

## Usage

```rust
use foxtk::prelude::*;

let statusbar = FXStatusBar::new(&parent);
statusbar.set_text("Ready");
statusbar.set_help_text("Application is ready");
```

## Methods

### `new(parent: &impl CompositeExt) -> Self`
Creates a new status bar widget.

**Parameters:**
- `parent`: The parent composite widget

**Returns:**
A new `FXStatusBar` instance

### `set_text(text: &str)`
Sets the status text displayed in the status bar.

**Parameters:**
- `text`: The text to display

### `text() -> String`
Gets the current status text.

**Returns:**
The current status text

### `set_help_text(text: &str)`
Sets the help text displayed in the status bar.

**Parameters:**
- `text`: The help text to display

### `help_text() -> String`
Gets the current help text.

**Returns:**
The current help text

## Traits

`FXStatusBar` implements the following traits:
- `ObjectExt`
- `IdExt`
- `FrameExt`
- `DrawableExt`
- `WindowExt`
- `CompositeExt`
- `PackerExt`
- `TextableExt`

## Example

```rust
use foxtk::prelude::*;

#[derive(Default)]
struct MyApp {
    statusbar: foxtk::StatusBar,
}

impl Component for MyApp {
    type Event = Msg;
    type State = i32;

    fn update(&self, model: &Self::State) {
        self.statusbar.set_text(&format!("Counter: {}", model));
    }

    fn view(&mut self, parent: &impl CompositeExt, _sender: Sender<Self::Event>) {
        foxtk::VerticalFrame::new(parent).inside(|prt| {
            // ... other widgets ...
            self.statusbar = foxtk::StatusBar::new(prt);
        });
    }
}
```

## See Also

- [FXStatusLine](FXStatusLine.md) - Single-line status display
- [FXLabel](FXLabel.md) - Simple text display
