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
  type widget##_get_value(const widget* self);                                 \
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
  typedef struct FXObject FXObject;
  void FXObject_delete(FXObject* self);

  //~ FXComposite.h
  typedef struct FXComposite FXComposite;
  int FXComposite_child_width(const FXComposite* self);
  int FXComposite_child_height(const FXComposite* self);

  typedef struct FX4Splitter FX4Splitter;
  typedef struct FX7Segment FX7Segment;
  FX7Segment* FX7Segment_new(FXComposite* prt, const char* text);
  EXT_JUSTIFY(FX7Segment)
  EXT_HELP(FX7Segment)

  typedef struct FXBitmap FXBitmap;
  typedef struct FXBitmapFrame FXBitmapFrame;
  FXBitmapFrame* FXBitmapFrame_new(FXComposite* prt);
  EXT_JUSTIFY(FXBitmapFrame)

  typedef struct FXBitmapView FXBitmapView;
  FXBitmapView* FXBitmapView_new(FXComposite* prt);
  typedef struct FXBMPIcon FXBMPIcon;
  typedef struct FXBMPImage FXBMPImage;

  //~ FXColorBar.h
  typedef struct FXColorBar FXColorBar;
  FXColorBar* FXColorBar_new(FXComposite* prt);
  EXT_HELP(FXColorBar)

  typedef struct FXColorDialog FXColorDialog;
  typedef struct FXColorList FXColorList;

  //~ FXColorRing.h
  typedef struct FXColorRing FXColorRing;
  FXColorRing* FXColorRing_new(FXComposite* prt);
  EXT_HELP(FXColorRing)

  //~ FXColorWell.h
  typedef struct FXColorWell FXColorWell;
  FXColorWell* FXColorWell_new(FXComposite* prt);
  EXT_HELP(FXColorWell)

  //~ FXColorWheel.h
  typedef struct FXColorWheel FXColorWheel;
  FXColorWheel* FXColorWheel_new(FXComposite* prt);
  EXT_HELP(FXColorWheel)

  //~ FXGradientBar.h
  typedef struct FXGradientBar FXGradientBar;
  FXGradientBar* FXGradientBar_new(FXComposite* prt);
  int FXGradientBar_get_num_segments(const FXGradientBar* self);
  int FXGradientBar_get_current_segment(const FXGradientBar* self);
  void FXGradientBar_set_current_segment(FXGradientBar* self, int index);
  void FXGradientBar_set_segment_lower_color(FXGradientBar* self,
                                             int segment,
                                             unsigned color);
  void FXGradientBar_set_segment_upper_color(FXGradientBar* self,
                                             int segment,
                                             unsigned color);

  typedef struct FXComposeContext FXComposeContext;
  typedef struct FXCURCursor FXCURCursor;
  typedef struct FXCursor FXCursor;
  typedef struct FXDataTarget FXDataTarget;
  typedef struct FXDirDialog FXDirDialog;
  typedef struct FXDirList FXDirList;
  typedef struct FXDirSelector FXDirSelector;
  typedef struct FXDockBar FXDockBar;
  FXDockBar* FXDockBar_new(FXComposite* prt);
  typedef struct FXDockSite FXDockSite;
  typedef struct FXDockHandler FXDockHandler;
  FXDockSite* FXDockSite_new(FXComposite* prt);
  typedef struct FXDockTitle FXDockTitle;
  FXDockTitle* FXDockTitle_new(FXDockBar* bar, const char* title);
  EXT_JUSTIFY(FXDockTitle)

  typedef struct FXDragCorner FXDragCorner;
  typedef struct FXFileList FXFileList;
  typedef struct FXFoldingList FXFoldingList;
  FXFoldingList* FXFoldingList_new(FXComposite* prt);
  typedef struct FXFontDialog FXFontDialog;

  //~ FXApp.h
  typedef struct FXApp FXApp;
  typedef long (*CbTimer)(FXApp* app, void* ctx);
  FXApp* FXApp_new(const char* name, const char* vendor, int argc, char** argv);
  int FXApp_run(FXApp* self);
  // The callback re-arms itself on every firing (a repeating interval
  // timer, not one-shot). FXApp_add_timeout returns a handle that must be
  // passed to FXApp_remove_timeout to cancel it and free the internal
  // target object — there is no other way to stop or free one, so a
  // caller that never calls FXApp_remove_timeout leaks it for the
  // lifetime of the app.
  typedef struct FXTimeout FXTimeout;
  FXTimeout* FXApp_add_timeout(FXApp* self, CbTimer cb, unsigned ns, void* ctx);
  void FXApp_remove_timeout(FXApp* self, FXTimeout* handle);

  //~ FXToolTip.h
  typedef struct FXToolTip FXToolTip;
  FXToolTip* FXToolTip_new(FXApp* app);
  void FXToolTip_show(FXToolTip* self);
  EXT_TEXT(FXToolTip)

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
  // FXDC is an abstract base with no public constructor, so it's never
  // reachable through this API as a standalone opaque handle. Drawing
  // entry points live on the concrete subclasses below (FXDCWindow,
  // FXDCPrint) instead.
  typedef struct FXDC FXDC;

  //~ FXDCPrint.h
  typedef struct FXDCPrint FXDCPrint;
  FXDCPrint* FXDCPrint_new(FXApp* app);
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
  void FXWindow_set_target(FXWindow* self, CbWidget cb, void* ctx);
  void FXWindow_set_selector(FXWindow* self, int val);
  void FXWindow_set_width(FXWindow* self, int width);
  void FXWindow_set_height(FXWindow* self, int height);
  void FXWindow_set_layout_hints(FXWindow* self, unsigned val);
  void FXWindow_set_x(FXWindow* self, int x);
  void FXWindow_set_y(FXWindow* self, int y);
  void FXWindow_disable(FXWindow* self);
  void FXWindow_enable(FXWindow* self);

  //~ FXImage.h
  typedef struct FXImage FXImage;
  FXImage* FXImage_new(FXApp* owner);

  //~ FXImageView.h
  typedef struct FXImageView FXImageView;
  FXImageView* FXImageView_new(FXComposite* prt);
  void FXImageView_set_image(FXImageView* self, FXImage* img);
  FXImage* FXImageView_get_image(const FXImageView* self);

  //~ FXImageFrame.h
  typedef struct FXImageFrame FXImageFrame;
  FXImageFrame* FXImageFrame_new(FXComposite* prt, FXImage* img);
  EXT_JUSTIFY(FXImageFrame)
  void FXImageFrame_set_image(FXImageFrame* self, FXImage* img);
  FXImage* FXImageFrame_get_image(const FXImageFrame* self);

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
  FXWizard* FXWizard_new(FXWindow* owner, const char* title);

  //~ FXPrintDialog.h
  typedef struct FXPrintDialog FXPrintDialog;
  FXPrintDialog* FXPrintDialog_new(FXWindow* owner, const char* title);

  //~ FXDialogBox.h
  typedef struct FXDialogBox FXDialogBox;
  FXDialogBox* FXDialogBox_new(FXWindow* owner, const char* title);
  void FXDialogBox_show(FXDialogBox* self);
  void FXDialogBox_hide(FXDialogBox* self);
  unsigned char FXDialogBox_shown(const FXDialogBox* self);

  //~ FXReplaceDialog.h
  typedef struct FXReplaceDialog FXReplaceDialog;
  FXReplaceDialog* FXReplaceDialog_new(FXWindow* owner, const char* caption);
  unsigned FXReplaceDialog_execute(FXReplaceDialog* self);
  const char* FXReplaceDialog_get_search_text(const FXReplaceDialog* self);
  void FXReplaceDialog_set_search_text(FXReplaceDialog* self, const char* text);
  const char* FXReplaceDialog_get_replace_text(const FXReplaceDialog* self);
  void FXReplaceDialog_set_replace_text(FXReplaceDialog* self,
                                        const char* text);
  unsigned FXReplaceDialog_get_search_mode(const FXReplaceDialog* self);
  void FXReplaceDialog_set_search_mode(FXReplaceDialog* self, unsigned mode);

  //~ FXSearchDialog.h
  // FXSearchDialog is a FXReplaceDialog with the replace field hidden —
  // same accessor set (including get/set_replace_text, which still works
  // even though the field isn't shown), duplicated under its own type
  // per this API's usual per-widget pattern rather than reusing
  // FXReplaceDialog's opaque handle.
  typedef struct FXSearchDialog FXSearchDialog;
  FXSearchDialog* FXSearchDialog_new(FXWindow* owner, const char* caption);
  unsigned FXSearchDialog_execute(FXSearchDialog* self);
  const char* FXSearchDialog_get_search_text(const FXSearchDialog* self);
  void FXSearchDialog_set_search_text(FXSearchDialog* self, const char* text);
  unsigned FXSearchDialog_get_search_mode(const FXSearchDialog* self);
  void FXSearchDialog_set_search_mode(FXSearchDialog* self, unsigned mode);

  //~ FXFileDialog.h
  typedef struct FXFileDialog FXFileDialog;
  FXFileDialog* FXFileDialog_new(FXWindow* owner, const char* title);
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
  void FXFileDialog_set_directory(FXFileDialog* self, const char* directory);
  const char* FXFileDialog_get_directory(const FXFileDialog* self);
  void FXFileDialog_set_filename(FXFileDialog* self, const char* filename);
  const char* FXFileDialog_get_filename(const FXFileDialog* self);
  void FXFileDialog_set_pattern(FXFileDialog* self, const char* pattern);
  const char* FXFileDialog_get_pattern(const FXFileDialog* self);

  //~ FXRecentFiles.h
  // Not wired to setTarget/setSelector — a selected recent file is
  // delivered to the target's message handler with the filename as the
  // void* ptr argument, a different shape from CbWidget/CTarget used
  // elsewhere in this file, and not worth inventing a one-off callback
  // type for here. Slots are fixed (index 0..get_max_files()-1, capped
  // at 10 by FOX itself); there's no separate "how many are set" count,
  // so a caller enumerates and checks for empty strings.
  typedef struct FXRecentFiles FXRecentFiles;
  FXRecentFiles* FXRecentFiles_new(FXApp* app);
  int FXRecentFiles_get_max_files(const FXRecentFiles* self);
  void FXRecentFiles_set_max_files(FXRecentFiles* self, int mx);
  const char* FXRecentFiles_get_file(const FXRecentFiles* self, int index);
  void FXRecentFiles_set_file(FXRecentFiles* self,
                              int index,
                              const char* filename);
  void FXRecentFiles_append_file(FXRecentFiles* self, const char* filename);
  void FXRecentFiles_remove_file(FXRecentFiles* self, const char* filename);
  void FXRecentFiles_clear(FXRecentFiles* self);

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

  //~ FXInputDialog.h
  // Distinct from FXFileDialog's "empty string means cancelled" convention
  // above: an empty string is a meaningful, confirmed answer here, so
  // cancellation is signaled by returning nullptr instead of collapsing
  // it into the same value as an intentionally empty confirmed answer.
  const char* FXInputDialog_get_string(FXWindow* owner,
                                       const char* caption,
                                       const char* label,
                                       const char* initial);
  // Returns 0 if the user cancelled (result is left untouched) or 1 if
  // they confirmed (result holds the entered value, clamped to [lo, hi]).
  unsigned char FXInputDialog_get_integer(int* result,
                                          FXWindow* owner,
                                          const char* caption,
                                          const char* label,
                                          int lo,
                                          int hi);

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
  FXKnob* FXKnob_new(FXComposite* parent);
  EXT_RANGE(FXKnob, int)
  EXT_HELP(FXKnob)

  //~ FXLabel.h
  typedef struct FXLabel FXLabel;
  FXLabel* FXLabel_new(FXComposite* parent, const char* title);
  EXT_JUSTIFY(FXLabel)
  EXT_TEXT(FXLabel)

  //~ FXText.h
  typedef struct FXText FXText;
  FXText* FXText_new(FXComposite* prt);
  EXT_TEXT(FXText)
  EXT_EDITABLE(FXText)

  //~ FXTextField.h
  typedef struct FXTextField FXTextField;
  FXTextField* FXTextField_new(FXComposite* prt);
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

  //~ FXRealSpinner.h
  typedef struct FXRealSpinner FXRealSpinner;
  FXRealSpinner* FXRealSpinner_new(FXComposite* parent);
  EXT_RANGE(FXRealSpinner, double)

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

  //~ FXProgressDialog.h
  typedef struct FXProgressDialog FXProgressDialog;
  FXProgressDialog* FXProgressDialog_new(FXWindow* owner,
                                         const char* caption,
                                         const char* label);
  void FXProgressDialog_show(FXProgressDialog* self);
  void FXProgressDialog_hide(FXProgressDialog* self);
  void FXProgressDialog_set_message(FXProgressDialog* self,
                                    const char* message);
  void FXProgressDialog_set_bar_style(FXProgressDialog* self, unsigned style);
  void FXProgressDialog_set_progress(FXProgressDialog* self, unsigned value);
  void FXProgressDialog_set_total(FXProgressDialog* self, unsigned total);
  void FXProgressDialog_increment(FXProgressDialog* self, unsigned value);
  unsigned char FXProgressDialog_is_cancelled(const FXProgressDialog* self);
  void FXProgressDialog_set_cancelled(FXProgressDialog* self,
                                      unsigned char cancelled);

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
  FXButton* FXButton_new(FXComposite* prt, const char* title);
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
  FXMDIDeleteButton* FXMDIDeleteButton_new(FXComposite* prt);
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

  //~ FXMDIChild.h
  typedef struct FXMDIClient FXMDIClient;
  typedef struct FXMDIChild FXMDIChild;
  FXMDIChild* FXMDIChild_new(FXMDIClient* client, const char* title);

  //~ FXMDIClient.h
  FXMDIClient* FXMDIClient_new(FXComposite* prt);

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

  //~ FXHeader.h
  typedef struct FXHeader FXHeader;
  FXHeader* FXHeader_new(FXComposite* prt);

  //~ FXRuler.h
  typedef struct FXRuler FXRuler;
  FXRuler* FXRuler_new(FXComposite* prt, unsigned orientation);

  //~ FXSpring.h
  typedef struct FXSpring FXSpring;
  FXSpring* FXSpring_new(FXComposite* prt);

  //~ FXSeparator.h
  typedef struct FXSeparator FXSeparator;
  FXSeparator* FXSeparator_new(FXComposite* prt);
  EXT_STYLE(FXSeparator)

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

  //~ FXShutter.h
  typedef struct FXShutter FXShutter;
  FXShutter* FXShutter_new(FXComposite* prt);
  int FXShutter_get_current(const FXShutter* self);
  void FXShutter_set_current(FXShutter* self, int panel);

  // FXShutterItem is itself a composite (FXVerticalFrame) that other
  // widgets can be added into. Unlike the FXTopWindow_set_hspacing note
  // above (a general limitation of this API's distinct opaque types),
  // this one has a real fix: FXShutterItem_get_content below returns the
  // item's content pane upcast to FXComposite*, so it's usable directly
  // as another widget's parent — e.g. FXButton_new(content, "OK").
  typedef struct FXShutterItem FXShutterItem;
  FXShutterItem* FXShutterItem_new(FXShutter* prt, const char* text);
  FXComposite* FXShutterItem_get_content(const FXShutterItem* self);
  EXT_HELP(FXShutterItem)

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
                                     FXTreeItem* parent,
                                     const char* text);
  void FXTreeList_clear_items(FXTreeList* self);

  //~ FXTable.h
  typedef struct FXTable FXTable;
  FXTable* FXTable_new(FXComposite* prt);
  const char* FXTable_get_item_text(const FXTable* self, int r, int c);
  void FXTable_set_table_size(FXTable* self, int nr, int nc);
  void FXTable_set_item_text(FXTable* self, int r, int c, const char* text);
  EXT_JUSTIFY(FXTable)

  //~ FXTableItem.h
  typedef struct FXTableItem FXTableItem;
  FXTableItem* FXTableItem_new(FXTable* tbl, const char* text);

  //~ FXCanvas.h
  typedef struct FXCanvas FXCanvas;
  typedef long (
    *CbMouse)(FXCanvas* widget, int event_code, int x, int y, void* context);
  FXCanvas* FXCanvas_new(FXComposite* prt);
  void FXCanvas_set_mouse_callback(FXCanvas* self, CbMouse cb, void* ctx);

  //~ FXGLVisual.h
  typedef struct FXGLVisual FXGLVisual;
  FXGLVisual* FXGLVisual_new(FXApp* app);

  //~ FXGLCanvas.h
  typedef struct FXGLCanvas FXGLCanvas;
  FXGLCanvas* FXGLCanvas_new(FXComposite* prt, FXGLVisual* visual);

  //~ FXGLViewer.h
  typedef struct FXGLViewer FXGLViewer;
  FXGLViewer* FXGLViewer_new(FXComposite* prt, FXGLVisual* visual);

  //~ FXTabBar.h
  typedef struct FXTabBar FXTabBar;
  FXTabBar* FXTabBar_new(FXComposite* prt);

  //~ FXTabBook.h
  typedef struct FXTabBook FXTabBook;
  FXTabBook* FXTabBook_new(FXComposite* prt);
  void FXTabBook_set_current(FXTabBook* self, int index);
  int FXTabBook_get_current(const FXTabBook* self);
  int FXTabBook_get_num_children(const FXTabBook* self);

  //~ FXTabItem.h
  typedef struct FXTabItem FXTabItem;
  FXTabItem* FXTabItem_new(FXTabBook* prt, const char* text);
  void FXTabItem_set_text(FXTabItem* self, const char* text);
  const char* FXTabItem_get_text(const FXTabItem* self);

  //~ FXScrollBar.h
  typedef struct FXScrollBar FXScrollBar;
  FXScrollBar* FXScrollBar_new(FXComposite* prt);
  int FXScrollBar_get_position(const FXScrollBar* self);
  void FXScrollBar_set_position(FXScrollBar* self, int pos);
  void FXScrollBar_set_range(FXScrollBar* self, int hi);

  //~ FXScrollWindow.h
  typedef struct FXScrollWindow FXScrollWindow;
  FXScrollWindow* FXScrollWindow_new(FXComposite* prt,
                                     unsigned opts,
                                     int x,
                                     int y,
                                     int w,
                                     int h);
  int FXScrollWindow_get_x_position(const FXScrollWindow* self);
  int FXScrollWindow_get_y_position(const FXScrollWindow* self);
  void FXScrollWindow_set_position(FXScrollWindow* self, int x, int y);
  unsigned FXScrollWindow_get_scroll_style(const FXScrollWindow* self);
  void FXScrollWindow_set_scroll_style(FXScrollWindow* self, unsigned style);

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

  //~ FXStatusBar.h
  typedef struct FXStatusBar FXStatusBar;
  FXStatusBar* FXStatusBar_new(FXComposite* prt);
  const char* FXStatusBar_get_text(const FXStatusBar* self);
  void FXStatusBar_set_help_text(FXStatusBar* self, const char* text);
  const char* FXStatusBar_get_help_text(const FXStatusBar* self);
  EXT_TEXT(FXStatusBar)

  //~ FXOption.h
  typedef struct FXOption FXOption;
  FXOption* FXOption_new(FXComposite* prt, const char* text);

  //~ FXOptionMenu.h
  typedef struct FXOptionMenu FXOptionMenu;
  FXOptionMenu* FXOptionMenu_new(FXComposite* prt);

  //~ FXToolBar.h
  typedef struct FXToolBar FXToolBar;
  FXToolBar* FXToolBar_new(FXComposite* prt);

  //~ FXToolBarGrip.h
  typedef struct FXToolBarGrip FXToolBarGrip;
  FXToolBarGrip* FXToolBarGrip_new(FXToolBar* toolbar);

  //~ FXToolBarTab.h
  typedef struct FXToolBarTab FXToolBarTab;
  FXToolBarTab* FXToolBarTab_new(FXToolBar* toolbar);

#ifdef __cplusplus
}
#endif
#endif
