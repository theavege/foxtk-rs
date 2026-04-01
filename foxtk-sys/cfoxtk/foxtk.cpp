#include <fx.h>
#include <foxtk.h>

// CALLBACK BRIDGE

class CTarget : public FXObject {
  FXDECLARE(CTarget)
protected:
    CTarget() {}
private:
  CWidgetCb callback = nullptr;
  FXWidgetPtr widget = nullptr;
  void*      context = nullptr;
public:
  enum { Selector = FXMainWindow::ID_LAST, ID_LAST };
  CTarget(CWidgetCb cb, FXWidgetPtr wgt, void* ctx) : callback(cb), widget(wgt) , context(ctx) {}
  long onCommand(FXObject*, FXSelector, void*) {
    long result = 1;
    if (this -> callback) result = this -> callback(this -> widget, this -> context);
    return result;
  }
};

FXDEFMAP(CTarget) CTargetMap[] = {
    FXMAPFUNC(SEL_COMMAND, CTarget::Selector, CTarget::onCommand),
};
FXIMPLEMENT(CTarget, FXObject, CTargetMap, ARRAYNUMBER(CTargetMap))

class CTimeout : public FXObject {
  FXDECLARE(CTimeout)
protected:
    CTimeout() {}
private:
  FXAppPtr application = nullptr;
  CTimerCb    callback = nullptr;
  unsigned int nanosec = 0;
  void*        context = nullptr;
public:
  enum { Selector = FXMainWindow::ID_LAST, ID_LAST };
  CTimeout(FXAppPtr app, CTimerCb cb, unsigned int ns, void* ctx) {
    this -> application = app;
    this -> callback = cb;
    this -> nanosec = ns;
    static_cast<FXApp*>(app)->addTimeout(this, FXMainWindow::ID_LAST, ns, ctx);
  }
  long onTimeout(FXObject*, FXSelector, void* ctx) {
        long result = 1;
        if (this -> callback) {
            result = this -> callback(this -> application, ctx);
            static_cast<FXApp*>(application)->addTimeout(this, FXMainWindow::ID_LAST, nanosec, ctx);
        };
        return result;
    }
};

FXDEFMAP(CTimeout) CTimeoutMap[] = {
    FXMAPFUNC(SEL_TIMEOUT, CTimeout::Selector, CTimeout::onTimeout),
};
FXIMPLEMENT(CTimeout, FXObject, CTimeoutMap, ARRAYNUMBER(CTimeoutMap))

extern "C" {

// APPLICATION
    FXAppPtr fox_app_new(const char* name, const char* vendor) {
        return new FXApp(name, vendor);
    }
    void fox_app_init(FXAppPtr app, int argc, char** argv) {
        static_cast<FXApp*>(app)->init(argc, argv);
    }
    int fox_app_run(FXAppPtr app) {
        auto obj = static_cast<FXApp*>(app);
        obj->create();
        return obj->run();
    }
    CTimeoutPtr fox_app_add_timeout(FXAppPtr app, CTimerCb cb, unsigned int ns, void* ctx) {
        return new CTimeout(app, cb, ns, ctx);
    }

// WINDOW
    FXParentPtr fox_main_window_new(FXAppPtr app, const char* title, int width, int height) {
        auto obj = static_cast<FXApp*>(app);
        return new FXMainWindow(obj, title, nullptr, nullptr, DECOR_ALL, 0, 0, width, height);
    }

    void fox_main_window_show(FXParentPtr window) {
        static_cast<FXMainWindow*>(window)-> show(PLACEMENT_SCREEN);
    }

// VerticalFrame
    FXParentPtr fox_vertical_frame_new(FXParentPtr win) {
        auto parent = static_cast<FXMainWindow*>(win);
        return new FXVerticalFrame(parent, LAYOUT_FILL_X | LAYOUT_FILL_Y);
    }

// Button
    FXWidgetPtr fox_button_new(FXParentPtr frm, const char* title) {
        auto parent = static_cast<FXVerticalFrame*>(frm);
        return new FXButton(parent, title);
    }
    void fox_button_set_target(FXWidgetPtr btn, CWidgetCb cb, void* ctx) {
        auto button = static_cast<FXButton*>(btn);
        button->setTarget(static_cast<FXObject*>(new CTarget(cb, btn, ctx)));
        button->setSelector(FXMainWindow::ID_LAST);
    }
    uint fox_button_get_state(FXWidgetPtr btn) {
        return static_cast<FXButton*>(btn)->getState();
    }
    const char* fox_button_get_text(FXWidgetPtr btn) {
        return static_cast<FXButton*>(btn)->getText().text();
    }

// TextField
    FXWidgetPtr fox_textfield_new(FXParentPtr frm, int ncols) {
        auto parent = static_cast<FXVerticalFrame*>(frm);
        return new FXTextField(parent, ncols);
    }
    void fox_textfield_set_text(FXWidgetPtr wgt, const char* text) {
        static_cast<FXTextField*>(wgt) -> setText(text);
    }
}
