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
        static thread_local FXString buffer;
        buffer = static_cast<FXLabel*>(wgt)->getText();
        return buffer.text();
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

// FXRadioButtonExt
    ObjectPtr fx_radio_button_new(ObjectPtr parent_, const char* title) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXRadioButton(parent, title);
    }
    unsigned char fx_radio_button_get_check(ObjectPtr wgt) {
        return static_cast<FXRadioButton*>(wgt)->getCheck();
    }
    void fx_radio_button_set_check(ObjectPtr wgt) {
        static_cast<FXRadioButton*>(wgt)->setCheck();
    }

// FXCheckButtonExt
    ObjectPtr fx_check_button_new(ObjectPtr parent_, const char* title) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXCheckButton(parent, title);
    }
    unsigned char fx_check_button_get_check(ObjectPtr wgt) {
        return static_cast<FXCheckButton*>(wgt)->getCheck();
    }
    void fx_check_button_set_check(ObjectPtr wgt, unsigned char check) {
        static_cast<FXCheckButton*>(wgt)->setCheck(check);
    }

// FXTextFieldExt
    ObjectPtr fx_textfield_new(ObjectPtr parent_, int ncols) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXTextField(parent, ncols);
    }
    const char* fx_textfield_get_text(ObjectPtr wgt) {
        static thread_local FXString buffer;
        buffer = static_cast<FXTextField*>(wgt)->getText();
        return buffer.text();
    }
    void fx_textfield_set_text(ObjectPtr wgt, const char* text) {
        static_cast<FXTextField*>(wgt) -> setText(text);
    }

// FXSpinnerExt
    ObjectPtr fx_spinner_new(ObjectPtr parent_, int cols, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXSpinner(parent, cols, static_cast<FXObject*>(tgt), sel, opts, x, y, w, h, pl, pr, pt, pb);
    }
    int fx_spinner_get_value(ObjectPtr wgt) {
        return static_cast<FXSpinner*>(wgt)->getValue();
    }
    void fx_spinner_set_value(ObjectPtr wgt, int value) {
        static_cast<FXSpinner*>(wgt)->setValue(value);
    }
    void fx_spinner_get_range(ObjectPtr wgt, int* lo, int* hi) {
        FXint lower, upper;
        static_cast<FXSpinner*>(wgt)->getRange(lower, upper);
        if (lo) *lo = lower;
        if (hi) *hi = upper;
    }
    void fx_spinner_set_range(ObjectPtr wgt, int lo, int hi) {
        static_cast<FXSpinner*>(wgt)->setRange(lo, hi);
    }
    void fx_spinner_set_increment(ObjectPtr wgt, int inc) {
        static_cast<FXSpinner*>(wgt)->setIncrement(inc);
    }
    void fx_spinner_increment(ObjectPtr wgt) {
        static_cast<FXSpinner*>(wgt)->increment();
    }
    void fx_spinner_decrement(ObjectPtr wgt) {
        static_cast<FXSpinner*>(wgt)->decrement();
    }

// FXSliderExt
    ObjectPtr fx_slider_new(ObjectPtr parent_, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXSlider(parent, static_cast<FXObject*>(tgt), sel, opts, x, y, w, h, pl, pr, pt, pb);
    }
    int fx_slider_get_value(ObjectPtr wgt) {
        return static_cast<FXSlider*>(wgt)->getValue();
    }
    void fx_slider_set_value(ObjectPtr wgt, int value) {
        static_cast<FXSlider*>(wgt)->setValue(value);
    }
    void fx_slider_get_range(ObjectPtr wgt, int* lo, int* hi) {
        FXint lower, upper;
        static_cast<FXSlider*>(wgt)->getRange(lower, upper);
        if (lo) *lo = lower;
        if (hi) *hi = upper;
    }
    void fx_slider_set_range(ObjectPtr wgt, int lo, int hi) {
        static_cast<FXSlider*>(wgt)->setRange(lo, hi);
    }
    int fx_slider_get_increment(ObjectPtr wgt) {
        return static_cast<FXSlider*>(wgt)->getIncrement();
    }
    void fx_slider_set_increment(ObjectPtr wgt, int inc) {
        static_cast<FXSlider*>(wgt)->setIncrement(inc);
    }

// FXProgressBarExt
    ObjectPtr fx_progressbar_new(ObjectPtr parent_, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXProgressBar(parent, static_cast<FXObject*>(tgt), sel, opts, x, y, w, h, pl, pr, pt, pb);
    }
    void fx_progressbar_set_progress(ObjectPtr wgt, unsigned int value) {
        static_cast<FXProgressBar*>(wgt)->setProgress(value);
    }
    unsigned int fx_progressbar_get_progress(ObjectPtr wgt) {
        return static_cast<FXProgressBar*>(wgt)->getProgress();
    }
    void fx_progressbar_set_total(ObjectPtr wgt, unsigned int value) {
        static_cast<FXProgressBar*>(wgt)->setTotal(value);
    }
    unsigned int fx_progressbar_get_total(ObjectPtr wgt) {
        return static_cast<FXProgressBar*>(wgt)->getTotal();
    }
    void fx_progressbar_increment(ObjectPtr wgt, unsigned int value) {
        static_cast<FXProgressBar*>(wgt)->increment(value);
    }
    void fx_progressbar_show_number(ObjectPtr wgt) {
        static_cast<FXProgressBar*>(wgt)->showNumber();
    }
    void fx_progressbar_hide_number(ObjectPtr wgt) {
        static_cast<FXProgressBar*>(wgt)->hideNumber();
    }
    void fx_progressbar_set_bar_size(ObjectPtr wgt, int size) {
        static_cast<FXProgressBar*>(wgt)->setBarSize(size);
    }
    int fx_progressbar_get_bar_size(ObjectPtr wgt) {
        return static_cast<FXProgressBar*>(wgt)->getBarSize();
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
