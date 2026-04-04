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

// FXSliderExt
    ObjectPtr fx_slider_new(ObjectPtr parent_, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);
    int fx_slider_get_value(ObjectPtr wgt);
    void fx_slider_set_value(ObjectPtr wgt, int value);
    void fx_slider_get_range(ObjectPtr wgt, int* lo, int* hi);
    void fx_slider_set_range(ObjectPtr wgt, int lo, int hi);
    int fx_slider_get_increment(ObjectPtr wgt);
    void fx_slider_set_increment(ObjectPtr wgt, int inc);

// FXProgressBarExt
    ObjectPtr fx_progressbar_new(ObjectPtr parent_, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);
    void fx_progressbar_set_progress(ObjectPtr wgt, unsigned int value);
    unsigned int fx_progressbar_get_progress(ObjectPtr wgt);
    void fx_progressbar_set_total(ObjectPtr wgt, unsigned int value);
    unsigned int fx_progressbar_get_total(ObjectPtr wgt);
    void fx_progressbar_increment(ObjectPtr wgt, unsigned int value);
    void fx_progressbar_show_number(ObjectPtr wgt);
    void fx_progressbar_hide_number(ObjectPtr wgt);
    void fx_progressbar_set_bar_size(ObjectPtr wgt, int size);
    int fx_progressbar_get_bar_size(ObjectPtr wgt);

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

// FXComboBoxExt
    ObjectPtr fx_combo_box_new(ObjectPtr parent_, int cols, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);
    void fx_combo_box_append_item(ObjectPtr wgt, const char* text, void* ptr);
    void fx_combo_box_clear_items(ObjectPtr wgt);
    int fx_combo_box_get_current_item(ObjectPtr wgt);
    void fx_combo_box_set_current_item(ObjectPtr wgt, int index);
    const char* fx_combo_box_get_item_text(ObjectPtr wgt, int index);
    int fx_combo_box_get_num_items(ObjectPtr wgt);

// FXListBoxExt
    ObjectPtr fx_list_box_new(ObjectPtr parent_, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);
    void fx_list_box_append_item(ObjectPtr wgt, const char* text, void* ptr);
    void fx_list_box_clear_items(ObjectPtr wgt);
    int fx_list_box_get_current_item(ObjectPtr wgt);
    void fx_list_box_set_current_item(ObjectPtr wgt, int index);
    const char* fx_list_box_get_item_text(ObjectPtr wgt, int index);
    int fx_list_box_get_num_items(ObjectPtr wgt);

// FXTextExt
    ObjectPtr fx_text_new(ObjectPtr parent_, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);
    void fx_text_set_text(ObjectPtr wgt, const char* text);
    const char* fx_text_get_text(ObjectPtr wgt);

// FXTreeListExt
    ObjectPtr fx_tree_list_new(ObjectPtr parent_, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);
    ObjectPtr fx_tree_list_append_item(ObjectPtr wgt, ObjectPtr parent_item, const char* text, void* openicon, void* closedicon, void* ptr);
    void fx_tree_list_clear_items(ObjectPtr wgt);

// FXLabelExt
    ObjectPtr fx_label_new(ObjectPtr parent_, const char* text, ObjectPtr icon, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);

// FXTableExt
    ObjectPtr fx_table_new(ObjectPtr parent_, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);
    void fx_table_set_table_size(ObjectPtr wgt, int nr, int nc);
    void fx_table_set_item_text(ObjectPtr wgt, int r, int c, const char* text);
    const char* fx_table_get_item_text(ObjectPtr wgt, int r, int c);

// FXCanvasExt
    ObjectPtr fx_canvas_new(ObjectPtr parent_, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);

// FXTabBookExt
    ObjectPtr fx_tab_book_new(ObjectPtr parent_, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);
    ObjectPtr fx_tab_item_new(ObjectPtr parent_, const char* text, ObjectPtr icon, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);

// FXScrollBarExt
    ObjectPtr fx_scroll_bar_new(ObjectPtr parent_, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb);
    int fx_scroll_bar_get_position(ObjectPtr wgt);
    void fx_scroll_bar_set_position(ObjectPtr wgt, int pos);
    void fx_scroll_bar_set_range(ObjectPtr wgt, int lo, int hi);

// FXMenuBarExt
    ObjectPtr fx_menu_bar_new(ObjectPtr parent_, ObjectPtr tgt, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb, int hs, int vs, int d1, int d2, int d3, int d4, int d5, int d6);
    ObjectPtr fx_menu_pane_new(ObjectPtr parent_, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb, int dummy);
    ObjectPtr fx_menu_title_new(ObjectPtr parent_, const char* text, ObjectPtr icon, ObjectPtr pup, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb, int dummy);
    ObjectPtr fx_menu_command_new(ObjectPtr parent_, const char* text, ObjectPtr icon, ObjectPtr tgt, int sel, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb, int dummy);

#ifdef __cplusplus
}
#endif
