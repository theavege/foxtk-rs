# FoxTK-rs

![FOX-toolkit](assets/foxlogo_small.jpg)

Work in progress Rust bindings for the [fox-toolkit](http://www.fox-toolkit.org).

cfoxtk.cpp -> cfoxtk.h -[foxtk-sys]-> foxtk.prelude -> foxtk.lib -> example


## What is FOX?

FOX is a C++ based Toolkit for developing Graphical User Interfaces easily and effectively. It offers a wide, and growing, collection of Controls, and provides state of the art facilities such as drag and drop, selection, as well as OpenGL widgets for 3D graphical manipulation. FOX also implements icons, images, and user-convenience features such as status line help, and tooltips. Tooltips may even be used for 3D objects!

Considerable importance has been placed on making FOX one of the fastest toolkits around, and to minimize memory use:- FOX uses a number of techniques to speed up drawing and spatial layout of the GUI. Memory is conserved by allowing programmers to create and destroy GUI elements on the fly.

Even though FOX offers a large collection of Controls already, FOX leverages C++ to allow programmers to easily build additional Controls and GUI elements, simply by taking existing controls, and creating a derived class which simply adds or redefines the desired behavior.

One of the prime design goals of FOX is the ease of programming; thus, most controls can be created using a single line of C++ code; most parameters have sensible default values, so that they may be omitted, and layout managers ensure that designers of GUI's do not have to worry about precise alignments.

Another nice feature of FOX which significantly reduces the number of lines of code which have to be written is FOX's ability to have widgets connect to each other, and passing certain commands between them; for example, a menu entry Hide Toolbar can be directly connected to the Toolbar, and cause it to hide.

Finally, FOX makes it easy to maintain the state of the GUI in an application by having the GUI elements automatically updating themselves by interrogating the application's state. This feature eliminates the large amount of effort that may go into sensitizing, graying out, checking/unchecking etc. depending on the application state.

## [Dependencies](.github/workflows/make.sh)

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

- [ ] [FXObject](http://fox-toolkit.org/ref/classFX_1_1FXObject.html#details)
   - [x] [FXApp](docs/FXApp.md)
   - [ ] [FXId](http://fox-toolkit.org/ref/classFX_1_1FXId.html#details)
    - [ ] [FXDrawable](http://fox-toolkit.org/ref/classFX_1_1FXDrawable.html#details)
      - [ ] [FXFont](http://fox-toolkit.org/ref/classFX_1_1FXFont.html#details)
      - [x] [FXWindow](docs/FXWindow.md)
        - [ ] [FXComposite](http://fox-toolkit.org/ref/classFX_1_1FXComposite.html#details)
          - [x] [FXPacker](docs/FXPacker.md)
            - [x] [FXHorizontalFrame](docs/FXHorizontalFrame.md)
            - [x] [FXVerticalFrame](docs/FXVerticalFrame.md)
            - [x] [FXComboBox](docs/FXComboBox.md)
            - [x] [FXListBox](docs/FXListBox.md)
            - [x] [FXSwitcher](docs/FXSwitcher.md)
        - [ ] [FXFrame](http://fox-toolkit.org/ref/classFX_1_1FXFrame.html#details)
          - [ ] [FXTextField](http://fox-toolkit.org/ref/classFX_1_1FXTextField.html#details)
          - [ ] [FXProgressBar](http://fox-toolkit.org/ref/classFX_1_1FXProgressBar.html#details)
          - [ ] [FXLabel](http://fox-toolkit.org/ref/classFX_1_1FXLabel.html#details)
            - [x] [FXButton](docs/FXButton.md)
            - [ ] [FXRadioButton](http://fox-toolkit.org/ref/classFX_1_1FXRadioButton.html#details)
            - [ ] [FXCheckButton](http://fox-toolkit.org/ref/classFX_1_1FXCheckButton.html#details)
