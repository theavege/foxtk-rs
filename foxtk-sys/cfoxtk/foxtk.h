#ifdef __cplusplus
extern "C" {
#endif

// Opaque handles
typedef void* ObjectPtr;
typedef long (*CWidgetCb)(ObjectPtr widget, void* context);
typedef long (*CTimerCb)(ObjectPtr application, void* context);

// FXIdExt
    ObjectPtr fx_id_get_app(ObjectPtr wgt);

// FXAppExt
    ObjectPtr fx_app_new(const char* name, const char* vendor, int argc, char** argv);
    int fx_app_run(ObjectPtr app);
    ObjectPtr fx_app_add_timeout(ObjectPtr app, CTimerCb cb, unsigned int ns, void* ctx);

// FXLabelExt
    void fx_label_set_text(ObjectPtr wgt, const char* text);
    const char* fx_label_get_text(ObjectPtr wgt);
    void fx_label_set_justify(ObjectPtr wgt, unsigned int justify);
    unsigned int fx_label_get_justify(ObjectPtr wgt);

// FXWindowExt
    void fx_window_set_target(ObjectPtr wgt, CWidgetCb callback, void* context);

// FXTextFieldExt
    ObjectPtr fx_textfield_new(ObjectPtr frm, int ncols);
    void fx_textfield_set_text(ObjectPtr wgt, const char* text);
    const char* fx_textfield_get_text(ObjectPtr wgt);

// FXSpinnerExt
    ObjectPtr fx_spinner_new(ObjectPtr parent_, int cols, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);
    int fx_spinner_get_value(ObjectPtr wgt);
    void fx_spinner_set_value(ObjectPtr wgt, int value);
    void fx_spinner_get_range(ObjectPtr wgt, int* lo, int* hi);
    void fx_spinner_set_range(ObjectPtr wgt, int lo, int hi);
    void fx_spinner_set_increment(ObjectPtr wgt, int inc);
    void fx_spinner_increment(ObjectPtr wgt);
    void fx_spinner_decrement(ObjectPtr wgt);

// FXButtonExt
    ObjectPtr fx_button_new(ObjectPtr parent_, const char* title);

// FXRadioButtonExt
    ObjectPtr fx_radio_button_new(ObjectPtr parent_, const char* title);
    unsigned char fx_radio_button_get_check(ObjectPtr wgt);
    void fx_radio_button_set_check(ObjectPtr wgt);

// FXCheckButtonExt
    ObjectPtr fx_check_button_new(ObjectPtr parent_, const char* title);
    unsigned char fx_check_button_get_check(ObjectPtr wgt);
    void fx_check_button_set_check(ObjectPtr wgt, unsigned char check);

// FXMainWindowExt
    ObjectPtr fx_main_window_new(ObjectPtr app, const char* title, int width, int height);
    void fx_main_window_show(ObjectPtr wgt_);

// FXVerticalFrameExt
    ObjectPtr fx_vertical_frame_new(ObjectPtr parent_);

// FXHorizontalFrameExt
    ObjectPtr fx_horizontal_frame_new(ObjectPtr parent_);

#ifdef __cplusplus
}
#endif
