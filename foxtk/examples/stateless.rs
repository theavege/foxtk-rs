use foxtk::prelude::*;

fn main() {
    // Make application
    let app = foxtk::App::new("ApplicationName", "VendorName");

    // Make MainWindow
    let (width, height) = (640, 400);
    foxtk::MainWindow::new(&app, "title", width, height).show();

    // Run
    app.run();
}
