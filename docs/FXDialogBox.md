# FXDialogBox

Dialog box widget for creating modal and modeless dialogs.

## Overview

The `FXDialogBox` widget provides a dialog box that can be used to create custom dialogs. It can be modal (blocks input to other windows) or modeless (allows input to other windows).

## Usage

```rust
use foxtk::prelude::*;

let dialog = FXDialogBox::new(&window, "My Dialog");
dialog.show();
```

## Methods

### `new(parent: &impl WindowExt, title: &str) -> Self`
Creates a new dialog box widget.

**Parameters:**
- `parent`: The parent window
- `title`: The title of the dialog box

**Returns:**
A new `FXDialogBox` instance

### `show()`
Shows the dialog box.

### `hide()`
Hides the dialog box.

### `shown() -> bool`
Checks if the dialog box is currently shown.

**Returns:**
`true` if the dialog is shown, `false` otherwise

### `with_style(style: DialogBoxStyle) -> Self`
Sets the dialog box style.

**Parameters:**
- `style`: The style to apply

**Returns:**
Self for method chaining

## Styles

The `DialogBoxStyle` enum provides the following options:

- `Normal` - Default dialog box
- `Modal` - Modal dialog (blocks input to other windows)
- `Resizable` - Dialog can be resized
- `Minimize` - Dialog has a minimize button
- `Maximize` - Dialog has a maximize button

## Traits

`FXDialogBox` implements the following traits:
- `ObjectExt`
- `IdExt`
- `FrameExt`
- `DrawableExt`
- `WindowExt`
- `TopWindowExt`

## Example

```rust
use foxtk::prelude::*;

let app = App::new("MyApp", "Vendor");
let window = MainWindow::new(&app, "Main Window", 800, 600);

let dialog = FXDialogBox::new(&window, "My Dialog")
    .with_style(DialogBoxStyle::Modal | DialogBoxStyle::Resizable);

dialog.show();
```

## See Also

- [FXFileDialog](FXFileDialog.md) - File open/save dialogs
- [FXMessageBox](FXMessageBox.md) - Predefined message boxes
- [FXTopWindow](FXTopWindow.md) - Top-level window
