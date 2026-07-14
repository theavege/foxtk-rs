#include <fx.h>
#include <FXGradientBar.h>

//~ OPAQUE HANDLES

typedef void ObjectPtr;
typedef long (*CWidgetCb)(ObjectPtr* widget, void* context);
typedef long (*CTimerCb)(ObjectPtr* application, void* context);

//~ CALLBACK BRIDGE

class CTarget : public FXObject {
  FXDECLARE(CTarget)
protected:
    CTarget() {}
private:
  CWidgetCb callback = nullptr;
  void*      context = nullptr;
public:
  enum { SEL_COMMAND, SEL_CHANGED };
  CTarget(CWidgetCb cb, void* ctx) : callback(cb) , context(ctx) {}
  long callBack(FXObject* wgt, FXSelector, void*) {
    long result = 0;
    if (this -> callback) result = this -> callback(wgt, this -> context);
    return result;
  }
};

FXDEFMAP(CTarget) CTargetMap[] = {
    FXMAPFUNC(SEL_COMMAND, CTarget::SEL_COMMAND, CTarget::callBack),
    FXMAPFUNC(SEL_CHANGED, CTarget::SEL_CHANGED, CTarget::callBack),
};
FXIMPLEMENT(CTarget, FXObject, CTargetMap, ARRAYNUMBER(CTargetMap))

class CTimeout : public FXObject {
  FXDECLARE(CTimeout)
protected:
    CTimeout() {}
private:
  CTimerCb    callback = nullptr;
  unsigned int nanosec = 0;
public:
    enum { SEL_TIMEOUT, SEL_CHORE };
    CTimeout(CTimerCb cb, unsigned int ns) {
        this -> callback = cb;
        this -> nanosec = ns;
    }
    long onTimeout(FXObject* app, FXSelector, void* ctx) {
        long result = 0;
        if (this -> callback) {
            result = this -> callback(app, ctx);
            static_cast<FXApp*>(app) -> removeTimeout(this, CTimeout::SEL_TIMEOUT);
            static_cast<FXApp*>(app) -> addTimeout(this, CTimeout::SEL_TIMEOUT, nanosec, ctx);
        };
        return result;
    }
    long onChore(FXObject* app, FXSelector, void* ctx) {
        long result = 0;
        if (this -> callback) {
            result = this -> callback(app, ctx);
        };
        return result;
    }
};

FXDEFMAP(CTimeout) CTimeoutMap[] = {
    FXMAPFUNC(SEL_TIMEOUT, CTimeout::SEL_TIMEOUT, CTimeout::onTimeout),
    FXMAPFUNC(SEL_CHORE, CTimeout::SEL_CHORE, CTimeout::onChore),
};
FXIMPLEMENT(CTimeout, FXObject, CTimeoutMap, ARRAYNUMBER(CTimeoutMap))

// CMouseTarget bridge (defined outside extern "C")
class CMouseTarget : public FXObject {
    FXDECLARE(CMouseTarget)
protected:
    CMouseTarget() {}
private:
    long (*callback)(ObjectPtr*, int, int, int, void*) = nullptr;
    void* context = nullptr;
public:
    enum { SEL_LBP = SEL_LEFTBUTTONPRESS, SEL_LBR = SEL_LEFTBUTTONRELEASE, SEL_MOT = SEL_MOTION, SEL_RBP = SEL_RIGHTBUTTONPRESS, SEL_RBR = SEL_RIGHTBUTTONRELEASE };
    CMouseTarget(long (*cb)(ObjectPtr*, int, int, int, void*), void* ctx) : callback(cb), context(ctx) {}
    long callBack(FXObject* wgt, FXSelector sel, void* ptr) {
        long result = 0;
        if (this->callback) {
                int x = 0;
                int y = 0;
                FXEvent* ev = static_cast<FXEvent*>(ptr);
                if (ev) {
                    x = ev->win_x;
                    y = ev->win_y;
                }
                int code = 0;
                if (sel == SEL_LEFTBUTTONPRESS) code = 1;
                else if (sel == SEL_LEFTBUTTONRELEASE) code = 2;
                else if (sel == SEL_MOTION) code = 3;
                else if (sel == SEL_RIGHTBUTTONPRESS) code = 4;
                else if (sel == SEL_RIGHTBUTTONRELEASE) code = 5;
                result = this->callback(wgt, code, x, y, this->context);
        }
        return result;
    }
};

FXDEFMAP(CMouseTarget) CMouseTargetMap[] = {
    FXMAPFUNC(SEL_LEFTBUTTONPRESS, CMouseTarget::SEL_LBP, CMouseTarget::callBack),
    FXMAPFUNC(SEL_LEFTBUTTONRELEASE, CMouseTarget::SEL_LBR, CMouseTarget::callBack),
    FXMAPFUNC(SEL_MOTION, CMouseTarget::SEL_MOT, CMouseTarget::callBack),
    FXMAPFUNC(SEL_RIGHTBUTTONPRESS, CMouseTarget::SEL_RBP, CMouseTarget::callBack),
    FXMAPFUNC(SEL_RIGHTBUTTONRELEASE, CMouseTarget::SEL_RBR, CMouseTarget::callBack),
};
FXIMPLEMENT(CMouseTarget, FXObject, CMouseTargetMap, ARRAYNUMBER(CMouseTargetMap))

// FOX marks these constructors protected to discourage direct instantiation,
// but we need them for the C bridge. These thin subclasses expose them safely.

class FXTopWindowEx : public FXTopWindow {
public:
    FXTopWindowEx(FXApp* app, const FXString& name, FXIcon* ic, FXIcon* mi, FXuint opts, FXint x, FXint y, FXint w, FXint h, FXint pl, FXint pr, FXint pt, FXint pb, FXint hs, FXint vs)
        : FXTopWindow(app, name, ic, mi, opts, x, y, w, h, pl, pr, pt, pb, hs, vs) {}
};

