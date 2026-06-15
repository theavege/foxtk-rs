# [FXApp](http://fox-toolkit.org/ref/classFX_1_1FXApp.html#details)

The application object manages the message queue, timers, chores, signal handling, GUI updating, and other system facilities. Each FOX application will have exactly one application instance. Every FOX application will start by constructing one FXApp object prior to building the GUI.  Usually, the FXApp object is the last object to be deleted as well.

Using the code below, the application object will be constructed on the stack and hence is automatically destroyed when the program terminates.  Also, when the application object is destroyed, all the windows and other resources it knows about are destroyed as well.

```rust
use foxtk::prelude::*;

fn main() {

    // Make application
    let app = foxtk::App::new("ApplicationName","VendorName");

    // Make MainWindow
    let width = 480;
    let height = 270;
    foxtk::MainWindow::new(&app, "title", width, height);

   // Run
    app.run();
}
```
