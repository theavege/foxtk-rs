# FoxTK-rs

![FOX-toolkit](http://www.fox-toolkit.org/art/foxlogo_small.jpg)

Rust bindings for the [fox-toolkit](http://www.fox-toolkit.org).

## What is [FOX](http://www.fox-toolkit.org)?

FOX is a C++ based Toolkit for developing Graphical User Interfaces easily and effectively. It offers a wide, and growing, collection of Controls, and provides state of the art facilities such as drag and drop, selection, as well as OpenGL widgets for 3D graphical manipulation. FOX also implements icons, images, and user-convenience features such as status line help, and tooltips. Tooltips may even be used for 3D objects!

Considerable importance has been placed on making FOX one of the fastest toolkits around, and to minimize memory use:- FOX uses a number of techniques to speed up drawing and spatial layout of the GUI. Memory is conserved by allowing programmers to create and destroy GUI elements on the fly.

Even though FOX offers a large collection of Controls already, FOX leverages C++ to allow programmers to easily build additional Controls and GUI elements, simply by taking existing controls, and creating a derived class which simply adds or redefines the desired behavior.

One of the prime design goals of FOX is the ease of programming; thus, most controls can be created using a single line of C++ code; most parameters have sensible default values, so that they may be omitted, and layout managers ensure that designers of GUI's do not have to worry about precise alignments.

Another nice feature of FOX which significantly reduces the number of lines of code which have to be written is FOX's ability to have widgets connect to each other, and passing certain commands between them; for example, a menu entry Hide Toolbar can be directly connected to the Toolbar, and cause it to hide.

Finally, FOX makes it easy to maintain the state of the GUI in an application by having the GUI elements automatically updating themselves by interrogating the application's state. This feature eliminates the large amount of effort that may go into sensitizing, graying out, checking/unchecking etc. depending on the application state.

## Dependencies

- [Linux](.github/workflows/make.sh)
- [Windows](.github/workflows/make.ps1)

## [Other](https://rubydoc.info/gems/fxruby/frames) bindings for [fox-toolkit](http://www.fox-toolkit.org)

## Software using [fox-toolkit](http://www.fox-toolkit.org)

- [Simulation of Urban MObility](https://github.com/eclipse-sumo/sumo)
- [X File Explorer](https://github.com/roland65/xfe)
- [ReZound](https://sourceforge.net/projects/rezound)
- [FOX Calculator](http://fox-toolkit.org/calc.html)
- [Adie](http://fox-toolkit.org/adie.html)

## Alternatives

- [FLTK-rs](https://github.com/fltk-rs)
- [GTK-rs](https://github.com/gtk-rs)
- [RSTK](https://codeberg.org/peterlane/rstk)
- [EFL-rs](https://codeberg.org/JustSoup321/efl-rs)

## Work in process

- [x] [FXApp](docs/fx_app.md)
- [x] [Containers](docs/fx_composite.md
  - [x] [FXGroupBox](docs/fx_composite.md#FXGroupBox)
  - [x] [FXHorizontalFrame](docs/fx_composite.md#FXHorizontalFrame)
  - [x] [FXVerticalFrame](docs/fx_composite.md#FXVerticalFrame)
  - [x] [FXSwitcher](docs/fx_composite.md#FXSwitcher)
- [x] Widgets
  - [x] [Selectors](docs/fx_selectors.md)
    - [x] [FXComboBox](docs/fx_selectors.md#FXComboBox)
    - [x] [FXListBox](docs/fx_selectors.md#FXListBox)
    - [x] [FXList](docs/fx_selectors.md#FXList)
  - [x] [Inputs](docs/fx_inputs.md)
    - [x] [FXTextField](docs/fx_inputs.md#FXTextField)
    - [x] [FXRadioButton](docs/fx_inputs.md#FXRadioButton)
    - [x] [FXCheckButton](docs/fx_inputs.md#FXCheckButton)
  - [x] [Outputs](docs/fx_outputs.md)
    - [x] [FXProgressBar](docs/fx_outputs.md#FXProgressBar)
    - [x] [FXLabel](docs/fx_outputs.md#FXLabel)
  - [x] [Triggers](docs/fx_triggers.md)
    - [x] [FXButton](docs/fx_triggers.md#FXButton)