class FXShellEx : public FXShell {
public:
    FXShellEx(FXWindow* owner, FXuint opts, FXint x, FXint y, FXint w, FXint h)
        : FXShell(owner, opts, x, y, w, h) {}
};

class FXScrollAreaEx : public FXScrollArea {
public:
    FXScrollAreaEx(FXComposite* parent, FXuint opts, FXint x, FXint y, FXint w, FXint h)
        : FXScrollArea(parent, opts, x, y, w, h) {}
};

template <typename Widget, typename Parent, typename... Args>
inline ObjectPtr* make_widget(ObjectPtr* parent, Args&&... args) {
    if (!parent) {
        return nullptr;
    }
    return new Widget(static_cast<Parent*>(parent), std::forward<Args>(args)...);
}

template <typename Widget, typename... Args>
inline ObjectPtr* make_widget(Args&&... args) {
    return new Widget(std::forward<Args>(args)...);
}

template <typename Value>
inline const char* string_result(const Value& value) {
    static thread_local FXString buffer;
    buffer = value;
    return buffer.text();
}

#ifdef DEBUG
#define ASSERT_NOT_NULL(ptr) assert((ptr) != nullptr)
#else
#define ASSERT_NOT_NULL(ptr) ((void)0)
#endif

extern "C" {
    unsigned int fx_rgb(unsigned int r, unsigned int g, unsigned int b) {
        return FXRGB(r,g,b);
    }
    unsigned int fx_rgba(unsigned int r, unsigned int g, unsigned int b, unsigned int a) {
        return FXRGBA(r,g,b,a);
    }
    unsigned int fx_red_val(unsigned int rgba) {
        return FXREDVAL(rgba);
    }
    unsigned int fx_green_val(unsigned int rgba) {
        return FXGREENVAL(rgba);
    }
    unsigned int fx_blue_val(unsigned int rgba) {
        return FXBLUEVAL(rgba);
    }
    unsigned int fx_alpha_val(unsigned int rgba) {
        return FXALPHAVAL(rgba);
    }

//~ FXObject
    void fx_object_delete(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        if (wgt) delete static_cast<FXObject*>(wgt);
    }

//~ FXId
    ObjectPtr* fx_id_get_app(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXId*>(wgt) -> getApp();
    }
    FXID fx_id_get_id(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXId*>(wgt) -> id();
    }

//~ FXDrawable.h
    int fx_drawable_get_height(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXDrawable*>(wgt) -> getHeight();
    }
    int fx_drawable_get_width(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXDrawable*>(wgt) -> getWidth();
    }

//~ FXWindow.h
    FXWindow* fx_window_get_parent(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXWindow*>(wgt) -> getParent();
    }
    FXWindow* fx_window_get_root(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXWindow*>(wgt) -> getRoot();
    }
    long fx_window_has_focus(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXWindow*>(wgt) -> hasFocus();
    }
    void fx_window_set_target(ObjectPtr* wgt, CWidgetCb cb, void* ctx) {
        ASSERT_NOT_NULL(wgt);
        auto win = static_cast<FXWindow*>(wgt);
        auto old = win->getTarget();
        if (dynamic_cast<CTarget*>(old)) delete old;
        win->setTarget(new CTarget(cb, ctx));
    }
    void fx_window_set_selector(ObjectPtr* wgt_, int val) {
        ASSERT_NOT_NULL(wgt);
        auto wgt = static_cast<FXWindow*>(wgt_);
        if (val == 0) wgt->setSelector(CTarget::SEL_COMMAND);
        else if (val == 1) wgt->setSelector(CTarget::SEL_CHANGED);
    }
    void fx_window_set_width(ObjectPtr* wgt, int width) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXWindow*>(wgt) -> setWidth(width);
    }
    void fx_window_set_x(ObjectPtr* wgt, int x) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXWindow*>(wgt) -> setX(x);
    }
    void fx_window_set_y(ObjectPtr* wgt, int y) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXWindow*>(wgt) -> setY(y);
    }
    void fx_window_set_height(ObjectPtr* wgt, int height) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXWindow*>(wgt) -> setHeight(height);
    }
    void fx_window_set_layout_hints(ObjectPtr* wgt, unsigned int val) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXWindow*>(wgt) -> setLayoutHints(val);
    }
    void fx_window_disable(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXWindow*>(wgt) -> disable();
    }
    void fx_window_enable(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXWindow*>(wgt) -> enable();
    }

//~ FXComposite.h
    int fx_composite_child_width(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXComposite*>(wgt) -> maxChildWidth();
    }
    int fx_composite_child_height(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXComposite*>(wgt) -> maxChildHeight();
    }

//~ FXApp
    ObjectPtr* fx_app_new(const char* name, const char* vendor, int argc, char** argv) {
        auto app = new FXApp(name, vendor);
        app->init(argc, argv);
        return app;
    }
    int fx_app_run(ObjectPtr* app_) {
        ASSERT_NOT_NULL(app_);
        auto app = static_cast<FXApp*>(app_);
        app->create();
        return app->run();
    }
    void fx_app_add_timeout(ObjectPtr* app, CTimerCb cb, unsigned int ns, void* ctx) {
        ASSERT_NOT_NULL(app);
        static_cast<FXApp*>(app) -> addTimeout(new CTimeout(cb, ns), CTimeout::SEL_TIMEOUT, ns, ctx);
    }

