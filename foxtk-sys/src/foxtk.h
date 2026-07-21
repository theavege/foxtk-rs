#ifndef FOXTK_H
#define FOXTK_H
#define EXT_TEXT(widget)                                                       \
  const char* widget##_get_text(const widget* self);                           \
  void widget##_set_text(widget* self, const char* text);                      \
  void widget##_set_text_color(widget* self, unsigned color);                  \
  void widget##_set_font(widget* self, const char* family, int size);

#define EXT_HELP(widget)                                                       \
  const char* widget##_get_help_text(const widget* self);                      \
  const char* widget##_get_tip_text(const widget* self);                       \
  void widget##_set_help_text(widget* self, const char* text);                 \
  void widget##_set_tip_text(widget* self, const char* text);

#define EXT_CHECK(widget)                                                      \
  unsigned char widget##_get_check(const widget* self);                        \
  void widget##_set_check(widget* self, unsigned char check);

#define EXT_EDITABLE(widget)                                                   \
  unsigned char widget##_is_editable(const widget* self);                      \
  void widget##_set_editable(widget* self, unsigned char editable);

#define EXT_STATE(widget)                                                      \
  unsigned widget##_get_state(const widget* self);                             \
  void widget##_set_state(widget* self, unsigned check);

#define EXT_JUSTIFY(widget)                                                    \
  unsigned widget##_get_justify(const widget* self);                           \
  void widget##_set_justify(widget* self, unsigned justify);

#define EXT_STYLE(widget)                                                      \
  unsigned widget##_get_style(const widget* self);                             \
  void widget##_set_style(widget* self, unsigned style);

#define EXT_RANGE(widget, type)                                                \
  int widget##_get_value(const widget* self);                                  \
  void widget##_get_range(const widget* self, type* lo, type* hi);             \
  void widget##_set_value(widget* self, type value);                           \
  void widget##_set_range(widget* self, type lo, type hi);

#define EXT_SELECTABLE(widget)                                                 \
  const char* widget##_get_item_text(const widget* self, int index);           \
  int widget##_get_num_items(const widget* self);                              \
  int widget##_get_current_item(const widget* self);                           \
  void widget##_append_item(widget* self, const char* text);                   \
  void widget##_clear_items(widget* self);                                     \
  void widget##_set_current_item(widget* self, int index);                     \
  void widget##_set_num_visible(widget* self, int nvis);

#define EXT_DRAWING(widget)                                                    \
  const char* widget##_get_item_text(const widget* self, int index);           \
  void widget##_set_foreground(widget* self, unsigned color);                  \
  void widget##_set_line_width(widget* self, int width);                       \
  void widget##_draw_line(widget* self, int x1, int y1, int x2, int y2);       \
  void widget##_draw_point(widget* self, int x, int y);                        \
  void widget##_draw_rect(widget* self, int x, int y, int w, int h);           \
  void widget##_fill_rect(widget* self, int x, int y, int w, int h);

