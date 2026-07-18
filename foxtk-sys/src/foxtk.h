#ifndef FOXTK_H
#define FOXTK_H

#ifdef __cplusplus
extern "C" {
#endif

//~ fxdefs.h
    unsigned int fx_rgb(unsigned int r, unsigned int g, unsigned int b);
    unsigned int fx_rgba(unsigned int r, unsigned int g, unsigned int b, unsigned int a);
    unsigned int fx_red_val(unsigned int rgba);
    unsigned int fx_green_val(unsigned int rgba);
    unsigned int fx_blue_val(unsigned int rgba);
    unsigned int fx_alpha_val(unsigned int rgba);

//~ OPAQUE HANDLES
// ABI note: widget constructor entry points return owned FXObject handles (or nullptr
// when the parent/owner argument is missing). Getter functions return borrowed data.

//~ FXObject.h
    typedef void FXObject;
    void FXObject_delete(FXObject* wgt);

//~ FXApp
    typedef struct FXApp FXApp;
    typedef long (*CbTimer)(FXApp* app, void* ctx);
    FXApp * FXApp_new(const char* name, const char* vendor, int argc, char** argv);
    int FXApp_run(FXApp*  self);
    void FXApp_add_timeout(FXApp*  self, CbTimer cb, unsigned int ns, void* ctx);

//~ FXId.h
    typedef struct FXId FXId;
    FXApp* FXId_get_app(const FXId* self);
    #ifdef _WIN32
                void* FXId_get_id(const FXId* self);
    #else
        unsigned long FXId_get_id(const FXId* self);
    #endif

//~ FXComposite.h
    typedef struct FXComposite FXComposite;
    int FXComposite_child_width(const FXComposite* wgt);
    int FXComposite_child_height(const FXComposite* wgt);

//~ FXTriStateButton.h
    typedef struct FXTriStateButton FXTriStateButton;
    FXTriStateButton* FXTriStateButton_new(FXComposite* prt, const char* text1, const char* text2, const char* text3);

//~ FXTreeListBox.h
    typedef struct FXTreeListBox FXTreeListBox;
    FXTreeListBox* FXTreeListBox_new(FXComposite* prt);

//~ FXDriveBox.h
    typedef struct FXDriveBox FXDriveBox;
    FXDriveBox* FXDriveBox_new(FXComposite* prt);

//~ FXDirBox.h
    typedef struct FXDirBox FXDirBox;
    FXDirBox* FXDirBox_new(FXComposite* prt);

//~ FXFileSelector.h
    typedef struct FXFileSelector FXFileSelector;
    FXFileSelector* FXFileSelector_new(FXComposite* prt);

//~ FXFontSelector.h
    typedef struct FXFontSelector FXFontSelector;
    FXFontSelector* FXFontSelector_new(FXComposite* prt);

//~ FXColorSelector.h
    typedef struct FXColorSelector FXColorSelector;
    FXColorSelector* FXColorSelector_new(FXComposite* prt);

//~ FXDrawable.h
    typedef struct FXDrawable FXDrawable;
    int FXDrawable_get_height(const FXDrawable* wgt);
    int FXDrawable_get_width(const FXDrawable* wgt);

//~ FXDCWindow.h
    typedef struct FXDCWindow FXDCWindow;
    FXDCWindow* FXDCWindow_new(FXDrawable* drawable);

//~ FXDC (drawing).h
    typedef struct FXDC FXDC;
    void FXDC_set_foreground(FXDC* self, unsigned int color);
    void FXDC_set_line_width(FXDC* self, int width);
    void FXDC_draw_line(FXDC* self, int x1, int y1, int x2, int y2);
    void FXDC_draw_point(FXDC* self, int x, int y);
    void FXDC_draw_rect(FXDC* self, int x, int y, int w, int h);
    void FXDC_fill_rect(FXDC* self, int x, int y, int w, int h);

//~ FXWindow.h
    typedef struct FXWindow FXWindow;
    typedef long (*CbWidget)(FXWindow* wgt, void* ctx);
    FXWindow* FXWindow_get_parent(const FXWindow* self);
    FXWindow* FXWindow_get_root(const FXWindow* self);
    long FXWindow_has_focus(const FXWindow* self);
    void FXWindow_set_target(FXWindow* self, CbWidget callback, void* context);
    void FXWindow_set_selector(FXWindow* self, int val);
    void FXWindow_set_width(FXWindow* self, int val);
    void FXWindow_set_height(FXWindow* self, int val);
    void FXWindow_set_layout_hints(FXWindow* self, unsigned int val);
    void FXWindow_set_x(FXWindow* self, int x);
    void FXWindow_set_y(FXWindow* self, int y);
    void FXWindow_disable(FXWindow* self);
    void FXWindow_enable(FXWindow* self);

//~ FXChoiceBox.h
    int FXChoiceBox_ask(FXWindow*  owner, unsigned int opts, const char* caption, const char* text, FXObject* icon, const char** choices);

//~ FXDialogBox.h
    typedef struct FXDialogBox FXDialogBox;
    FXDialogBox* FXDialogBox_new(FXWindow*  owner, const char* title);

//~ FXFileDialog.h
    const char* FXFileDialog_get_open_filename(FXWindow* owner, const char* caption, const char* path, const char* patterns, int initial);
    const char* FXFileDialog_get_save_filename(FXWindow* owner, const char* caption, const char* path, const char* patterns, int initial);

//~ FXMessageBox.h
    unsigned int FXMessageBox_error(FXWindow* owner, unsigned int opts, const char* caption, const char* message);
    unsigned int FXMessageBox_warning(FXWindow* owner, unsigned int opts, const char* caption, const char* message);
    unsigned int FXMessageBox_question(FXWindow* owner, unsigned int opts, const char* caption, const char* message);
    unsigned int FXMessageBox_information(FXWindow* owner, unsigned int opts, const char* caption, const char* message);

//~ FXDial.h
    typedef struct FXDial FXDial;
    FXDial* FXDial_new(FXComposite* prt);

//~ FXFrame.h
    typedef struct FXFrame FXFrame;
    void FXFrame_set_frame_style(FXFrame* wgt, unsigned int val);
    void FXFrame_set_pad_bottom(FXFrame* wgt, int pad);
    void FXFrame_set_pad_left(FXFrame* wgt, int pad);
    void FXFrame_set_pad_right(FXFrame* wgt, int pad);
    void FXFrame_set_pad_top(FXFrame* wgt, int pad);
    void FXFrame_set_base_color(FXFrame* wgt, unsigned int color);
    void FXFrame_set_border_color(FXFrame* wgt, unsigned int color);
    void FXFrame_set_hilite_color(FXFrame* wgt, unsigned int color);
    void FXFrame_set_shadow_color(FXFrame* wgt, unsigned int color);

//~ FXKnob.h
    typedef struct FXKnob FXKnob;
    FXKnob* FXKnob_new(FXComposite* prt);
    void FXKnob_set_help_text(FXKnob* wgt, const char* text);
    void FXKnob_set_tip_text(FXKnob* wgt, const char* text);
    void FXKnob_set_value(FXKnob* wgt, int value);
    void FXKnob_set_range(FXKnob* wgt, int lo, int hi);
    void FXKnob_set_increment(FXKnob* wgt, int inc);

#define TextExt(widget)                                                        \
    typedef struct widget widget;                                              \
    const char* widget##_get_text(const widget* self);                         \
    const char* widget##_get_text(const widget* self);                         \
    void widget##_set_text(widget* self, const char* text);                    \
    void widget##_set_help_text(widget* self, const char* text);               \
    void widget##_set_tip_text(widget* self, const char* text);                \
    void widget##_set_text_color(widget* self, unsigned int color);            \
    void widget##_set_font(widget* self, const char* family, int size);        \

//~ FXLabel.h
    TextExt(FXLabel)
    FXLabel* FXLabel_new(FXComposite* prt, const char* title);
    void FXLabel_set_justify(FXLabel* self, unsigned int justify);

//~ FXText.h
    TextExt(FXText)
    FXText* FXText_new(FXComposite* prt);
    void FXText_set_editable(FXText* wgt, long editable);

//~ FXTextField.h
    TextExt(FXTextField)
    FXTextField* FXTextField_new(FXComposite*  frm);
    void FXTextField_set_editable(FXTextField* wgt, long val);

#define RangerExt(widget)                                                      \
    typedef struct widget widget;                                              \
    int widget##_get_increment(const widget *self);                            \
    int widget##_get_value(const widget *self);                                \
    void widget##_get_range(const widget *self, int* lo, int* hi);             \
    void widget##_set_value(widget *self, int value);                          \
    void widget##_set_range(widget *self, int lo, int hi);                     \
    void widget##_set_increment(widget *self, int inc);                        \

//~ FXSlider.h
    RangerExt(FXSlider)
    FXSlider* FXSlider_new(FXComposite *parent);

//~ FXSpinner.h
    RangerExt(FXSpinner)
    FXSpinner* FXSpinner_new(FXComposite *parent);
    void FXSpinner_decrement(FXSpinner *self);

//~ FXProgressBar.h
    typedef struct FXProgressBar FXProgressBar;
    FXProgressBar* FXProgressBar_new(FXComposite* prt);
    unsigned int FXProgressBar_get_progress(const FXProgressBar* wgt);
    unsigned int FXProgressBar_get_total(const FXProgressBar* wgt);
    int FXProgressBar_get_bar_size(const FXProgressBar* wgt);
    void FXProgressBar_set_progress(FXProgressBar* wgt, unsigned int value);
    void FXProgressBar_set_total(FXProgressBar* wgt, unsigned int value);
    void FXProgressBar_set_bar_size(FXProgressBar* wgt, int size);
    void FXProgressBar_increment(FXProgressBar* wgt, unsigned int value);
    void FXProgressBar_show_number(FXProgressBar* wgt);
    void FXProgressBar_hide_number(FXProgressBar* wgt);

//~ FXArrowButton.h
    typedef struct FXArrowButton FXArrowButton;
    FXArrowButton* FXArrowButton_new(FXComposite* parent);
    void FXArrowButton_set_arrow_size(FXArrowButton* self, int size);
    void FXArrowButton_set_arrow_color(FXArrowButton* self, unsigned int color);

//~ FXButton.h
    TextExt(FXButton)
    FXButton* FXButton_new(FXComposite*  parent, const char* title);
    void FXButton_set_state(FXButton* self, unsigned int state);
    void FXButton_set_style(FXButton* self, unsigned int style);

//~ FXCheckButton.h
    typedef struct FXCheckButton FXCheckButton;
    FXCheckButton* FXCheckButton_new(FXComposite* prt, const char* title);
    unsigned char FXCheckButton_get_check(const FXCheckButton* self);
    void FXCheckButton_set_check(FXCheckButton* self, unsigned char check);

//~ FXToggleButton.h
    typedef struct FXToggleButton FXToggleButton;
    FXToggleButton* FXToggleButton_new(FXComposite* prt, const char* text1, const char* text2);

//~ FXRadioButton.h
    typedef struct FXRadioButton FXRadioButton;
    FXRadioButton* FXRadioButton_new(FXComposite* prt, const char* title);
    unsigned char FXRadioButton_get_check(const FXRadioButton* self);
    void FXRadioButton_set_check(FXRadioButton* self);

//~ FXTopWindow.h
    typedef struct FXTopWindow FXTopWindow;
    void FXTopWindow_set_decorations(FXTopWindow* self, unsigned int decorations);
    void FXTopWindow_set_hspacing(FXTopWindow* self, int hspacing);
    void FXTopWindow_set_vspacing(FXTopWindow* self, int vspacing);

//~ FXSplashWindow.h
    typedef struct FXSplashWindow FXSplashWindow;
    FXSplashWindow* FXSplashWindow_new(FXApp* app);

//~ FXMainWindow.h
    typedef struct FXMainWindow FXMainWindow;
    FXMainWindow* FXMainWindow_new(FXApp* app, const char* title, int width, int height);
    void FXMainWindow_show(FXMainWindow* self);

//~ FXPacker.h
    typedef struct FXPacker FXPacker;
    FXPacker* FXPacker_new(FXComposite* prt);
    void FXPacker_set_hspacing(FXPacker* self, int val);
    void FXPacker_set_vspacing(FXPacker* self, int val);

//~ FXMatrix.h
    typedef struct FXMatrix FXMatrix;
    FXMatrix* FXMatrix_new(FXComposite* prt, int rows, unsigned int opts);
    unsigned int FXMatrix_get_matrix_style(const FXMatrix* self);
    int FXMatrix_get_num_rows(const FXMatrix* self);
    int FXMatrix_get_num_columns(const FXMatrix* self);
    void FXMatrix_set_matrix_style(FXMatrix* self, unsigned int style);
    void FXMatrix_set_num_rows(FXMatrix* self, int rows);
    void FXMatrix_set_num_columns(FXMatrix* self, int cols);

//~ FXSplitter.h
    typedef struct FXSplitter FXSplitter;
    FXSplitter* FXSplitter_new(FXComposite* prt, unsigned int opts);
    int FXSplitter_get_split(const FXSplitter* wgt, int index);
    int FXSplitter_get_bar_size(const FXSplitter* wgt);
    unsigned int FXSplitter_get_splitter_style(const FXSplitter* wgt);
    void FXSplitter_set_split(FXSplitter* wgt, int index, int size);
    void FXSplitter_set_splitter_style(FXSplitter* wgt, unsigned int style);
    void FXSplitter_set_bar_size(FXSplitter* wgt, int size);

//~ FXGroupBox.h
    typedef struct FXGroupBox FXGroupBox;
    FXGroupBox* FXGroupBox_new(FXComposite* prt, const char* title);
    void FXGroupBox_set_style(FXGroupBox* wgt, unsigned int val);
    void FXGroupBox_set_text(FXGroupBox* wgt, const char* text);

//~ FXVerticalFrame.h
    typedef struct FXVerticalFrame FXVerticalFrame;
    FXVerticalFrame* FXVerticalFrame_new(FXComposite* prt);

//~ FXHorizontalFrame.h
    typedef struct FXHorizontalFrame FXHorizontalFrame;
    FXHorizontalFrame* FXHorizontalFrame_new(FXComposite* prt);

//~ FXSwitcher.h
    typedef struct FXSwitcher FXSwitcher;
    FXSwitcher* FXSwitcher_new(FXComposite* prt);
    void FXSwitcher_set_current(FXSwitcher* wgt, int index);

//~ FXComboBox.h
    typedef struct FXComboBox FXComboBox;
    FXComboBox* FXComboBox_new(FXComposite* prt, int cols);
    const char* FXComboBox_get_item_text(const FXComboBox* self, int index);
    int FXComboBox_get_num_items(const FXComboBox* self);
    int FXComboBox_get_current_item(const FXComboBox* self);
    int FXComboBox_append_item(FXComboBox* self, const char* text);
    void FXComboBox_clear_items(FXComboBox* self);
    void FXComboBox_set_current_item(FXComboBox* self, int index);
    void FXComboBox_set_num_visible(FXComboBox* self, int nvis);

#define SelectorExt(widget)                                                    \
    typedef struct widget widget;                                              \
    widget* widget##_new(FXComposite* parent);                                 \
    const char* widget##_get_item_text(const widget* self, int index);         \
    int widget##_get_num_items(const widget* self);                            \
    int widget##_get_current_item(const widget* self);                         \
    void widget##_append_item(widget* self, const char* text);                 \
    void widget##_clear_items(widget* self);                                   \
    void widget##_set_current_item(widget* self, int index);                   \
    void widget##_set_num_visible(widget* self, int nvis);                     \

//~ FXList.h
    SelectorExt(FXList)
    void FXList_set_style(FXList* self, unsigned int style);

//~ FXListBox.h
    SelectorExt(FXListBox)

//~ FXTreeList.h
    typedef struct FXTreeItem FXTreeItem;
    typedef struct FXTreeList FXTreeList;
    FXTreeList* FXTreeList_new(FXComposite* prt);
    FXTreeItem* FXTreeList_append_item(FXTreeList* self, FXTreeItem* prt, const char* text);
    void FXTreeList_clear_items(FXTreeList* self);

//~ FXTable.h
    typedef struct FXTable FXTable;
    FXTable* FXTable_new(FXComposite* prt);
    const char* FXTable_get_item_text(const FXTable* self, int r, int c);
    void FXTable_set_table_size(FXTable* self, int nr, int nc);
    void FXTable_set_item_text(FXTable* self, int r, int c, const char* text);

//~ FXCanvas.h
    typedef struct FXCanvas FXCanvas;
    typedef long (*CbMouse)(FXCanvas* widget, int event_code, int x, int y, void* context);
    FXCanvas* FXCanvas_new(FXComposite* prt);
    void FXCanvas_set_mouse_callback(FXCanvas* self, CbMouse cb, void* ctx);

//~ FXTabBar.h
    typedef struct FXTabBar FXTabBar;
    FXTabBar* FXTabBar_new(FXComposite* prt);

//~ FXTabBook.h
    typedef struct FXTabBook FXTabBook;
    FXTabBook* FXTabBook_new(FXComposite* prt);

//~ FXTabItem.h
    typedef struct FXTabItem FXTabItem;
    FXTabItem* FXTabItem_new(FXTabBar* prt, const char* text);
    const char* FXTabItem_get_text(const FXTabItem* wgt);
    void FXTabItem_set_text(FXTabItem* wgt, const char* text);

//~ FXScrollBar.h
    typedef struct FXScrollBar FXScrollBar;
    FXScrollBar* FXScrollBar_new(FXComposite* prt);
    int FXScrollBar_get_position(const FXScrollBar* wgt);
    void FXScrollBar_set_position(FXScrollBar* wgt, int pos);
    void FXScrollBar_set_range(FXScrollBar* wgt, int hi);

//~ FXMenuBar.h
    typedef struct FXMenuBar FXMenuBar;
    FXMenuBar* FXMenuBar_new(FXComposite* prt);

//~ FXMenuButton.h
    typedef struct FXMenuButton FXMenuButton;
    FXMenuButton* FXMenuButton_new(FXComposite* prt, const char* title, FXObject* pop);
    void FXMenuButton_set_style(FXMenuButton* self, unsigned int style);
    void FXMenuButton_set_popup_style(FXMenuButton* self, unsigned int style);
    void FXMenuButton_set_attachment(FXMenuButton* self, unsigned int attachment);

//~ FXMenuCaption.h
    typedef struct FXMenuCaption FXMenuCaption;
    FXMenuCaption* FXMenuCaption_new(FXObject* prt, const char* text);

//~ FXMenuCascade.h
    typedef struct FXMenuCascade FXMenuCascade;
    FXMenuCascade* FXMenuCascade_new(FXObject* prt, const char* text);

//~ FXMenuPane.h
    typedef struct FXMenuPane FXMenuPane;
    FXMenuPane* FXMenuPane_new(FXWindow* prt);

//~ FXMenuTitle.h
    typedef struct FXMenuTitle FXMenuTitle;
    FXMenuTitle* FXMenuTitle_new(FXComposite* prt, const char* text, FXObject* pop);

//~ FXMenuCommand.h
    typedef struct FXMenuCommand FXMenuCommand;
    FXMenuCommand* FXMenuCommand_new(FXComposite* prt, const char* text);
    void FXMenuCommand_set_accel_text(FXMenuCommand* wgt, const char* text);
    const char* FXMenuCommand_get_accel_text(const FXMenuCommand* wgt);

//~ FXMenuSeparator.h
    typedef struct FXMenuSeparator FXMenuSeparator;
    FXMenuSeparator* FXMenuSeparator_new(FXObject* prt);

//~ FXMenuRadio.h
    typedef struct FXMenuRadio FXMenuRadio;
    FXMenuRadio* FXMenuRadio_new(FXComposite* prt, const char* text);
    unsigned char FXMenuRadio_get_check(const FXMenuRadio* wgt);
    void FXMenuRadio_set_check(FXMenuRadio* wgt);

//~ FXMenuCheck.h
    typedef struct FXMenuCheck FXMenuCheck;
    FXMenuCheck* FXMenuCheck_new(FXComposite* prt, const char* text);
    unsigned char FXMenuCheck_get_check(const FXMenuCheck* wgt);
    void FXMenuCheck_set_check(FXMenuCheck* wgt, unsigned char check);

//~ FXStatusLine.h
    typedef struct FXStatusLine FXStatusLine;
    FXStatusLine* FXStatusLine_new(FXObject* prt);
    const char* FXStatusLine_get_text(FXObject* wgt);
    void FXStatusLine_set_text(FXObject* wgt, const char* text);

//~ FXStatusBar.h
    typedef struct FXStatusBar FXStatusBar;
    FXStatusBar* FXStatusBar_new(FXObject* prt);

//~ FXOption.h
    typedef struct FXOption FXOption;
    FXOption* FXOption_new(FXObject* prt, const char* text);

//~ FXOptionMenu.h
    typedef struct FXOptionMenu FXOptionMenu;
    FXOptionMenu* FXOptionMenu_new(FXObject* prt);

#ifdef __cplusplus
}
#endif
#endif