//~ FXFrame
    void fx_frame_set_frame_style(ObjectPtr* wgt, unsigned int style) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXFrame*>(wgt) -> setFrameStyle(style);
    }
    void fx_frame_set_pad_bottom(ObjectPtr* wgt, int pad) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXFrame*>(wgt) -> setPadBottom(pad);
    }
    void fx_frame_set_pad_left(ObjectPtr* wgt, int pad) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXFrame*>(wgt) -> setPadLeft(pad);
    }
    void fx_frame_set_pad_right(ObjectPtr* wgt, int pad) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXFrame*>(wgt) -> setPadRight(pad);
    }
    void fx_frame_set_pad_top(ObjectPtr* wgt, int pad) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXFrame*>(wgt) -> setPadTop(pad);
    }
    void fx_frame_set_base_color(ObjectPtr* wgt, unsigned int color) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXFrame*>(wgt) -> setBaseColor(color);
    }
    void fx_frame_set_border_color(ObjectPtr* wgt, unsigned int color) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXFrame*>(wgt) -> setBorderColor(color);
    }
    void fx_frame_set_hilite_color(ObjectPtr* wgt, unsigned int color) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXFrame*>(wgt) -> setHiliteColor(color);
    }
    void fx_frame_set_shadow_color(ObjectPtr* wgt, unsigned int color) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXFrame*>(wgt) -> setShadowColor(color);
    }

//~ FXKnob.h
    ObjectPtr* fx_knob_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(wgt);
        return make_widget<FXKnob, FXComposite>(prt);
    }
    void fx_knob_set_help_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXKnob*>(wgt) -> setHelpText(text);
    }
    void fx_knob_set_tip_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXKnob*>(wgt) -> setTipText(text);
    }
    void fx_knob_set_value(ObjectPtr* wgt, int value) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXKnob*>(wgt) -> setValue(value);
    }
    void fx_knob_set_range(ObjectPtr* wgt, int lo, int hi) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXKnob*>(wgt) -> setRange(lo, hi);
    }
    void fx_knob_set_increment(ObjectPtr* wgt, int inc) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXKnob*>(wgt) -> setIncrement(inc);
    }


//~ FXLabel
    ObjectPtr* fx_label_new(ObjectPtr* prt, const char* title) {
        ASSERT_NOT_NULL(wgt);
        return make_widget<FXLabel, FXComposite>(prt, title);
    }
    const char* fx_label_get_text(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return string_result(static_cast<FXLabel*>(wgt) -> getText());
    }
    void fx_label_set_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXLabel*>(wgt) -> setText(text);
    }
    void fx_label_set_help_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXLabel*>(wgt) -> setHelpText(text);
    }
    void fx_label_set_tip_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXLabel*>(wgt) -> setTipText(text);
    }
    void fx_label_set_justify(ObjectPtr* wgt, unsigned int justify) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXLabel*>(wgt) -> setJustify(justify);
    }
    void fx_label_set_text_color(ObjectPtr* wgt, unsigned int color) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXLabel*>(wgt) -> setTextColor(color);
    }

//~ FXArrowButton.h
    ObjectPtr* fx_arrow_button_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(wgt);
        return make_widget<FXArrowButton, FXComposite>(prt);
    }
    void fx_arrow_button_set_arrow_size(ObjectPtr* wgt, int size) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXArrowButton*>(wgt) -> setArrowSize(size);
    }
    void fx_arrow_button_set_arrow_color(ObjectPtr* wgt, unsigned int color) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXArrowButton*>(wgt) -> setArrowColor(color);
    }

//~ FXMessageBox.h
    unsigned int fx_message_box_error(ObjectPtr* owner, unsigned int opts, const char* caption, const char* message) {
        ASSERT_NOT_NULL(owner);
        return FXMessageBox::error(static_cast<FXWindow*>(owner), opts, caption, message);
    }
    unsigned int fx_message_box_warning(ObjectPtr* owner, unsigned int opts, const char* caption, const char* message) {
        ASSERT_NOT_NULL(owner);
        return FXMessageBox::warning(static_cast<FXWindow*>(owner), opts, caption, message);
    }
    unsigned int fx_message_box_question(ObjectPtr* owner, unsigned int opts, const char* caption, const char* message) {
        ASSERT_NOT_NULL(owner);
        return FXMessageBox::question(static_cast<FXWindow*>(owner), opts, caption, message);
    }
    unsigned int fx_message_box_information(ObjectPtr* owner, unsigned int opts, const char* caption, const char* message) {
        ASSERT_NOT_NULL(owner);
        return FXMessageBox::information(static_cast<FXWindow*>(owner), opts, caption, message);
    }

//~ FXChoiceBox.h
    int fx_choice_box_ask(ObjectPtr* app, unsigned int opts, const char* caption, const char* text, ObjectPtr* icon, const char** choices) {
        ASSERT_NOT_NULL(app);
        return FXChoiceBox::ask(static_cast<FXApp*>(app), opts, caption, text, static_cast<FXIcon*>(icon), choices);
    }

//~ FXTriStateButton.h
    ObjectPtr* fx_tri_state_button_new(ObjectPtr* prt, const char* text1, const char* text2, const char* text3) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXTriStateButton, FXComposite>(prt, text1, text2, text3);
    }

//~ FXTreeListBox.h
    ObjectPtr* fx_tree_list_box_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXTreeListBox, FXComposite>(prt);
    }

//~ FXDriveBox.h
    ObjectPtr* fx_drive_box_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXDriveBox, FXComposite>(prt);
    }

//~ FXDirBox.h
    ObjectPtr* fx_dir_box_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXDirBox, FXComposite>(prt);
    }
    ObjectPtr* fx_dir_list_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXDirList, FXComposite>(prt);
    }
    ObjectPtr* fx_dir_selector_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXDirSelector, FXComposite>(prt);
    }

//~ FXFileSelector.h
    ObjectPtr* fx_file_selector_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXFileSelector, FXComposite>(prt);
    }
    ObjectPtr* fx_file_list_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXFileList, FXComposite>(prt);
    }

//~ FXFontSelector.h
    ObjectPtr* fx_font_selector_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXFontSelector, FXComposite>(prt);
    }

//~ FXColorSelector.h
    ObjectPtr* fx_color_selector_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXColorSelector, FXComposite>(prt);
    }