#ifdef __cplusplus
extern "C"
{
#endif

  //~ fxdefs.h
  unsigned fx_rgb(unsigned r, unsigned g, unsigned b);
  unsigned fx_rgba(unsigned r, unsigned g, unsigned b, unsigned a);
  unsigned fx_red_val(unsigned rgba);
  unsigned fx_green_val(unsigned rgba);
  unsigned fx_blue_val(unsigned rgba);
  unsigned fx_alpha_val(unsigned rgba);

  //~ OPAQUE HANDLES
  // ABI note: widget constructor entry points return owned FXObject handles (or
  // nullptr when the parent/owner argument is missing). Getter functions return
  // borrowed data.

  //~ FXObject.h
  typedef void FXObject;
  void FXObject_delete(FXObject* self);

  //~ FXComposite.h
  typedef struct FXComposite FXComposite;
  int FXComposite_child_width(const FXComposite* self);
  int FXComposite_child_height(const FXComposite* self);

  typedef struct FX4Splitter FX4Splitter;
  typedef struct FX7Segment FX7Segment;
  EXT_JUSTIFY(FX7Segment)
  EXT_HELP(FX7Segment)

  typedef struct FXBitmap FXBitmap;
  typedef struct FXBitmapFrame FXBitmapFrame;
  EXT_JUSTIFY(FXBitmapFrame)

  typedef struct FXBitmapView FXBitmapView;
  typedef struct FXBMPIcon FXBMPIcon;
  typedef struct FXBMPImage FXBMPImage;

  //~ FXColorBar.h
  typedef struct FXColorBar FXColorBar;
  EXT_HELP(FXColorBar)

  typedef struct FXColorDialog FXColorDialog;
  typedef struct FXColorList FXColorList;

  //~ FXColorRing.h
  typedef struct FXColorRing FXColorRing;
  EXT_HELP(FXColorRing)

  //~ FXColorWell.h
  typedef struct FXColorWell FXColorWell;
  EXT_HELP(FXColorWell)

  //~ FXColorWheel.h
  typedef struct FXColorWheel FXColorWheel;
  EXT_HELP(FXColorWheel)

  typedef struct FXComposeContext FXComposeContext;
  typedef struct FXCURCursor FXCURCursor;
  typedef struct FXCursor FXCursor;
  typedef struct FXDataTarget FXDataTarget;
  typedef struct FXDirDialog FXDirDialog;
  typedef struct FXDirList FXDirList;
  typedef struct FXDirSelector FXDirSelector;
  typedef struct FXDockBar FXDockBar;
  typedef struct FXDockHandler FXDockHandler;
  typedef struct FXDockSite FXDockSite;
  typedef struct FXDockSite FXDockSite;
  typedef struct FXDockTitle FXDockTitle;
  EXT_JUSTIFY(FXDockTitle)

  typedef struct FXDragCorner FXDragCorner;
  typedef struct FXDragCorner FXDragCorner;
  typedef struct FXFileDialog FXFileDialog;
  typedef struct FXFileList FXFileList;
  typedef struct FXFoldingList FXFoldingList;
  typedef struct FXFontDialog FXFontDialog;

  //~ FXApp.h
  typedef struct FXApp FXApp;
  typedef long (*CbTimer)(FXApp* app, void* ctx);
  FXApp* FXApp_new(const char* name, const char* vendor, int argc, char** argv);
  int FXApp_run(FXApp* self);
  void FXApp_add_timeout(FXApp* self, CbTimer cb, unsigned ns, void* ctx);

  //~ FXId.h
  typedef struct FXId FXId;
  FXApp* FXId_get_app(const FXId* self);
#ifdef _WIN32
  void* FXId_get_id(const FXId* self);
#else
unsigned long
FXId_get_id(const FXId* self);
#endif

  //~ FXTriStateButton.h
  typedef struct FXTriStateButton FXTriStateButton;
  FXTriStateButton* FXTriStateButton_new(FXComposite* prt,
                                         const char* text1,
                                         const char* text2,
                                         const char* text3);

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
  int FXDrawable_get_height(const FXDrawable* self);
  int FXDrawable_get_width(const FXDrawable* self);

  //~ FXDC.h
  typedef struct FXDC FXDC;
  EXT_DRAWING(FXDC)

  //~ FXDCPrint.h
  typedef struct FXDCPrint FXDCPrint;
  EXT_DRAWING(FXDCPrint)

  //~ FXDCWindow.h
  typedef struct FXDCWindow FXDCWindow;
  FXDCWindow* FXDCWindow_new(FXDrawable* drawable);
  EXT_DRAWING(FXDCWindow)

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
  void FXWindow_set_layout_hints(FXWindow* self, unsigned val);
  void FXWindow_set_x(FXWindow* self, int x);
  void FXWindow_set_y(FXWindow* self, int y);
  void FXWindow_disable(FXWindow* self);
  void FXWindow_enable(FXWindow* self);

  //~ FXImage.h
  typedef struct FXImage FXImage;
  FXImage* FXImage_new(FXApp* app);

  //~ FXImageView.h
  typedef struct FXImageView FXImageView;
  FXImageView* FXImageView_new(FXComposite* prt);

  //~ FXImageFrame.h
  typedef struct FXImageFrame FXImageFrame;
  FXImageFrame* FXImageFrame_new(FXComposite* prt, FXImage* img);
  EXT_JUSTIFY(FXImageFrame)

  //~ FXIcon.h
  typedef struct FXIcon FXIcon;
  FXIcon* FXIcon_new(FXApp* app);

  //~ FXChoiceBox.h
  int FXChoiceBox_ask(FXWindow* owner,
                      unsigned opts,
                      const char* caption,
                      const char* text,
                      FXIcon* icon,
                      const char** choices);

  //~ FXWizard.h
  typedef struct FXWizard FXWizard;
  FXWizard FXWizard_new(FXWindow* owner, const char* title);

  //~ FXPrintDialog.h
  typedef struct FXPrintDialog FXPrintDialog;
  FXPrintDialog FXPrintDialog_new(FXWindow* owner, const char* title);

  //~ FXDialogBox.h
  typedef struct FXDialogBox FXDialogBox;
  FXDialogBox* FXDialogBox_new(FXWindow* owner, const char* title);

  //~ FXFileDialog.h
  const char* FXFileDialog_get_open_filename(FXWindow* owner,
                                             const char* caption,
                                             const char* path,
                                             const char* patterns,
                                             int initial);
  const char* FXFileDialog_get_save_filename(FXWindow* owner,
                                             const char* caption,
                                             const char* path,
                                             const char* patterns,
                                             int initial);

  //~ FXMessageBox.h
  unsigned FXMessageBox_error(FXWindow* owner,
                              unsigned opts,
                              const char* caption,
                              const char* message);
  unsigned FXMessageBox_warning(FXWindow* owner,
                                unsigned opts,
                                const char* caption,
                                const char* message);
  unsigned FXMessageBox_question(FXWindow* owner,
                                 unsigned opts,
                                 const char* caption,
                                 const char* message);
  unsigned FXMessageBox_information(FXWindow* owner,
                                    unsigned opts,
                                    const char* caption,
                                    const char* message);

  //~ FXDial.h
  typedef struct FXDial FXDial;
  FXDial* FXDial_new(FXComposite* prt);
  EXT_RANGE(FXDial, int)
  EXT_HELP(FXDial)

  //~ FXFrame.h
  typedef struct FXFrame FXFrame;
  void FXFrame_set_pad_bottom(FXFrame* self, int pad);
  void FXFrame_set_pad_left(FXFrame* self, int pad);
  void FXFrame_set_pad_right(FXFrame* self, int pad);
  void FXFrame_set_pad_top(FXFrame* self, int pad);
  void FXFrame_set_base_color(FXFrame* self, unsigned color);
  void FXFrame_set_border_color(FXFrame* self, unsigned color);
  void FXFrame_set_hilite_color(FXFrame* self, unsigned color);
  void FXFrame_set_shadow_color(FXFrame* self, unsigned color);
  EXT_STYLE(FXFrame)

  //~ FXKnob.h
  typedef struct FXKnob FXKnob;
  FXKnob* FXKnob_new(FXComposite* prt);
  EXT_RANGE(FXKnob, int)
  EXT_HELP(FXKnob)

  //~ FXLabel.h
  typedef struct FXLabel FXLabel;
  FXLabel* FXLabel_new(FXComposite* prt, const char* title);
  EXT_JUSTIFY(FXLabel)
  EXT_TEXT(FXLabel)

  //~ FXText.h
  typedef struct FXText FXText;
  FXText* FXText_new(FXComposite* prt);
  EXT_TEXT(FXText)
  EXT_EDITABLE(FXText)

  //~ FXTextField.h
  typedef struct FXTextField FXTextField;
  FXTextField* FXTextField_new(FXComposite* frm);
  EXT_TEXT(FXTextField)
  EXT_JUSTIFY(FXTextField)
  EXT_EDITABLE(FXTextField)

  //~ FXSlider.h
  typedef struct FXSlider FXSlider;
  FXSlider* FXSlider_new(FXComposite* parent);
  EXT_RANGE(FXSlider, int)

  //~ FXSpinner.h
  typedef struct FXSpinner FXSpinner;
  FXSpinner* FXSpinner_new(FXComposite* parent);
  void FXSpinner_decrement(FXSpinner* self);
  EXT_RANGE(FXSpinner, int)
  EXT_EDITABLE(FXSpinner)

  //~ FXRealSpinner.h
  typedef struct FXRealSpinner FXRealSpinner;
  FXRealSpinner* FXRealSpinner_new(FXComposite* parent);
  EXT_RANGE(FXRealSpinner, double)
  EXT_EDITABLE(FXRealSpinner)

  //~ FXRealSlider.h
  typedef struct FXRealSlider FXRealSlider;
  FXRealSlider* FXRealSlider_new(FXComposite* parent);
  EXT_RANGE(FXRealSlider, double)

  //~ FXProgressBar.h
  typedef struct FXProgressBar FXProgressBar;
  FXProgressBar* FXProgressBar_new(FXComposite* prt);
  unsigned FXProgressBar_get_progress(const FXProgressBar* self);
  unsigned FXProgressBar_get_total(const FXProgressBar* self);
  int FXProgressBar_get_bar_size(const FXProgressBar* self);
  void FXProgressBar_set_progress(FXProgressBar* self, unsigned value);
  void FXProgressBar_set_total(FXProgressBar* self, unsigned value);
  void FXProgressBar_set_bar_size(FXProgressBar* self, int size);
  void FXProgressBar_increment(FXProgressBar* self, unsigned value);
  void FXProgressBar_show_number(FXProgressBar* self);
  void FXProgressBar_hide_number(FXProgressBar* self);

  //~ FXArrowButton.h
  typedef struct FXArrowButton FXArrowButton;
  FXArrowButton* FXArrowButton_new(FXComposite* parent);
  void FXArrowButton_set_arrow_size(FXArrowButton* self, int size);
  void FXArrowButton_set_arrow_color(FXArrowButton* self, unsigned color);
  EXT_JUSTIFY(FXArrowButton)
  EXT_STATE(FXArrowButton)
  EXT_HELP(FXArrowButton)

  //~ FXButton.h
  typedef struct FXButton FXButton;
  FXButton* FXButton_new(FXComposite* parent, const char* title);
  EXT_STYLE(FXButton)
  EXT_TEXT(FXButton)
  EXT_STATE(FXButton)

  //~ FXPopup.h
  typedef struct FXPopup FXPopup;
  FXPopup* FXPopup_new(FXWindow* owner);

  //~ FXCheckButton.h
  typedef struct FXCheckButton FXCheckButton;
  FXCheckButton* FXCheckButton_new(FXComposite* prt, const char* title);
  EXT_CHECK(FXCheckButton)

  //~ FXMDIButton.h
  typedef struct FXMDIDeleteButton FXMDIDeleteButton;
  FXMDIDeleteButton* FXMDIButton_new(FXComposite* prt);
  typedef struct FXMDIMaximizeButton FXMDIMaximizeButton;
  FXMDIMaximizeButton* FXMDIMaximizeButton_new(FXComposite* prt);
  typedef struct FXMDIMenu FXMDIMenu;
  FXMDIMenu* FXMDIMenu_new(FXComposite* prt);
  typedef struct FXMDIMinimizeButton FXMDIMinimizeButton;
  FXMDIMinimizeButton* FXMDIMinimizeButton_new(FXComposite* prt);
  typedef struct FXMDIRestoreButton FXMDIRestoreButton;
  FXMDIRestoreButton* FXMDIRestoreButton_new(FXComposite* prt);
  typedef struct FXMDIWindowButton FXMDIWindowButton;
  FXMDIWindowButton* FXMDIWindowButton_new(FXComposite* prt, FXPopup* pup);

  //~ FXToggleButton.h
  typedef struct FXToggleButton FXToggleButton;
  FXToggleButton* FXToggleButton_new(FXComposite* prt,
                                     const char* text1,
                                     const char* text2);
  EXT_STATE(FXToggleButton)

  //~ FXRadioButton.h
  typedef struct FXRadioButton FXRadioButton;
  FXRadioButton* FXRadioButton_new(FXComposite* prt, const char* title);
  EXT_CHECK(FXRadioButton)
  EXT_TEXT(FXRadioButton)

  //~ FXTopWindow.h
  typedef struct FXTopWindow FXTopWindow;
  void FXTopWindow_set_decorations(FXTopWindow* self, unsigned decorations);
  void FXTopWindow_set_hspacing(FXTopWindow* self, int hspacing);
  void FXTopWindow_set_vspacing(FXTopWindow* self, int vspacing);

  //~ FXSplashWindow.h
  typedef struct FXSplashWindow FXSplashWindow;
  FXSplashWindow* FXSplashWindow_new(FXApp* app);

  //~ FXMainWindow.h
  typedef struct FXMainWindow FXMainWindow;
  FXMainWindow* FXMainWindow_new(FXApp* app,
                                 const char* title,
                                 int width,
                                 int height);
  void FXMainWindow_show(FXMainWindow* self);

  //~ FXPacker.h
  typedef struct FXPacker FXPacker;
  FXPacker* FXPacker_new(FXComposite* prt);
  void FXPacker_set_hspacing(FXPacker* self, int val);
  void FXPacker_set_vspacing(FXPacker* self, int val);

  //~ FXMatrix.h
  typedef struct FXMatrix FXMatrix;
  FXMatrix* FXMatrix_new(FXComposite* prt, int rows, unsigned opts);
  int FXMatrix_get_num_rows(const FXMatrix* self);
  int FXMatrix_get_num_columns(const FXMatrix* self);
  void FXMatrix_set_num_rows(FXMatrix* self, int rows);
  void FXMatrix_set_num_columns(FXMatrix* self, int cols);
  EXT_STYLE(FXMatrix)

  //~ FXSplitter.h
  typedef struct FXSplitter FXSplitter;
  FXSplitter* FXSplitter_new(FXComposite* prt, unsigned opts);
  int FXSplitter_get_split(const FXSplitter* self, int index);
  int FXSplitter_get_bar_size(const FXSplitter* self);
  void FXSplitter_set_split(FXSplitter* self, int index, int size);
  void FXSplitter_set_bar_size(FXSplitter* self, int size);
  EXT_STYLE(FXSplitter)

  //~ FXGroupBox.h
  typedef struct FXGroupBox FXGroupBox;
  FXGroupBox* FXGroupBox_new(FXComposite* prt, const char* title);
  EXT_STYLE(FXGroupBox)
  EXT_TEXT(FXGroupBox)

  //~ FXVerticalFrame.h
  typedef struct FXVerticalFrame FXVerticalFrame;
  FXVerticalFrame* FXVerticalFrame_new(FXComposite* prt);

  //~ FXHorizontalFrame.h
  typedef struct FXHorizontalFrame FXHorizontalFrame;
  FXHorizontalFrame* FXHorizontalFrame_new(FXComposite* prt);

  //~ FXSwitcher.h
  typedef struct FXSwitcher FXSwitcher;
  FXSwitcher* FXSwitcher_new(FXComposite* prt);
  void FXSwitcher_set_current(FXSwitcher* self, int index);

  //~ FXComboBox.h
  typedef struct FXComboBox FXComboBox;
  FXComboBox* FXComboBox_new(FXComposite* prt, int cols);
  EXT_JUSTIFY(FXComboBox)
  EXT_SELECTABLE(FXComboBox)
  EXT_EDITABLE(FXComboBox)
  EXT_HELP(FXComboBox)

  //~ FXList.h
  typedef struct FXList FXList;
  FXList* FXList_new(FXComposite* prt);
  EXT_SELECTABLE(FXList)
  EXT_STYLE(FXList)

  //~ FXListBox.h
  typedef struct FXListBox FXListBox;
  FXListBox* FXListBox_new(FXComposite* prt);
  EXT_SELECTABLE(FXListBox)

  //~ FXTreeList.h
  typedef struct FXTreeItem FXTreeItem;
  typedef struct FXTreeList FXTreeList;
  FXTreeList* FXTreeList_new(FXComposite* prt);
  FXTreeItem* FXTreeList_append_item(FXTreeList* self,
                                     FXTreeItem* prt,
                                     const char* text);
  void FXTreeList_clear_items(FXTreeList* self);

  //~ FXTable.h
  typedef struct FXTable FXTable;
  FXTable* FXTable_new(FXComposite* prt);
  const char* FXTable_get_item_text(const FXTable* self, int r, int c);
  void FXTable_set_table_size(FXTable* self, int nr, int nc);
  void FXTable_set_item_text(FXTable* self, int r, int c, const char* text);
  EXT_JUSTIFY(FXTable)
  EXT_EDITABLE(FXTable)

  //~ FXCanvas.h
  typedef struct FXCanvas FXCanvas;
  typedef long (
    *CbMouse)(FXCanvas* widget, int event_code, int x, int y, void* context);
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

  //~ FXScrollBar.h
  typedef struct FXScrollBar FXScrollBar;
  FXScrollBar* FXScrollBar_new(FXComposite* prt);
  int FXScrollBar_get_position(const FXScrollBar* self);
  void FXScrollBar_set_position(FXScrollBar* self, int pos);
  void FXScrollBar_set_range(FXScrollBar* self, int hi);

  //~ FXMenuBar.h
  typedef struct FXMenuBar FXMenuBar;
  FXMenuBar* FXMenuBar_new(FXComposite* prt);

  //~ FXMenuButton.h
  typedef struct FXMenuButton FXMenuButton;
  FXMenuButton* FXMenuButton_new(FXComposite* prt,
                                 const char* title,
                                 FXPopup* pop);
  void FXMenuButton_set_popup_style(FXMenuButton* self, unsigned style);
  void FXMenuButton_set_attachment(FXMenuButton* self, unsigned attachment);
  EXT_STYLE(FXMenuButton)

  //~ FXMenuCaption.h
  typedef struct FXMenuCaption FXMenuCaption;
  FXMenuCaption* FXMenuCaption_new(FXComposite* prt, const char* text);

  //~ FXMenuCascade.h
  typedef struct FXMenuCascade FXMenuCascade;
  FXMenuCascade* FXMenuCascade_new(FXComposite* prt, const char* text);

  //~ FXMenuPane.h
  typedef struct FXMenuPane FXMenuPane;
  FXMenuPane* FXMenuPane_new(FXWindow* prt);

  //~ FXMenuTitle.h
  typedef struct FXMenuTitle FXMenuTitle;
  FXMenuTitle* FXMenuTitle_new(FXComposite* prt,
                               const char* text,
                               FXPopup* pop);

  //~ FXMenuCommand.h
  typedef struct FXMenuCommand FXMenuCommand;
  FXMenuCommand* FXMenuCommand_new(FXComposite* prt, const char* text);
  void FXMenuCommand_set_accel_text(FXMenuCommand* self, const char* text);
  const char* FXMenuCommand_get_accel_text(const FXMenuCommand* self);

  //~ FXMenuSeparator.h
  typedef struct FXMenuSeparator FXMenuSeparator;
  FXMenuSeparator* FXMenuSeparator_new(FXComposite* prt);

  //~ FXMenuRadio.h
  typedef struct FXMenuRadio FXMenuRadio;
  FXMenuRadio* FXMenuRadio_new(FXComposite* prt, const char* text);
  EXT_CHECK(FXMenuRadio)

  //~ FXMenuCheck.h
  typedef struct FXMenuCheck FXMenuCheck;
  FXMenuCheck* FXMenuCheck_new(FXComposite* prt, const char* text);
  EXT_CHECK(FXMenuCheck)

  //~ FXStatusLine.h
  typedef struct FXStatusLine FXStatusLine;
  FXStatusLine* FXStatusLine_new(FXComposite* prt);
  EXT_TEXT(FXStatusLine)

  //~ FXStatusBar.h
  typedef struct FXStatusBar FXStatusBar;
  FXStatusBar* FXStatusBar_new(FXComposite* prt);

  //~ FXOption.h
  typedef struct FXOption FXOption;
  FXOption* FXOption_new(FXComposite* prt, const char* text);

  //~ FXOptionMenu.h
  typedef struct FXOptionMenu FXOptionMenu;
  FXOptionMenu* FXOptionMenu_new(FXComposite* prt);

#ifdef __cplusplus
}
#endif
#endif
