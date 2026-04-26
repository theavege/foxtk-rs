#ifndef FOXTK_H
#define FOXTK_H

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handles
typedef void* ObjectPtr;
typedef long (*CWidgetCb)(ObjectPtr widget, void* context);
typedef long (*CTimerCb)(ObjectPtr application, void* context);

// FXObject
    void fx_object_delete(ObjectPtr wgt);

// FXId
    ObjectPtr fx_id_get_app(ObjectPtr wgt);

// FXApp
    ObjectPtr fx_app_new(const char* name, const char* vendor, int argc, char** argv);
    int fx_app_run(ObjectPtr app);
    void fx_app_add_timeout(ObjectPtr app, CTimerCb cb, unsigned int ns, void* ctx);
    void fx_app_add_chore(ObjectPtr app, CTimerCb cb, void* ctx);

// FXLabel
    ObjectPtr fx_label_new(ObjectPtr parent, const char* title);
    void fx_label_set_text(ObjectPtr wgt, const char* text);
    const char* fx_label_get_text(ObjectPtr wgt);
    void fx_label_set_justify(ObjectPtr wgt, unsigned int justify);
    unsigned int fx_label_get_justify(ObjectPtr wgt);

// FXWindow
    void fx_window_set_target(ObjectPtr wgt, CWidgetCb callback, void* context);
    void fx_window_set_selector(ObjectPtr wgt, int val);
    void fx_window_set_width(ObjectPtr wgt, int val);
    void fx_window_set_height(ObjectPtr wgt, int val);
    void fx_window_set_layout_hints(ObjectPtr wgt, unsigned int val);
    ObjectPtr fx_window_get_parent(ObjectPtr wgt);

// FXTextField
    ObjectPtr fx_textfield_new(ObjectPtr frm);
    void fx_textfield_set_text(ObjectPtr wgt, const char* text);
    const char* fx_textfield_get_text(ObjectPtr wgt);

// FXSpinner
    ObjectPtr fx_spinner_new(ObjectPtr parent);
    int fx_spinner_get_value(ObjectPtr wgt);
    void fx_spinner_set_value(ObjectPtr wgt, int value);
    void fx_spinner_get_range(ObjectPtr wgt, int* lo, int* hi);
    void fx_spinner_set_range(ObjectPtr wgt, int lo, int hi);
    void fx_spinner_set_increment(ObjectPtr wgt, int inc);
    void fx_spinner_increment(ObjectPtr wgt);
    void fx_spinner_decrement(ObjectPtr wgt);

// FXSlider
    ObjectPtr fx_slider_new(ObjectPtr parent_);
    int fx_slider_get_value(ObjectPtr wgt);
    void fx_slider_set_value(ObjectPtr wgt, int value);
    void fx_slider_get_range(ObjectPtr wgt, int* lo, int* hi);
    void fx_slider_set_range(ObjectPtr wgt, int lo, int hi);
    int fx_slider_get_increment(ObjectPtr wgt);
    void fx_slider_set_increment(ObjectPtr wgt, int inc);

// FXProgressBar
    ObjectPtr fx_progressbar_new(ObjectPtr parent_);
    void fx_progressbar_set_progress(ObjectPtr wgt, unsigned int value);
    unsigned int fx_progressbar_get_progress(ObjectPtr wgt);
    void fx_progressbar_set_total(ObjectPtr wgt, unsigned int value);
    unsigned int fx_progressbar_get_total(ObjectPtr wgt);
    void fx_progressbar_increment(ObjectPtr wgt, unsigned int value);
    void fx_progressbar_show_number(ObjectPtr wgt);
    void fx_progressbar_hide_number(ObjectPtr wgt);
    void fx_progressbar_set_bar_size(ObjectPtr wgt, int size);
    int fx_progressbar_get_bar_size(ObjectPtr wgt);

// FXButton
    ObjectPtr fx_button_new(ObjectPtr parent, const char* title);

// FXRadioButton
    ObjectPtr fx_radio_button_new(ObjectPtr parent_, const char* title);
    unsigned char fx_radio_button_get_check(ObjectPtr wgt);
    void fx_radio_button_set_check(ObjectPtr wgt);

// FXCheckButton
    ObjectPtr fx_check_button_new(ObjectPtr parent, const char* title);
    unsigned char fx_check_button_get_check(ObjectPtr wgt);
    void fx_check_button_set_check(ObjectPtr wgt, unsigned char check);