//~ FXDial.h
    ObjectPtr* fx_dial_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXDial, FXComposite>(prt);
    }

//~ FXRealSpinner.h
    ObjectPtr* fx_real_spinner_new(ObjectPtr* prt, int cols) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXRealSpinner, FXComposite>(prt, cols);
    }

//~ FXRealSlider.h
    ObjectPtr* fx_real_slider_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXRealSlider, FXComposite>(prt);
    }

//~ FXColorWell.h
    ObjectPtr* fx_color_well_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXColorWell, FXComposite>(prt);
    }

//~ FXColorWheel.h
    ObjectPtr* fx_color_wheel_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXColorWheel, FXComposite>(prt);
    }

//~ FXColorRing.h
    ObjectPtr* fx_color_ring_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXColorRing, FXComposite>(prt);
    }

//~ FXColorBar.h
    ObjectPtr* fx_color_bar_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXColorBar, FXComposite>(prt);
    }

//~ FXGradientBar.h
    ObjectPtr* fx_gradient_bar_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FX::FXGradientBar, FXComposite>(prt);
    }

//~ FX7Segment.h
    ObjectPtr* fx_7segment_new(ObjectPtr* prt, const char* text) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FX7Segment, FXComposite>(prt, text);
    }

//~ FXColorDialog.h
    ObjectPtr* fx_color_dialog_new(ObjectPtr* owner, const char* title) {
        ASSERT_NOT_NULL(owner);
        return make_widget<FXColorDialog, FXWindow>(owner, title);
    }

//~ FXDialogBox.h
    ObjectPtr* fx_dialog_box_new(ObjectPtr* owner, const char* title) {
        return make_widget<FXDialogBox, FXWindow>(owner, title);
    }

//~ FXFileDialog.h
    const char* fx_file_dialog_get_open_filename(ObjectPtr* owner, const char* caption, const char* path, const char* patterns, int initial) {
        return string_result(FXFileDialog::getOpenFilename(static_cast<FXWindow*>(owner), caption, path, patterns, initial));
    }
    const char* fx_file_dialog_get_save_filename(ObjectPtr* owner, const char* caption, const char* path, const char* patterns, int initial) {
        return string_result(FXFileDialog::getSaveFilename(static_cast<FXWindow*>(owner), caption, path, patterns, initial));
    }

//~ FXButton.h
    ObjectPtr* fx_button_new(ObjectPtr* prt, const char* title) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXButton, FXComposite>(prt, title);
    }
    void fx_button_set_state(ObjectPtr* wgt, unsigned int state) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXButton*>(wgt) -> setState(state);
    }
    void fx_button_set_style(ObjectPtr* wgt, unsigned int style) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXButton*>(wgt) -> setButtonStyle(style);
    }

//~ FXCheckButton.h
    ObjectPtr* fx_check_button_new(ObjectPtr* prt, const char* title) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXCheckButton, FXComposite>(prt, title);
    }
    unsigned char fx_check_button_get_check(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXCheckButton*>(wgt) -> getCheck();
    }
    void fx_check_button_set_check(ObjectPtr* wgt, unsigned char check) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXCheckButton*>(wgt) -> setCheck(check);
    }

//~ FXRadioButton.h
    ObjectPtr* fx_radio_button_new(ObjectPtr* prt, const char* title) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXRadioButton, FXComposite>(prt, title);
    }
    unsigned char fx_radio_button_get_check(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXRadioButton*>(wgt) -> getCheck();
    }
    void fx_radio_button_set_check(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXRadioButton*>(wgt) -> setCheck();
    }

//~ FXToggleButton.h
    ObjectPtr* fx_toggle_button_new(ObjectPtr* prt, const char* text1, const char* text2) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXToggleButton, FXComposite>(prt, text1, text2);
    }

//~ FXText.h
    ObjectPtr* fx_text_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXText, FXComposite>(prt);
    }
    const char* fx_text_get_text(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static thread_local FXString buffer;
        buffer = static_cast<FXText*>(wgt) -> getText();
        return string_result(buffer.text());
    }
    void fx_text_set_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXText*>(wgt) -> setText(text);
    }
    void fx_text_set_editable(ObjectPtr* wgt, long editable) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXText*>(wgt) -> setEditable(editable != 0);
    }
    void fx_text_set_help_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXText*>(wgt) -> setHelpText(text);
    }
    void fx_text_set_tip_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXText*>(wgt) -> setTipText(text);
    }
    void fx_text_set_font(ObjectPtr* wgt, const char* family, int size) {
        ASSERT_NOT_NULL(wgt);
        auto text = static_cast<FXText*>(wgt);
        auto old_font = text->getFont();
        auto new_font = new FXFont(text->getApp(), family, size, 0, 0);
        text->setFont(new_font);
        // Only delete if it's not the app's default font
        if (old_font && old_font != text->getApp()->getNormalFont()) {
            delete old_font;
        }
    }

//~ FXTextField
    ObjectPtr* fx_textfield_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXTextField, FXComposite>(prt, 8);
    }
    const char* fx_textfield_get_text(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static thread_local FXString buffer;
        buffer = static_cast<FXTextField*>(wgt) -> getText();
        return string_result(buffer.text());
    }
    void fx_textfield_set_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXTextField*>(wgt) -> setText(text);
    }
    void fx_textfield_set_help_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXTextField*>(wgt) -> setHelpText(text);
    }
    void fx_textfield_set_tip_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXTextField*>(wgt) -> setTipText(text);
    }
    void fx_textfield_set_editable(ObjectPtr* wgt, long val) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXTextField*>(wgt) -> setEditable(val != 0);
    }
    void fx_textfield_set_text_color(ObjectPtr* wgt, unsigned int color) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXTextField*>(wgt) -> setTextColor(color);
    }

