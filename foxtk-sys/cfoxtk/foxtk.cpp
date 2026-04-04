#include <fx.h>
#include <foxtk.h>

// CALLBACK BRIDGE

class CTarget : public FXObject {
  FXDECLARE(CTarget)
protected:
    CTarget() {}
private:
  CWidgetCb callback = nullptr;
  ObjectPtr widget = nullptr;
  void*      context = nullptr;
public:
  enum { SEL_COMMAND, ID_LAST };
  CTarget(CWidgetCb cb, ObjectPtr wgt, void* ctx) : callback(cb), widget(wgt) , context(ctx) {}
  long onCommand(FXObject*, FXSelector, void*) {
    long result = 1;
    if (this -> callback) result = this -> callback(this -> widget, this -> context);
    return result;
  }
};

FXDEFMAP(CTarget) CTargetMap[] = {
    FXMAPFUNC(SEL_COMMAND, CTarget::SEL_COMMAND, CTarget::onCommand),
};
FXIMPLEMENT(CTarget, FXObject, CTargetMap, ARRAYNUMBER(CTargetMap))

class CTimeout : public FXObject {
  FXDECLARE(CTimeout)
protected:
    CTimeout() {}
private:
  ObjectPtr application = nullptr;
  CTimerCb    callback = nullptr;
  unsigned int nanosec = 0;
  void*        context = nullptr;
public:
  enum { SEL_TIMEOUT, ID_LAST };
  CTimeout(ObjectPtr app, CTimerCb cb, unsigned int ns, void* ctx) {
    this -> application = app;
    this -> callback = cb;
    this -> nanosec = ns;
    static_cast<FXApp*>(app)->addTimeout(this, CTimeout::SEL_TIMEOUT, ns, ctx);
  }
  long onTimeout(FXObject*, FXSelector, void* ctx) {
        long result = 1;
        if (this -> callback) {
            result = this -> callback(this -> application, ctx);
            static_cast<FXApp*>(application)->addTimeout(this, CTimeout::SEL_TIMEOUT, nanosec, ctx);
        };
        return result;
    }
};

FXDEFMAP(CTimeout) CTimeoutMap[] = {
    FXMAPFUNC(SEL_TIMEOUT, CTimeout::SEL_TIMEOUT, CTimeout::onTimeout),
};
FXIMPLEMENT(CTimeout, FXObject, CTimeoutMap, ARRAYNUMBER(CTimeoutMap))

extern "C" {
// FXIdExt
    ObjectPtr fx_id_get_app(ObjectPtr wgt) {
        return static_cast<FXId*>(wgt) -> getApp();
    }

// FXWindowExt
    void fx_window_set_target(ObjectPtr wgt_, CWidgetCb cb, void* ctx) {
        auto wgt = static_cast<FXWindow*>(wgt_);
        wgt->setTarget(static_cast<FXObject*>(new CTarget(cb, wgt_, ctx)));
        wgt->setSelector(CTarget::SEL_COMMAND);
    }

// FXAppExt
    ObjectPtr fx_app_new(const char* name, const char* vendor, int argc, char** argv) {
        auto app = new FXApp(name, vendor);
        app->init(argc, argv);
        return app;
    }
    int fx_app_run(ObjectPtr app_) {
        auto app = static_cast<FXApp*>(app_);
        app->create();
        return app->run();
    }
    ObjectPtr fx_app_add_timeout(ObjectPtr app, CTimerCb cb, unsigned int ns, void* ctx) {
        return new CTimeout(app, cb, ns, ctx);
    }

// FXLabelExt
    const char* fx_label_get_text(ObjectPtr wgt) {
        return static_cast<FXLabel*>(wgt)->getText().text();
    }
    void fx_label_set_text(ObjectPtr wgt, const char* text) {
        static_cast<FXLabel*>(wgt) -> setText(text);
    }
    unsigned int fx_label_get_justify(ObjectPtr wgt) {
        return static_cast<FXLabel*>(wgt)->getJustify();
    }
    void fx_label_set_justify(ObjectPtr wgt, unsigned int justify) {
        static_cast<FXLabel*>(wgt) -> setJustify(justify);
    }

// FXButtonExt
    ObjectPtr fx_button_new(ObjectPtr parent_, const char* title) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXButton(parent, title);
    }

// FXTextFieldExt
    ObjectPtr fx_textfield_new(ObjectPtr parent_, int ncols) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXTextField(parent, ncols);
    }
    const char* fx_textfield_get_text(ObjectPtr wgt) {
        return static_cast<FXTextField*>(wgt)->getText().text();
    }
    void fx_textfield_set_text(ObjectPtr wgt, const char* text) {
        static_cast<FXTextField*>(wgt) -> setText(text);
    }

// FXVerticalFrameExt
    ObjectPtr fx_vertical_frame_new(ObjectPtr parent_) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXVerticalFrame(parent, LAYOUT_FILL_X | LAYOUT_FILL_Y);
    }

// FXHorizontalFrameExt
    ObjectPtr fx_horizontal_frame_new(ObjectPtr parent_) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXHorizontalFrame(parent, LAYOUT_FILL_X | LAYOUT_FILL_Y);
    }

// FXMainWindowExt
    ObjectPtr fx_main_window_new(ObjectPtr app_, const char* title, int width, int height) {
        auto obj = static_cast<FXApp*>(app_);
        return new FXMainWindow(obj, title, nullptr, nullptr, DECOR_ALL, 0, 0, width, height);
    }
    void fx_main_window_show(ObjectPtr wgt) {
        static_cast<FXMainWindow*>(wgt)-> show(PLACEMENT_SCREEN);
    }
}