// FXMainWindow
    ObjectPtr fx_main_window_new(ObjectPtr app, const char* title, int width, int height);
    void fx_main_window_show(ObjectPtr wgt_);

// FXPacker
    ObjectPtr fx_packer_new(ObjectPtr parent);
    void fx_packer_set_hspacing(ObjectPtr wgt, int val);
    void fx_packer_set_vspacing(ObjectPtr wgt, int val);

// FXGroupBox
    ObjectPtr fx_groupbox_new(ObjectPtr parent, const char* title);
    void fx_groupbox_set_style(ObjectPtr wgt, unsigned int val);

// FXSpring
    ObjectPtr fx_spring_new(ObjectPtr parent);

// FXVerticalFrame
    ObjectPtr fx_vertical_frame_new(ObjectPtr parent);

// FXHorizontalFrame
    ObjectPtr fx_horizontal_frame_new(ObjectPtr parent);

// FXSwitcher
    ObjectPtr fx_switcher_new(ObjectPtr parent);

    /// Bring the child window at index to the top
    void fx_switcher_set_current(ObjectPtr wgt, int index);

// FXComboBox
    ObjectPtr fx_combo_box_new(ObjectPtr parent_, int cols);
    void fx_combo_box_append_item(ObjectPtr wgt, const char* text);
    void fx_combo_box_clear_items(ObjectPtr wgt);
    int fx_combo_box_get_current_item(ObjectPtr wgt);
    void fx_combo_box_set_current_item(ObjectPtr wgt, int index);
    const char* fx_combo_box_get_item_text(ObjectPtr wgt, int index);
    int fx_combo_box_get_num_items(ObjectPtr wgt);

// FXList
    ObjectPtr fx_list_new(ObjectPtr parent);
    void fx_list_append_item(ObjectPtr wgt, const char* text);
    void fx_list_clear_items(ObjectPtr wgt);
    int fx_list_get_current_item(ObjectPtr wgt);
    void fx_list_set_current_item(ObjectPtr wgt, int index);
    const char* fx_list_get_item_text(ObjectPtr wgt, int index);
    int fx_list_get_num_items(ObjectPtr wgt);


// FXListBox
    ObjectPtr fx_list_box_new(ObjectPtr parent);
    void fx_list_box_append_item(ObjectPtr wgt, const char* text);
    void fx_list_box_clear_items(ObjectPtr wgt);
    int fx_list_box_get_current_item(ObjectPtr wgt);
    void fx_list_box_set_current_item(ObjectPtr wgt, int index);
    const char* fx_list_box_get_item_text(ObjectPtr wgt, int index);
    int fx_list_box_get_num_items(ObjectPtr wgt);

// FXText
    ObjectPtr fx_text_new(ObjectPtr parent);
    void fx_text_set_text(ObjectPtr wgt, const char* text);
    const char* fx_text_get_text(ObjectPtr wgt);

// FXTreeList
    ObjectPtr fx_tree_list_new(ObjectPtr parent);
    ObjectPtr fx_tree_list_append_item(ObjectPtr wgt, ObjectPtr parent_item, const char* text, void* openicon, void* closedicon, void* ptr);
    void fx_tree_list_clear_items(ObjectPtr wgt);

// FXTable
    ObjectPtr fx_table_new(ObjectPtr parent);
    void fx_table_set_table_size(ObjectPtr wgt, int nr, int nc);
    void fx_table_set_item_text(ObjectPtr wgt, int r, int c, const char* text);
    const char* fx_table_get_item_text(ObjectPtr wgt, int r, int c);

// FXCanvas
    ObjectPtr fx_canvas_new(ObjectPtr parent);

// FXTabBook
    ObjectPtr fx_tab_book_new(ObjectPtr parent);
    ObjectPtr fx_tab_item_new(ObjectPtr parent_, const char* text);

// FXScrollBar
    ObjectPtr fx_scroll_bar_new(ObjectPtr parent);
    int fx_scroll_bar_get_position(ObjectPtr wgt);
    void fx_scroll_bar_set_position(ObjectPtr wgt, int pos);
    void fx_scroll_bar_set_range(ObjectPtr wgt, int hi);

// FXMenuBar
    ObjectPtr fx_menu_bar_new(ObjectPtr parent);
    ObjectPtr fx_menu_pane_new(ObjectPtr parent);
    ObjectPtr fx_menu_title_new(ObjectPtr parent, const char* text, ObjectPtr pop);
    ObjectPtr fx_menu_command_new(ObjectPtr parent_, const char* text);

#ifdef __cplusplus
}
#endif
#endif