//~ FXSpinner
    ObjectPtr* fx_spinner_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXSpinner, FXComposite>(prt, 8);
    }
    int fx_spinner_get_value(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXSpinner*>(wgt) -> getValue();
    }
    void fx_spinner_set_value(ObjectPtr* wgt, int value) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSpinner*>(wgt) -> setValue(value);
    }
    void fx_spinner_get_range(ObjectPtr* wgt, int* lo, int* hi) {
        ASSERT_NOT_NULL(wgt);
        FXint lower, upper;
        static_cast<FXSpinner*>(wgt) -> getRange(lower, upper);
        if (lo) *lo = lower;
        if (hi) *hi = upper;
    }
    void fx_spinner_set_range(ObjectPtr* wgt, int lo, int hi) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSpinner*>(wgt) -> setRange(lo, hi);
    }
    void fx_spinner_set_increment(ObjectPtr* wgt, int inc) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSpinner*>(wgt) -> setIncrement(inc);
    }
    void fx_spinner_increment(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSpinner*>(wgt) -> increment();
    }
    void fx_spinner_decrement(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSpinner*>(wgt) -> decrement();
    }

//~ FXSlider
    ObjectPtr* fx_slider_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXSlider, FXComposite>(prt);
    }
    int fx_slider_get_increment(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXSlider*>(wgt) -> getIncrement();
    }
    int fx_slider_get_value(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXSlider*>(wgt) -> getValue();
    }
    void fx_slider_get_range(ObjectPtr* wgt, int* lo, int* hi) {
        ASSERT_NOT_NULL(wgt);
        FXint lower, upper;
        static_cast<FXSlider*>(wgt) -> getRange(lower, upper);
        if (lo) *lo = lower;
        if (hi) *hi = upper;
    }
    void fx_slider_set_value(ObjectPtr* wgt, int value) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSlider*>(wgt) -> setValue(value);
    }
    void fx_slider_set_range(ObjectPtr* wgt, int lo, int hi) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSlider*>(wgt) -> setRange(lo, hi);
    }
    void fx_slider_set_increment(ObjectPtr* wgt, int inc) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSlider*>(wgt) -> setIncrement(inc);
    }

//~ FXProgressBar
    ObjectPtr* fx_progressbar_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXProgressBar, FXComposite>(prt);
    }
    void fx_progressbar_set_progress(ObjectPtr* wgt, unsigned int value) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXProgressBar*>(wgt) -> setProgress(value);
    }
    unsigned int fx_progressbar_get_progress(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXProgressBar*>(wgt) -> getProgress();
    }
    void fx_progressbar_set_total(ObjectPtr* wgt, unsigned int value) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXProgressBar*>(wgt) -> setTotal(value);
    }
    unsigned int fx_progressbar_get_total(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXProgressBar*>(wgt) -> getTotal();
    }
    void fx_progressbar_increment(ObjectPtr* wgt, unsigned int value) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXProgressBar*>(wgt) -> increment(value);
    }
    void fx_progressbar_show_number(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXProgressBar*>(wgt) -> showNumber();
    }
    void fx_progressbar_hide_number(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXProgressBar*>(wgt) -> hideNumber();
    }
    void fx_progressbar_set_bar_size(ObjectPtr* wgt, int size) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXProgressBar*>(wgt) -> setBarSize(size);
    }
    int fx_progressbar_get_bar_size(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXProgressBar*>(wgt) -> getBarSize();
    }

//~ FXPacker
    ObjectPtr* fx_packer_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXPacker, FXComposite>(prt);
    }
    void fx_packer_set_hspacing(ObjectPtr* wgt, int val) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXPacker*>(wgt) -> setHSpacing(val);
    }
    void fx_packer_set_vspacing(ObjectPtr* wgt, int val) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXPacker*>(wgt) -> setVSpacing(val);
    }

//~ FXMatrix
    ObjectPtr* fx_matrix_new(ObjectPtr* prt, int rows, unsigned int opts) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXMatrix, FXComposite>(prt, rows, opts);
    }
    void fx_matrix_set_matrix_style(ObjectPtr* wgt, unsigned int style) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXMatrix*>(wgt) -> setMatrixStyle(style);
    }
    void fx_matrix_set_num_rows(ObjectPtr* wgt, int rows) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXMatrix*>(wgt) -> setNumRows(rows);
    }
    void fx_matrix_set_num_columns(ObjectPtr* wgt, int cols) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXMatrix*>(wgt) -> setNumColumns(cols);
    }
    unsigned int fx_matrix_get_matrix_style(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXMatrix*>(wgt) -> getMatrixStyle();
    }
    int fx_matrix_get_num_rows(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXMatrix*>(wgt) -> getNumRows();
    }
    int fx_matrix_get_num_columns(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXMatrix*>(wgt) -> getNumColumns();
    }

//~ FXSplitter
    ObjectPtr* fx_splitter_new(ObjectPtr* prt, unsigned int opts) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXSplitter, FXComposite>(prt, opts);
    }
    int fx_splitter_get_split(ObjectPtr* wgt, int index) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXSplitter*>(wgt) -> getSplit(index);
    }
    void fx_splitter_set_split(ObjectPtr* wgt, int index, int size) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSplitter*>(wgt) -> setSplit(index, size);
    }
    void fx_splitter_set_splitter_style(ObjectPtr* wgt, unsigned int style) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSplitter*>(wgt) -> setSplitterStyle(style);
    }
    unsigned int fx_splitter_get_splitter_style(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXSplitter*>(wgt) -> getSplitterStyle();
    }
    void fx_splitter_set_bar_size(ObjectPtr* wgt, int size) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSplitter*>(wgt) -> setBarSize(size);
    }
    int fx_splitter_get_bar_size(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXSplitter*>(wgt) -> getBarSize();
    }

//~ FX4Splitter
    ObjectPtr* fx_four_splitter_new(ObjectPtr* prt, unsigned int opts) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FX4Splitter, FXComposite>(prt, opts);
    }

