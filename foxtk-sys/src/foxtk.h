#ifndef FOXTK_H
#define FOXTK_H

#ifdef __cplusplus
extern "C" {
#endif

//~ OPAQUE HANDLES
    typedef unsigned int FXuint;
    typedef void        ObjectPtr;
    typedef long         (*CWidgetCb)(ObjectPtr*  widget, void* context);
    typedef long         (*CTimerCb)(ObjectPtr*  application, void* context);

    unsigned int fx_rgb(unsigned int r, unsigned int g, unsigned int b);
    unsigned int fx_rgba(unsigned int r, unsigned int g, unsigned int b, unsigned int a);
    unsigned int fx_red_val(unsigned int rgba);
    unsigned int fx_green_val(unsigned int rgba);
    unsigned int fx_blue_val(unsigned int rgba);
    unsigned int fx_alpha_val(unsigned int rgba);

//~ FXObject
    void fx_object_delete(ObjectPtr*  wgt);

//~ FXId
    ObjectPtr* fx_id_get_app(ObjectPtr*  wgt);
    unsigned long fx_id_get_id(ObjectPtr* wgt);

//~ FXApp
    ObjectPtr* fx_app_new(const char* name, const char* vendor, int argc, char** argv);
    int fx_app_run(ObjectPtr*  app);
    void fx_app_add_timeout(ObjectPtr*  app, CTimerCb cb, unsigned int ns, void* ctx);
    void fx_app_add_chore(ObjectPtr*  app, CTimerCb cb, void* ctx);

//~ FXMessageBox.h
    unsigned int fx_message_box_error(ObjectPtr*  owner, unsigned int opts, const char* caption, const char* message);
    unsigned int fx_message_box_warning(ObjectPtr*  owner, unsigned int opts, const char* caption, const char* message);
    unsigned int fx_message_box_question(ObjectPtr*  owner, unsigned int opts, const char* caption, const char* message);
    unsigned int fx_message_box_information(ObjectPtr*  owner, unsigned int opts, const char* caption, const char* message);

//~ FXChoiceBox.h
    int fx_choice_box_ask(ObjectPtr*  app, unsigned int opts, const char* caption, const char* text, ObjectPtr* icon, const char** choices);

//~ FXTreeListBox.h
    ObjectPtr* fx_tree_list_box_box_new(ObjectPtr*  prt);

//~ FXDriveBox.h
    ObjectPtr* fx_drive_box_new(ObjectPtr*  prt);

//~ FXDirBox.h
    ObjectPtr* fx_dir_box_new(ObjectPtr*  prt);

//~ FXDialogBox.h
    ObjectPtr* fx_dialog_box_new(ObjectPtr*  owner, const char* title);

//~ FXFileDialog.h
    const char* fx_file_dialog_get_open_filename(ObjectPtr* owner, const char* caption, const char* path, const char* patterns, int initial);
    const char* fx_file_dialog_get_save_filename(ObjectPtr* owner, const char* caption, const char* path, const char* patterns, int initial);

//~ FXDrawable.h
    int fx_drawable_get_height(ObjectPtr* wgt);
    int fx_drawable_get_width(ObjectPtr* wgt);

//~ FXDCWindow
    ObjectPtr* fx_dc_window_new(ObjectPtr* drawable);

//~ FXDC (drawing)
    void fx_dc_set_foreground(ObjectPtr* dc, unsigned int color);
    void fx_dc_set_line_width(ObjectPtr* dc, int width);
    void fx_dc_draw_line(ObjectPtr* dc, int x1, int y1, int x2, int y2);
    void fx_dc_draw_point(ObjectPtr* dc, int x, int y);
    void fx_dc_draw_rect(ObjectPtr* dc, int x, int y, int w, int h);
    void fx_dc_fill_rect(ObjectPtr* dc, int x, int y, int w, int h);

//~ FXWindow
    ObjectPtr* fx_window_get_parent(ObjectPtr* wgt);
    ObjectPtr* fx_window_get_root(ObjectPtr* wgt);
    long fx_window_has_focus(ObjectPtr* wgt);
    void fx_window_set_target(ObjectPtr*  wgt, CWidgetCb callback, void* context);
    void fx_window_set_selector(ObjectPtr*  wgt, int val);
    void fx_window_set_width(ObjectPtr*  wgt, int val);
    void fx_window_set_height(ObjectPtr*  wgt, int val);
    void fx_window_set_layout_hints(ObjectPtr*  wgt, unsigned int val);
    void fx_window_set_x(ObjectPtr* wgt, int x);
    void fx_window_set_y(ObjectPtr* wgt, int y);
    void fx_window_disable(ObjectPtr*  wgt);
    void fx_window_enable(ObjectPtr*  wgt);

//~ FXComposite.h
    int fx_composite_child_width(ObjectPtr* wgt);
    int fx_composite_child_height(ObjectPtr* wgt);

//~ FXDial.h
    ObjectPtr* fx_dial_new(ObjectPtr*  prt);

//~ FXFrame
    void fx_frame_set_frame_style(ObjectPtr*  wgt, unsigned int val);
    void fx_frame_set_pad_bottom(ObjectPtr*  wgt, int pad);
    void fx_frame_set_pad_left(ObjectPtr*  wgt, int pad);
    void fx_frame_set_pad_right(ObjectPtr*  wgt, int pad);
    void fx_frame_set_pad_top(ObjectPtr*  wgt, int pad);
    void fx_frame_set_base_color(ObjectPtr*  wgt, unsigned int color);
    void fx_frame_set_border_color(ObjectPtr*  wgt, unsigned int color);
    void fx_frame_set_hilite_color(ObjectPtr* wgt, unsigned int color);
    void fx_frame_set_shadow_color(ObjectPtr* wgt, unsigned int color);

//~ FXKnob.h
    ObjectPtr* fx_knob_new(ObjectPtr* prt);
    void fx_knob_set_help_text(ObjectPtr* wgt, const char* text);
    void fx_knob_set_tip_text(ObjectPtr* wgt, const char* text);
    void fx_knob_set_value(ObjectPtr* wgt, int value);
    void fx_knob_set_range(ObjectPtr* wgt, int lo, int hi);
    void fx_knob_set_increment(ObjectPtr* wgt, int inc);

//~ FXLabel
    ObjectPtr* fx_label_new(ObjectPtr*  prt, const char* title);
    const char* fx_label_get_text(ObjectPtr* wgt);
    void fx_label_set_text(ObjectPtr*  wgt, const char* text);
    void fx_label_set_help_text(ObjectPtr*  wgt, const char* text);
    void fx_label_set_tip_text(ObjectPtr*  wgt, const char* text);
    void fx_label_set_justify(ObjectPtr*  wgt, unsigned int justify);
    void fx_label_set_text_color(ObjectPtr*  wgt, unsigned int color);

//~ FXText.h
    ObjectPtr* fx_text_new(ObjectPtr*  prt);
    const char* fx_text_get_text(ObjectPtr*  wgt);
    void fx_text_set_text(ObjectPtr*  wgt, const char* text);
    void fx_text_set_help_text(ObjectPtr*  wgt, const char* text);
    void fx_text_set_tip_text(ObjectPtr*  wgt, const char* text);
    void fx_text_set_editable(ObjectPtr*  wgt, long editable);
    void fx_text_set_font(ObjectPtr*  wgt, const char* family, int size);

//~ FXTextField
    ObjectPtr* fx_textfield_new(ObjectPtr*  frm);
    const char* fx_textfield_get_text(ObjectPtr*  wgt);
    void fx_textfield_set_text(ObjectPtr*  wgt, const char* text);
    void fx_textfield_set_help_text(ObjectPtr*  wgt, const char* text);
    void fx_textfield_set_tip_text(ObjectPtr*  wgt, const char* text);
    void fx_textfield_set_editable(ObjectPtr*  wgt, long val);
    void fx_textfield_set_text_color(ObjectPtr*  wgt, unsigned int color);

//~ FXSpinner
    ObjectPtr* fx_spinner_new(ObjectPtr*  prt);
    int fx_spinner_get_value(ObjectPtr*  wgt);
    void fx_spinner_set_value(ObjectPtr*  wgt, int value);
    void fx_spinner_get_range(ObjectPtr*  wgt, int* lo, int* hi);
    void fx_spinner_set_range(ObjectPtr*  wgt, int lo, int hi);
    void fx_spinner_set_increment(ObjectPtr*  wgt, int inc);
    void fx_spinner_increment(ObjectPtr*  wgt);
    void fx_spinner_decrement(ObjectPtr*  wgt);

//~ FXSlider
    ObjectPtr* fx_slider_new(ObjectPtr*  prt);
    int fx_slider_get_increment(ObjectPtr*  wgt);
    int fx_slider_get_value(ObjectPtr*  wgt);
    void fx_slider_set_value(ObjectPtr*  wgt, int value);
    void fx_slider_get_range(ObjectPtr*  wgt, int* lo, int* hi);
    void fx_slider_set_range(ObjectPtr*  wgt, int lo, int hi);
    void fx_slider_set_increment(ObjectPtr*  wgt, int inc);

//~ FXProgressBar
    ObjectPtr* fx_progressbar_new(ObjectPtr*  prt);
    unsigned int fx_progressbar_get_progress(ObjectPtr*  wgt);
    unsigned int fx_progressbar_get_total(ObjectPtr*  wgt);
    int fx_progressbar_get_bar_size(ObjectPtr*  wgt);
    void fx_progressbar_set_progress(ObjectPtr*  wgt, unsigned int value);
    void fx_progressbar_set_total(ObjectPtr*  wgt, unsigned int value);
    void fx_progressbar_increment(ObjectPtr*  wgt, unsigned int value);
    void fx_progressbar_show_number(ObjectPtr*  wgt);
    void fx_progressbar_hide_number(ObjectPtr*  wgt);
    void fx_progressbar_set_bar_size(ObjectPtr*  wgt, int size);

//~ FXArrowButton.h
    ObjectPtr* fx_arrow_button_new(ObjectPtr*  prt);
    void fx_arrow_button_set_arrow_size(ObjectPtr*  wgt, int size);
    void fx_arrow_button_set_arrow_color(ObjectPtr*  wgt, unsigned int color);

//~ FXButton.h
    ObjectPtr* fx_button_new(ObjectPtr*  prt, const char* title);
    void fx_button_set_state(ObjectPtr* wgt, unsigned int state);
    void fx_button_set_style(ObjectPtr* wgt, unsigned int style);

//~ FXButton.h
    ObjectPtr* fx_button_new(ObjectPtr*  prt, const char* title);

//~ FXCheckButton
    ObjectPtr* fx_check_button_new(ObjectPtr*  prt, const char* title);
    unsigned char fx_check_button_get_check(ObjectPtr*  wgt);
    void fx_check_button_set_check(ObjectPtr*  wgt, unsigned char check);

//~ FXToggleButton.h
    ObjectPtr* fx_toggle_button_new(ObjectPtr*  prt, const char* text1, const char* text2);

//~ FXRadioButton
    ObjectPtr* fx_radio_button_new(ObjectPtr*  prt, const char* title);
    unsigned char fx_radio_button_get_check(ObjectPtr*  wgt);
    void fx_radio_button_set_check(ObjectPtr*  wgt);

//~ FXTopWindow.h
    ObjectPtr* fx_top_window_new(ObjectPtr*  app, const char* title, int width, int height);
    void fx_top_window_set_decorations(ObjectPtr* wgt, unsigned int decorations);
    void fx_top_window_set_hspacing(ObjectPtr* wgt, int hspacing);
    void fx_top_window_set_vspacing(ObjectPtr* wgt, int vspacing);

//~ FXSplashWindow.h
    ObjectPtr* fx_splash_window_new(ObjectPtr* app);

//~ FXToolBarShell.h
    ObjectPtr* fx_tool_bar_shell_new(ObjectPtr* owner);

//~ FXRootWindow.h
    ObjectPtr* fx_root_window_new(ObjectPtr* app);

//~ FXShell.h
    ObjectPtr* fx_shell_new(ObjectPtr* owner, unsigned int opts, int x, int y, int w, int h);

//~ FXMainWindow.h
    ObjectPtr* fx_main_window_new(ObjectPtr*  app, const char* title, int width, int height);
    void fx_main_window_show(ObjectPtr*  wgt_);

//~ FXPacker
    ObjectPtr* fx_packer_new(ObjectPtr*  prt);
    void fx_packer_set_hspacing(ObjectPtr*  wgt, int val);
    void fx_packer_set_vspacing(ObjectPtr*  wgt, int val);

//~ FXMatrix.h
    ObjectPtr* fx_matrix_new(ObjectPtr*  prt, int rows, unsigned int opts);
    void fx_matrix_set_matrix_style(ObjectPtr*  wgt, unsigned int style);
    void fx_matrix_set_num_rows(ObjectPtr*  wgt, int rows);
    void fx_matrix_set_num_columns(ObjectPtr*  wgt, int cols);
    unsigned int fx_matrix_get_matrix_style(ObjectPtr*  wgt);
    int fx_matrix_get_num_rows(ObjectPtr*  wgt);
    int fx_matrix_get_num_columns(ObjectPtr*  wgt);

//~ FXSplitter.h
    ObjectPtr* fx_splitter_new(ObjectPtr* prt, unsigned int opts);
    ObjectPtr* fx_splitter_new_with_target(ObjectPtr* prt, ObjectPtr* target, int selector, unsigned int opts);
    int fx_splitter_get_split(ObjectPtr* wgt, int index);
    void fx_splitter_set_split(ObjectPtr* wgt, int index, int size);
    void fx_splitter_set_splitter_style(ObjectPtr* wgt, unsigned int style);
    unsigned int fx_splitter_get_splitter_style(ObjectPtr* wgt);
    void fx_splitter_set_bar_size(ObjectPtr* wgt, int size);
    int fx_splitter_get_bar_size(ObjectPtr* wgt);

//~ FXGroupBox.h
    ObjectPtr* fx_groupbox_new(ObjectPtr*  prt, const char* title);
    void fx_groupbox_set_style(ObjectPtr*  wgt, unsigned int val);
    void fx_groupbox_set_text(ObjectPtr*  wgt, const char* text);

//~ FXSpring.h
    ObjectPtr* fx_spring_new(ObjectPtr*  prt);

//~ FXVerticalFrame.h
    ObjectPtr* fx_vertical_frame_new(ObjectPtr*  prt);

//~ FXHorizontalFrame.h
    ObjectPtr* fx_horizontal_frame_new(ObjectPtr*  prt);

//~ FXSwitcher.h
    ObjectPtr* fx_switcher_new(ObjectPtr*  prt);
    void fx_switcher_set_current(ObjectPtr*  wgt, int index);

//~ FXComboBox.h
    ObjectPtr* fx_combo_box_new(ObjectPtr*  prt, int cols);
    const char* fx_combo_box_get_item_text(ObjectPtr*  wgt, int index);
    int fx_combo_box_get_num_items(ObjectPtr*  wgt);
    int fx_combo_box_get_current_item(ObjectPtr*  wgt);
    void fx_combo_box_append_item(ObjectPtr*  wgt, const char* text);
    void fx_combo_box_clear_items(ObjectPtr*  wgt);
    void fx_combo_box_set_current_item(ObjectPtr*  wgt, int index);
    void fx_combo_box_set_num_visible(ObjectPtr* wgt, int nvis);

//~ FXList.h
    ObjectPtr* fx_list_new(ObjectPtr*  prt);
    const char* fx_list_get_item_text(ObjectPtr*  wgt, int index);
    int fx_list_get_num_items(ObjectPtr*  wgt);
    int fx_list_get_current_item(ObjectPtr*  wgt);
    void fx_list_append_item(ObjectPtr*  wgt, const char* text);
    void fx_list_set_style(ObjectPtr* wgt, unsigned int style);
    void fx_list_clear_items(ObjectPtr*  wgt);
    void fx_list_set_current_item(ObjectPtr*  wgt, int index);
    void fx_list_set_num_visible(ObjectPtr* wgt, int nvis);

//~ FXListBox.h
    ObjectPtr* fx_list_box_new(ObjectPtr*  prt);
    const char* fx_list_box_get_item_text(ObjectPtr*  wgt, int index);
    int fx_list_box_get_num_items(ObjectPtr*  wgt);
    int fx_list_box_get_current_item(ObjectPtr*  wgt);
    void fx_list_box_append_item(ObjectPtr*  wgt, const char* text);
    void fx_list_box_clear_items(ObjectPtr*  wgt);
    void fx_list_box_set_current_item(ObjectPtr*  wgt, int index);
    void fx_list_box_set_num_visible(ObjectPtr*  wgt, int nvis);

//~ FXText.h
    ObjectPtr* fx_text_new(ObjectPtr*  prt);
    const char* fx_text_get_text(ObjectPtr*  wgt);
    void fx_text_set_text(ObjectPtr*  wgt, const char* text);

//~ FXTreeList.h
    ObjectPtr* fx_tree_list_new(ObjectPtr*  prt);
    ObjectPtr* fx_tree_list_append_item(ObjectPtr*  wgt, ObjectPtr* prt, const char* text);
    void fx_tree_list_clear_items(ObjectPtr*  wgt);

//~ FXTable.h
    ObjectPtr* fx_table_new(ObjectPtr*  prt);
    const char* fx_table_get_item_text(ObjectPtr*  wgt, int r, int c);
    void fx_table_set_table_size(ObjectPtr*  wgt, int nr, int nc);
    void fx_table_set_item_text(ObjectPtr*  wgt, int r, int c, const char* text);

//~ FXCanvas.h
    ObjectPtr* fx_canvas_new(ObjectPtr*  prt);

//~ FXCanvas mouse callback
    typedef long (*CMouseCb)(ObjectPtr* widget, int selector, int x, int y, void* context);
    void fx_canvas_set_mouse_callback(ObjectPtr* wgt, CMouseCb cb, void* ctx);

//~ FXTabBar.h
    ObjectPtr* fx_tab_bar_new(ObjectPtr*  prt);

//~ FXTabBook.h
    ObjectPtr* fx_tab_book_new(ObjectPtr*  prt);
    ObjectPtr* fx_tab_item_new(ObjectPtr*  prt, const char* text);

//~ FXTabItem.h
    void fx_tab_item_set_text(ObjectPtr*  wgt, const char* text);
    const char* fx_tab_item_get_text(ObjectPtr*  wgt);

//~ FXScrollBar.h
    ObjectPtr* fx_scroll_bar_new(ObjectPtr*  prt);
    int fx_scroll_bar_get_position(ObjectPtr*  wgt);
    void fx_scroll_bar_set_position(ObjectPtr*  wgt, int pos);
    void fx_scroll_bar_set_range(ObjectPtr*  wgt, int hi);

//~ FXMenuBar.h
    ObjectPtr* fx_menu_bar_new(ObjectPtr*  prt);

//~ FXMenuButton.h
    ObjectPtr* fx_menu_button_new(ObjectPtr*  prt, const char* title, ObjectPtr* pop);
    void fx_menu_button_style(ObjectPtr*  wgt, FXuint style);
    void fx_menu_button_popup_style(ObjectPtr*  wgt, FXuint style);
    void fx_menu_button_attachment(ObjectPtr*  wgt, FXuint attachment);

//~ FXMenuCaption.h
    ObjectPtr* fx_menu_caption_new(ObjectPtr*  prt, const char* text);

//~ FXMenuCascade.h
    ObjectPtr* fx_menu_cascade_new(ObjectPtr*  prt, const char* text);

//~ FXMenuPane.h
    ObjectPtr* fx_menu_pane_new(ObjectPtr*  prt);

//~ FXMenuTitle.h
    ObjectPtr* fx_menu_title_new(ObjectPtr*  prt, const char* text, ObjectPtr* pop);

//~ FXMenuCommand
    ObjectPtr* fx_menu_command_new(ObjectPtr*  prt, const char* text);

//~ FXMenuSeparator
    ObjectPtr* fx_menu_separator_new(ObjectPtr*  prt);

//~ FXMenuRadio
    ObjectPtr* fx_menu_radio_new(ObjectPtr*  prt, const char* text);
    unsigned char fx_menu_radio_get_check(ObjectPtr*  wgt);
    void fx_menu_radio_set_check(ObjectPtr*  wgt);

//~ FXMenuCheck
    ObjectPtr* fx_menu_check_new(ObjectPtr*  prt, const char* text);

//~ FXStatusLine
    ObjectPtr* fx_status_line_new(ObjectPtr*  prt);
    const char* fx_status_line_get_text(ObjectPtr*  wgt);
    void fx_status_line_set_text(ObjectPtr*  wgt, const char* text);

//~ FXStatusBar
    ObjectPtr* fx_status_bar_new(ObjectPtr*  prt);

//~ FXOption
    ObjectPtr* fx_option_new(ObjectPtr*  prt, const char* text);

//~ FXOptionMenu
    ObjectPtr* fx_option_menu_new(ObjectPtr*  prt);

#ifdef __cplusplus
}
#endif
#endif
