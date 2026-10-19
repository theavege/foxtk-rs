use foxtk::prelude::TextableExt;
use foxtk::{App, Label, MainWindow};

#[test]
fn label_text_round_trips_through_fox() {
    let app = App::new("foxtk-tests", "foxtk");
    let window = MainWindow::new(&app, "Runtime test", 320, 200);
    let label = Label::new(&window, "before");

    label.set_text("after");

    assert_eq!(label.text(), "after");
}