//~ FXScrollArea
    ObjectPtr* fx_scroll_area_new(ObjectPtr* prt, unsigned int opts, int x, int y, int w, int h) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXScrollAreaEx, FXComposite>(prt, opts, x, y, w, h);
    }

//~ FXScrollWindow
    ObjectPtr* fx_scroll_window_new(ObjectPtr* prt, unsigned int opts, int x, int y, int w, int h) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXScrollWindow, FXComposite>(prt, opts, x, y, w, h);
    }

//~ FXGroupBox
    ObjectPtr* fx_groupbox_new(ObjectPtr* prt, const char* title) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXGroupBox, FXComposite>(prt, title);
    }
    void fx_groupbox_set_style(ObjectPtr* wgt, unsigned int style) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXGroupBox*>(wgt) -> setGroupBoxStyle(style);
    }
    void fx_groupbox_set_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXGroupBox*>(wgt) -> setText(text);
    }

//~ FXVerticalFrame
    ObjectPtr* fx_vertical_frame_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXVerticalFrame, FXComposite>(prt);
    }

//~ FXHorizontalFrame
    ObjectPtr* fx_horizontal_frame_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXHorizontalFrame, FXComposite>(prt);
    }

//~ FXSpring
    ObjectPtr* fx_spring_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXSpring, FXComposite>(prt);
    }

//~ FXSwitcher
    ObjectPtr* fx_switcher_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXSwitcher, FXComposite>(prt);
    }

    void fx_switcher_set_current(ObjectPtr* wgt, int index) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXSwitcher*>(wgt) -> setCurrent(index);
    }

//~ FXDCWindow
    ObjectPtr* fx_dc_window_new(ObjectPtr* drawable) {
        ASSERT_NOT_NULL(drawable);
        return make_widget<FXDCWindow, FXDrawable>(drawable);
    }

//~ FXDC (drawing)
    void fx_dc_set_foreground(ObjectPtr* dc, unsigned int color) {
        ASSERT_NOT_NULL(dc);
        static_cast<FXDCWindow*>(dc) -> setForeground(color);
    }
    void fx_dc_set_line_width(ObjectPtr* dc, int width) {
        ASSERT_NOT_NULL(dc);
        static_cast<FXDCWindow*>(dc) -> setLineWidth(width);
    }
    void fx_dc_draw_line(ObjectPtr* dc, int x1, int y1, int x2, int y2) {
        ASSERT_NOT_NULL(dc);
        static_cast<FXDCWindow*>(dc) -> drawLine(x1, y1, x2, y2);
    }
    void fx_dc_draw_point(ObjectPtr* dc, int x, int y) {
        ASSERT_NOT_NULL(dc);
        static_cast<FXDCWindow*>(dc) -> drawPoint(x, y);
    }
    void fx_dc_draw_rect(ObjectPtr* dc, int x, int y, int w, int h) {
        static_cast<FXDCWindow*>(dc) -> drawRectangle(x, y, w, h);
    }
    void fx_dc_fill_rect(ObjectPtr* dc, int x, int y, int w, int h) {
        static_cast<FXDCWindow*>(dc) -> fillRectangle(x, y, w, h);
    }

//~ FXSplashWindow
    ObjectPtr* fx_splash_window_new(ObjectPtr* app) {
        return make_widget<FXSplashWindow, FXApp>(app, nullptr);
    }

//~ FXToolBarShell
    ObjectPtr* fx_tool_bar_shell_new(ObjectPtr* owner) {
        return make_widget<FXToolBarShell, FXWindow>(owner);
    }

//~ FXRootWindow
    ObjectPtr* fx_root_window_new(ObjectPtr* app) {
        return make_widget<FXRootWindow, FXApp>(app, nullptr);
    }

//~ FXShell
    ObjectPtr* fx_shell_new(ObjectPtr* owner, unsigned int opts, int x, int y, int w, int h) {
        return make_widget<FXShellEx, FXWindow>(owner, opts, x, y, w, h);
    }

//~ FXMainWindow
    ObjectPtr* fx_main_window_new(ObjectPtr* app, const char* title, int width, int height) {
        return make_widget<FXMainWindow, FXApp>(app, title, nullptr, nullptr, DECOR_ALL, 0, 0, width, height);
    }
    void fx_main_window_show(ObjectPtr* wgt) {
        static_cast<FXMainWindow*>(wgt) ->  show(PLACEMENT_SCREEN);
    }

//~ FXTopWindow.h
    void fx_top_window_set_decorations(ObjectPtr* wgt, unsigned int decorations) {
        static_cast<FXTopWindow*>(wgt) -> setDecorations(decorations);
    }
    void fx_top_window_set_hspacing(ObjectPtr* wgt, int hspacing) {
        static_cast<FXTopWindow*>(wgt) -> setHSpacing(hspacing);
    }
    void fx_top_window_set_vspacing(ObjectPtr* wgt, int vspacing) {
        static_cast<FXTopWindow*>(wgt) -> setVSpacing(vspacing);
    }

//~ FXComboBox
    ObjectPtr* fx_combo_box_new(ObjectPtr* prt, int cols) {
        return make_widget<FXComboBox, FXComposite>(prt, cols);
    }
    void fx_combo_box_append_item(ObjectPtr* wgt, const char* text) {
        static_cast<FXComboBox*>(wgt) -> appendItem(text);
    }
    void fx_combo_box_clear_items(ObjectPtr* wgt) {
        static_cast<FXComboBox*>(wgt) -> clearItems();
    }
    int fx_combo_box_get_current_item(ObjectPtr* wgt) {
        return static_cast<FXComboBox*>(wgt) -> getCurrentItem();
    }
    void fx_combo_box_set_current_item(ObjectPtr* wgt, int index) {
        static_cast<FXComboBox*>(wgt) -> setCurrentItem(index);
    }
    void fx_combo_box_set_num_visible(ObjectPtr* wgt, int nvis) {
        static_cast<FXComboBox*>(wgt) -> setNumVisible(nvis);
    }
    const char* fx_combo_box_get_item_text(ObjectPtr* wgt, int index) {
        static thread_local FXString buffer;
        buffer = static_cast<FXComboBox*>(wgt) -> getItemText(index);
        return buffer.text();
    }
    int fx_combo_box_get_num_items(ObjectPtr* wgt) {
        return static_cast<FXComboBox*>(wgt) -> getNumItems();
    }

