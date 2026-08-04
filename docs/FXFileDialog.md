# FXFileDialog

File dialog widget for opening and saving files.

## Overview

The `FXFileDialog` widget provides a dialog for selecting files to open or save. It includes features like file filtering, directory navigation, and file preview.

## Usage

```rust
use foxtk::prelude::*;

let filedialog = FXFileDialog::new(&window, "Open File");
filedialog.set_pattern("*.txt;*.rs");
filedialog.show();
```

## Methods

### `new(parent: &impl WindowExt, title: &str) -> Self`
Creates a new file dialog widget.

**Parameters:**
- `parent`: The parent window
- `title`: The title of the file dialog

**Returns:**
A new `FXFileDialog` instance

### `show()`
Shows the file dialog.

### `set_directory(directory: &str)`
Sets the current directory.

**Parameters:**
- `directory`: The directory path

### `directory() -> String`
Gets the current directory.

**Returns:**
The current directory path

### `set_filename(filename: &str)`
Sets the current filename.

**Parameters:**
- `filename`: The filename

### `filename() -> String`
Gets the current filename.

**Returns:**
The current filename

### `set_pattern(pattern: &str)`
Sets the file pattern filter (e.g., "*.txt;*.rs").

**Parameters:**
- `pattern`: The file pattern

### `pattern() -> String`
Gets the current file pattern.

**Returns:**
The current file pattern

## Convenience Methods

The `WindowExt` trait provides convenience methods for showing file dialogs without creating a `FXFileDialog` instance:

### `open_file_dialog(caption: &str, path: &str, patterns: &str, initial: i32) -> String`
Shows an open file dialog and returns the selected filename.

**Parameters:**
- `caption`: The dialog caption
- `path`: The initial path
- `patterns`: The file patterns (e.g., "*.txt;*.rs")
- `initial`: Initial filter index

**Returns:**
The selected filename, or empty string if cancelled

### `save_file_dialog(caption: &str, path: &str, patterns: &str, initial: i32) -> String`
Shows a save file dialog and returns the selected filename.

**Parameters:**
- `caption`: The dialog caption
- `path`: The initial path
- `patterns`: The file patterns (e.g., "*.txt;*.rs")
- `initial`: Initial filter index

**Returns:**
The selected filename, or empty string if cancelled

## Traits

`FXFileDialog` implements the following traits:
- `ObjectExt`
- `IdExt`
- `FrameExt`
- `DrawableExt`
- `WindowExt`
- `TopWindowExt`

## Example

```rust
use foxtk::prelude::*;

// Using the convenience method
let filename = window.open_file_dialog("Open File", ".", "*.txt;*.rs", 0);
println!("Selected file: {}", filename);

// Using the widget directly
let filedialog = FXFileDialog::new(&window, "Open File");
filedialog.set_pattern("*.txt;*.rs");
filedialog.set_directory("/home/user");
filedialog.show();
```

## See Also

- [FXDialogBox](FXDialogBox.md) - Custom dialogs
- [FXWindowExt](FXWindowExt.md) - Window extension traits