//~ FXList
    ObjectPtr* fx_list_new(ObjectPtr* prt) {
        return make_widget<FXList, FXComposite>(prt);
    }
    void fx_list_append_item(ObjectPtr* wgt, const char* text) {
        static_cast<FXList*>(wgt) -> appendItem(text);
    }
    void fx_list_clear_items(ObjectPtr* wgt) {
        static_cast<FXList*>(wgt) -> clearItems();
    }
    void fx_list_set_current_item(ObjectPtr* wgt, int index) {
        static_cast<FXList*>(wgt) -> setCurrentItem(index);
    }
    void fx_list_set_num_visible(ObjectPtr* wgt, int nvis) {
        static_cast<FXList*>(wgt) -> setNumVisible(nvis);
    }
    void fx_list_set_style(ObjectPtr* wgt, unsigned int style) {
        static_cast<FXList*>(wgt) -> setListStyle(style);
    }
    const char* fx_list_get_item_text(ObjectPtr* wgt, int index) {
        static thread_local FXString buffer;
        buffer = static_cast<FXList*>(wgt) -> getItemText(index);
        return buffer.text();
    }
    int fx_list_get_current_item(ObjectPtr* wgt) {
        return static_cast<FXList*>(wgt) -> getCurrentItem();
    }
    int fx_list_get_num_items(ObjectPtr* wgt) {
        return static_cast<FXList*>(wgt) -> getNumItems();
    }

//~ FXListBox
    ObjectPtr* fx_list_box_new(ObjectPtr* prt) {
        return make_widget<FXListBox, FXComposite>(prt);
    }
    void fx_list_box_append_item(ObjectPtr* wgt, const char* text) {
        static_cast<FXListBox*>(wgt) -> appendItem(text);
    }
    void fx_list_box_clear_items(ObjectPtr* wgt) {
        static_cast<FXListBox*>(wgt) -> clearItems();
    }
    int fx_list_box_get_current_item(ObjectPtr* wgt) {
        return static_cast<FXListBox*>(wgt) -> getCurrentItem();
    }
    void fx_list_box_set_current_item(ObjectPtr* wgt, int index) {
        static_cast<FXListBox*>(wgt) -> setCurrentItem(index);
    }
    const char* fx_list_box_get_item_text(ObjectPtr* wgt, int index) {
        static thread_local FXString buffer;
        buffer = static_cast<FXListBox*>(wgt) -> getItemText(index);
        return buffer.text();
    }
    int fx_list_box_get_num_items(ObjectPtr* wgt) {
        return static_cast<FXListBox*>(wgt) -> getNumItems();
    }
    void fx_list_box_set_num_visible(ObjectPtr* wgt, int nvis) {
        static_cast<FXListBox*>(wgt) -> setNumVisible(nvis);
    }

//~ FXTreeList
    ObjectPtr* fx_tree_list_new(ObjectPtr* prt) {
        return make_widget<FXTreeList, FXComposite>(prt);
    }
    ObjectPtr* fx_tree_list_append_item(ObjectPtr* wgt, ObjectPtr* prt, const char* text) {
        return static_cast<FXTreeList*>(wgt) -> appendItem(static_cast<FXTreeItem*>(prt), text);
    }
    void fx_tree_list_clear_items(ObjectPtr* wgt) {
        static_cast<FXTreeList*>(wgt) -> clearItems();
    }

//~ FXTable
    ObjectPtr* fx_table_new(ObjectPtr* prt) {
        return make_widget<FXTable, FXComposite>(prt);
    }
    void fx_table_set_table_size(ObjectPtr* wgt, int nr, int nc) {
        static_cast<FXTable*>(wgt) -> setTableSize(nr, nc);
    }
    void fx_table_set_item_text(ObjectPtr* wgt, int r, int c, const char* text) {
        static_cast<FXTable*>(wgt) -> setItemText(r, c, text);
    }
    const char* fx_table_get_item_text(ObjectPtr* wgt, int r, int c) {
        static thread_local FXString buffer;
        buffer = static_cast<FXTable*>(wgt) -> getItemText(r, c);
        return buffer.text();
    }

//~ FXCanvas.h
    ObjectPtr* fx_canvas_new(ObjectPtr* prt) {
        return make_widget<FXCanvas, FXComposite>(prt);
    }
    void fx_canvas_set_mouse_callback(ObjectPtr* wgt, long (*cb)(ObjectPtr*, int, int, int, void*), void* ctx) {
        auto canvas = static_cast<FXCanvas*>(wgt);
        auto old = canvas->getTarget();
        if (dynamic_cast<CMouseTarget*>(old)) delete old;
        canvas->setTarget(static_cast<FXObject*>(new CMouseTarget(cb, ctx)));
    }

//~ FXTabBar.h
    ObjectPtr* fx_tab_bar_new(ObjectPtr* prt) {
        return make_widget<FXTabBar, FXComposite>(prt);
    }

//~ FXTabBook
    ObjectPtr* fx_tab_book_new(ObjectPtr* prt) {
        return make_widget<FXTabBook, FXComposite>(prt);
    }

//~ FXTabItem.h
    ObjectPtr* fx_tab_item_new(ObjectPtr* prt, const char* text) {
        return make_widget<FXTabItem, FXTabBar>(prt, text);
    }
    void fx_tab_item_set_text(ObjectPtr* wgt, const char* text) {
        static_cast<FXTabItem*>(wgt) -> setText(text);
    }
    const char* fx_tab_item_get_text(ObjectPtr* wgt) {
        static thread_local FXString buffer;
        buffer = static_cast<FXTabItem*>(wgt) ->getText();
        return string_result(buffer.text());
    }

//~ FXScrollBar
    ObjectPtr* fx_scroll_bar_new(ObjectPtr* prt) {
        return make_widget<FXScrollBar, FXComposite>(prt);
    }
    int fx_scroll_bar_get_position(ObjectPtr* wgt) {
        return static_cast<FXScrollBar*>(wgt) ->getPosition();
    }
    void fx_scroll_bar_set_position(ObjectPtr* wgt, int pos) {
        static_cast<FXScrollBar*>(wgt) ->setPosition(pos);
    }
    void fx_scroll_bar_set_range(ObjectPtr* wgt, int hi) {
        static_cast<FXScrollBar*>(wgt) ->setRange(hi);
    }

//~ FXMenuBar
    ObjectPtr* fx_menu_bar_new(ObjectPtr* prt) {
        return make_widget<FXMenuBar, FXComposite>(prt, nullptr);
    }

//~ FXMenuPane
    ObjectPtr* fx_menu_pane_new(ObjectPtr* prt) {
        return make_widget<FXMenuPane, FXWindow>(prt);
    }

//~ FXMenuButton.h
    ObjectPtr* fx_menu_button_new(ObjectPtr* prt, const char* title, ObjectPtr* pop) {
        auto wgt = make_widget<FXMenuButton, FXComposite>(prt, title);
        ASSERT_NOT_NULL(wgt);
        static_cast<FXMenuButton*>(wgt) -> setMenu(static_cast<FXPopup*>(pop));
        return wgt;
    }
    void fx_menu_button_style(ObjectPtr* wgt, FXuint style) {
        static_cast<FXMenuButton*>(wgt) -> setButtonStyle(style);
    }
    void fx_menu_button_popup_style(ObjectPtr* wgt, FXuint style) {
        static_cast<FXMenuButton*>(wgt) -> setPopupStyle(style);
    }
    void fx_menu_button_attachment(ObjectPtr* wgt, FXuint attachment) {
        static_cast<FXMenuButton*>(wgt) -> setAttachment(attachment);
    }

//~ FXMenuTitle
    ObjectPtr* fx_menu_title_new(ObjectPtr* prt, const char* text, ObjectPtr* pop) {
        auto wgt = make_widget<FXMenuTitle, FXComposite>(prt, text);
        ASSERT_NOT_NULL(wgt);
        static_cast<FXMenuTitle*>(wgt) -> setMenu(static_cast<FXPopup*>(pop));
        return wgt;
    }

//~ FXMenuCaption
    ObjectPtr* fx_menu_caption_new(ObjectPtr* prt, const char* text) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXMenuCaption, FXComposite>(prt, text);
    }

//~ FXMenuCascade
    ObjectPtr* fx_menu_cascade_new(ObjectPtr* prt, const char* text) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXMenuCascade, FXComposite>(prt, text);
    }

//~ FXMenuRadio
    ObjectPtr* fx_menu_radio_new(ObjectPtr* prt, const char* text) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXMenuRadio, FXComposite>(prt, text);
    }
    unsigned char fx_menu_radio_get_check(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXMenuRadio*>(wgt) -> getCheck();
    }
    void fx_menu_radio_set_check(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXMenuRadio*>(wgt) -> setCheck();
    }

//~ FXMenuCheck
    ObjectPtr* fx_menu_check_new(ObjectPtr* prt, const char* text) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXMenuCheck, FXComposite>(prt, text);
    }
    unsigned char fx_menu_check_get_check(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        return static_cast<FXMenuCheck*>(wgt) -> getCheck();
    }
    void fx_menu_check_set_check(ObjectPtr* wgt, unsigned char check) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXMenuCheck*>(wgt) -> setCheck(check);
    }

//~ FXMenuSeparator
    ObjectPtr* fx_menu_separator_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXMenuSeparator, FXComposite>(prt);
    }

//~ FXMenuCommand
    ObjectPtr* fx_menu_command_new(ObjectPtr* prt, const char* text) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXMenuCommand, FXComposite>(prt, text);
    }
    void fx_menu_command_set_accel_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXMenuCommand*>(wgt) -> setAccelText(text);
    }
    const char* fx_menu_command_get_accel_text(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static thread_local FXString buffer;
        buffer = static_cast<FXMenuCommand*>(wgt) -> getAccelText();
        return buffer.text();
    }

//~ FXStatusLine
    ObjectPtr* fx_status_line_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXStatusLine, FXComposite>(prt);
    }
    const char* fx_status_line_get_text(ObjectPtr* wgt) {
        ASSERT_NOT_NULL(wgt);
        static thread_local FXString buffer;
        buffer = static_cast<FXStatusLine*>(wgt) -> getText();
        return string_result(buffer.text());
    }
    void fx_status_line_set_text(ObjectPtr* wgt, const char* text) {
        ASSERT_NOT_NULL(wgt);
        static_cast<FXStatusLine*>(wgt) -> setText(text);
    }

//~ FXStatusBar
    ObjectPtr* fx_status_bar_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXStatusBar, FXComposite>(prt);
    }

//~ FXOption
    ObjectPtr* fx_option_new(ObjectPtr* prt, const char* text) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXOption, FXComposite>(prt, text);
    }

//~ FXOptionMenu
    ObjectPtr* fx_option_menu_new(ObjectPtr* prt) {
        ASSERT_NOT_NULL(prt);
        return make_widget<FXOptionMenu, FXComposite>(prt);
    }
}
