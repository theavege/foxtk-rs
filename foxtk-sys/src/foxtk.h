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

  // In C++ mode this is a no-op: by the time this header is parsed in
  // foxtk.cpp, the real FOX headers (fx.h/fx3d.h/FXGradientBar.h) are
  // already included, so every one of these classes is already fully
  // declared and visible via FOX's own `using namespace FX;`. A bare
  // `struct name;` re-declaration here previously triggered
  // -Wmismatched-tags (a real risk under the Microsoft C++ ABI, which --
  // unlike Itanium -- distinguishes struct/class in some contexts,
  // reportedly causing type-identity issues at link time); a bare
  // `class name;` instead creates an *ambiguous* redeclaration against
  // the already-visible real class. Declaring nothing avoids both.
  // NOTE: this means a hypothetical pure-C++ consumer including *only*
  // this header, without first including FOX's own headers, would not
  // get these types declared at all. That's not this project's actual
  // usage (foxtk.cpp always includes the real FOX headers first) but is
  // worth knowing as a constraint of this approach.
#ifdef __cplusplus
#define FOXTK_OPAQUE(name)
#else
#define FOXTK_OPAQUE(name) typedef struct name name;
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
  FOXTK_OPAQUE(FXObject)
  void FXObject_delete(FXObject* self);

  //~ FXComposite.h
  FOXTK_OPAQUE(FXComposite)
  int FXComposite_child_width(const FXComposite* self);
  int FXComposite_child_height(const FXComposite* self);

  FOXTK_OPAQUE(FX4Splitter)
  FOXTK_OPAQUE(FX7Segment)
  FX7Segment* FX7Segment_new(FXComposite* prt, const char* text);
  EXT_JUSTIFY(FX7Segment)
  EXT_HELP(FX7Segment)

  FOXTK_OPAQUE(FXBitmap)
  FOXTK_OPAQUE(FXBitmapFrame)
  FXBitmapFrame* FXBitmapFrame_new(FXComposite* prt);
  EXT_JUSTIFY(FXBitmapFrame)

  FOXTK_OPAQUE(FXBitmapView)
  FXBitmapView* FXBitmapView_new(FXComposite* prt);
  FOXTK_OPAQUE(FXBMPIcon)
  FOXTK_OPAQUE(FXBMPImage)

  //~ FXColorBar.h
  FOXTK_OPAQUE(FXColorBar)
  FXColorBar* FXColorBar_new(FXComposite* prt);
  EXT_HELP(FXColorBar)

  FOXTK_OPAQUE(FXColorDialog)
  FOXTK_OPAQUE(FXColorList)

  //~ FXColorRing.h
  FOXTK_OPAQUE(FXColorRing)
  FXColorRing* FXColorRing_new(FXComposite* prt);
  EXT_HELP(FXColorRing)

  //~ FXColorWell.h
  FOXTK_OPAQUE(FXColorWell)
  FXColorWell* FXColorWell_new(FXComposite* prt);
  EXT_HELP(FXColorWell)

  //~ FXColorWheel.h
  FOXTK_OPAQUE(FXColorWheel)
  FXColorWheel* FXColorWheel_new(FXComposite* prt);
  EXT_HELP(FXColorWheel)

  //~ FXGradientBar.h
  FOXTK_OPAQUE(FXGradientBar)
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

  FOXTK_OPAQUE(FXComposeContext)
  FOXTK_OPAQUE(FXCURCursor)
  FOXTK_OPAQUE(FXCursor)
  FOXTK_OPAQUE(FXDataTarget)
  FOXTK_OPAQUE(FXDirDialog)
  FOXTK_OPAQUE(FXDirList)
  FOXTK_OPAQUE(FXDirSelector)
  FOXTK_OPAQUE(FXDockBar)
  FXDockBar* FXDockBar_new(FXComposite* prt);
  FOXTK_OPAQUE(FXDockSite)
  FOXTK_OPAQUE(FXDockHandler)
  FXDockSite* FXDockSite_new(FXComposite* prt);
  FOXTK_OPAQUE(FXDockTitle)
  FXDockTitle* FXDockTitle_new(FXDockBar* bar, const char* title);
  EXT_JUSTIFY(FXDockTitle)

  FOXTK_OPAQUE(FXDragCorner)
  FOXTK_OPAQUE(FXFileList)
  FOXTK_OPAQUE(FXFoldingList)
  FXFoldingList* FXFoldingList_new(FXComposite* prt);
  FOXTK_OPAQUE(FXFontDialog)

  //~ FXApp.h
  FOXTK_OPAQUE(FXApp)
  typedef long (*CbTimer)(FXApp* app, void* ctx);
  FXApp* FXApp_new(const char* name, const char* vendor, int argc, char** argv);
  int FXApp_run(FXApp* self);
  // The callback re-arms itself on every firing (a repeating interval
  // timer, not one-shot). FXApp_add_timeout returns a handle that must be
  // passed to FXApp_remove_timeout to cancel it and free the internal
  // target object — there is no other way to stop or free one, so a
  // caller that never calls FXApp_remove_timeout leaks it for the
  // lifetime of the app.
  // FXTimeout has no real FOX counterpart to alias -- it's this
  // wrapper's own synthetic handle (see FXApp_add_timeout below) -- so
  // it always needs a real declaration, unlike every other
  // FOXTK_OPAQUE() type above which relies on FOX's own headers already
  // being visible in C++ mode.
  typedef struct FXTimeout FXTimeout;
  FXTimeout* FXApp_add_timeout(FXApp* self, CbTimer cb, unsigned ns, void* ctx);
  void FXApp_remove_timeout(FXApp* self, FXTimeout* handle);

  //~ FXToolTip.h
  FOXTK_OPAQUE(FXToolTip)
  FXToolTip* FXToolTip_new(FXApp* app);
  void FXToolTip_show(FXToolTip* self);
  EXT_TEXT(FXToolTip)

  //~ FXId.h
  FOXTK_OPAQUE(FXId)
  FXApp* FXId_get_app(const FXId* self);
#ifdef _WIN32
  void* FXId_get_id(const FXId* self);
#else
unsigned long
FXId_get_id(const FXId* self);
#endif

  //~ FXTriStateButton.h
  FOXTK_OPAQUE(FXTriStateButton)
  FXTriStateButton* FXTriStateButton_new(FXComposite* prt,
                                         const char* text1,
                                         const char* text2,
                                         const char* text3);

  //~ FXTreeListBox.h
  FOXTK_OPAQUE(FXTreeListBox)
  FXTreeListBox* FXTreeListBox_new(FXComposite* prt);

  //~ FXDriveBox.h
  FOXTK_OPAQUE(FXDriveBox)
  FXDriveBox* FXDriveBox_new(FXComposite* prt);

  //~ FXDirBox.h
  FOXTK_OPAQUE(FXDirBox)
  FXDirBox* FXDirBox_new(FXComposite* prt);

  //~ FXFileSelector.h
  FOXTK_OPAQUE(FXFileSelector)
  FXFileSelector* FXFileSelector_new(FXComposite* prt);

  //~ FXFontSelector.h
  FOXTK_OPAQUE(FXFontSelector)
  FXFontSelector* FXFontSelector_new(FXComposite* prt);

  //~ FXColorSelector.h
  FOXTK_OPAQUE(FXColorSelector)
  FXColorSelector* FXColorSelector_new(FXComposite* prt);

  //~ FXDrawable.h
  FOXTK_OPAQUE(FXDrawable)
  int FXDrawable_get_height(const FXDrawable* self);
  int FXDrawable_get_width(const FXDrawable* self);

  //~ FXDC.h
  // FXDC is an abstract base with no public constructor, so it's never
  // reachable through this API as a standalone opaque handle. Drawing
  // entry points live on the concrete subclasses below (FXDCWindow,
  // FXDCPrint) instead.
  FOXTK_OPAQUE(FXDC)

  //~ FXDCPrint.h
  FOXTK_OPAQUE(FXDCPrint)
  FXDCPrint* FXDCPrint_new(FXApp* app);
  EXT_DRAWING(FXDCPrint)

  //~ FXDCWindow.h
  FOXTK_OPAQUE(FXDCWindow)
  FXDCWindow* FXDCWindow_new(FXDrawable* drawable);
  EXT_DRAWING(FXDCWindow)

  //~ FXWindow.h
  FOXTK_OPAQUE(FXWindow)
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
  FOXTK_OPAQUE(FXImage)
  FXImage* FXImage_new(FXApp* owner);

  //~ FXImageView.h
  FOXTK_OPAQUE(FXImageView)
  FXImageView* FXImageView_new(FXComposite* prt);
  void FXImageView_set_image(FXImageView* self, FXImage* img);
  FXImage* FXImageView_get_image(const FXImageView* self);

  //~ FXImageFrame.h
  FOXTK_OPAQUE(FXImageFrame)
  FXImageFrame* FXImageFrame_new(FXComposite* prt, FXImage* img);
  EXT_JUSTIFY(FXImageFrame)
  void FXImageFrame_set_image(FXImageFrame* self, FXImage* img);
  FXImage* FXImageFrame_get_image(const FXImageFrame* self);

  //~ FXIcon.h
  FOXTK_OPAQUE(FXIcon)
  FXIcon* FXIcon_new(FXApp* app);

  //~ FXChoiceBox.h
  int FXChoiceBox_ask(FXWindow* owner,
                      unsigned opts,
                      const char* caption,
                      const char* text,
                      FXIcon* icon,
                      const char** choices);

  //~ FXWizard.h
  FOXTK_OPAQUE(FXWizard)
  FXWizard* FXWizard_new(FXWindow* owner, const char* title);

  //~ FXPrintDialog.h
  FOXTK_OPAQUE(FXPrintDialog)
  FXPrintDialog* FXPrintDialog_new(FXWindow* owner, const char* title);

  //~ FXDialogBox.h
  FOXTK_OPAQUE(FXDialogBox)
  FXDialogBox* FXDialogBox_new(FXWindow* owner, const char* title);
  void FXDialogBox_show(FXDialogBox* self);
  void FXDialogBox_hide(FXDialogBox* self);
  unsigned char FXDialogBox_shown(const FXDialogBox* self);

  //~ FXReplaceDialog.h
  FOXTK_OPAQUE(FXReplaceDialog)
  FXReplaceDialog* FXReplaceDialog_new(FXWindow* owner, const char* caption);
  unsigned FXReplaceDialog_execute(FXReplaceDialog* self);
  const char* FXReplaceDialog_get_search_text(const FXReplaceDialog* self);
  void FXReplaceDialog_set_search_text(FXReplaceDialog* self,
                                       const char* text);
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
  FOXTK_OPAQUE(FXSearchDialog)
  FXSearchDialog* FXSearchDialog_new(FXWindow* owner, const char* caption);
  unsigned FXSearchDialog_execute(FXSearchDialog* self);
  const char* FXSearchDialog_get_search_text(const FXSearchDialog* self);
  void FXSearchDialog_set_search_text(FXSearchDialog* self, const char* text);
  unsigned FXSearchDialog_get_search_mode(const FXSearchDialog* self);
  void FXSearchDialog_set_search_mode(FXSearchDialog* self, unsigned mode);

  //~ FXFileDialog.h
  FOXTK_OPAQUE(FXFileDialog)
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
  FOXTK_OPAQUE(FXRecentFiles)
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
  FOXTK_OPAQUE(FXDial)
  FXDial* FXDial_new(FXComposite* prt);
  EXT_RANGE(FXDial, int)
  EXT_HELP(FXDial)

  //~ FXFrame.h
  FOXTK_OPAQUE(FXFrame)
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
  FOXTK_OPAQUE(FXKnob)
  FXKnob* FXKnob_new(FXComposite* parent);
  EXT_RANGE(FXKnob, int)
  EXT_HELP(FXKnob)

  //~ FXLabel.h
  FOXTK_OPAQUE(FXLabel)
  FXLabel* FXLabel_new(FXComposite* parent, const char* title);
  EXT_JUSTIFY(FXLabel)
  EXT_TEXT(FXLabel)

  //~ FXText.h
  FOXTK_OPAQUE(FXText)
  FXText* FXText_new(FXComposite* prt);
  EXT_TEXT(FXText)
  EXT_EDITABLE(FXText)

  //~ FXTextField.h
  FOXTK_OPAQUE(FXTextField)
  FXTextField* FXTextField_new(FXComposite* prt);
  EXT_TEXT(FXTextField)
  EXT_JUSTIFY(FXTextField)
  EXT_EDITABLE(FXTextField)

  //~ FXSlider.h
  FOXTK_OPAQUE(FXSlider)
  FXSlider* FXSlider_new(FXComposite* parent);
  EXT_RANGE(FXSlider, int)

  //~ FXSpinner.h
  FOXTK_OPAQUE(FXSpinner)
  FXSpinner* FXSpinner_new(FXComposite* parent);
  void FXSpinner_decrement(FXSpinner* self);
  EXT_RANGE(FXSpinner, int)

  //~ FXRealSpinner.h
  FOXTK_OPAQUE(FXRealSpinner)
  FXRealSpinner* FXRealSpinner_new(FXComposite* parent);
  EXT_RANGE(FXRealSpinner, double)

  //~ FXRealSlider.h
  FOXTK_OPAQUE(FXRealSlider)
  FXRealSlider* FXRealSlider_new(FXComposite* parent);
  EXT_RANGE(FXRealSlider, double)

  //~ FXProgressBar.h
  FOXTK_OPAQUE(FXProgressBar)
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
  FOXTK_OPAQUE(FXProgressDialog)
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
  FOXTK_OPAQUE(FXArrowButton)
  FXArrowButton* FXArrowButton_new(FXComposite* parent);
  void FXArrowButton_set_arrow_size(FXArrowButton* self, int size);
  void FXArrowButton_set_arrow_color(FXArrowButton* self, unsigned color);
  EXT_JUSTIFY(FXArrowButton)
  EXT_STATE(FXArrowButton)
  EXT_HELP(FXArrowButton)

  //~ FXButton.h
  FOXTK_OPAQUE(FXButton)
  FXButton* FXButton_new(FXComposite* prt, const char* title);
  EXT_STYLE(FXButton)
  EXT_TEXT(FXButton)
  EXT_STATE(FXButton)

  //~ FXPopup.h
  FOXTK_OPAQUE(FXPopup)
  FXPopup* FXPopup_new(FXWindow* owner);

  //~ FXCheckButton.h
  FOXTK_OPAQUE(FXCheckButton)
  FXCheckButton* FXCheckButton_new(FXComposite* prt, const char* title);
  EXT_CHECK(FXCheckButton)

  //~ FXMDIButton.h
  FOXTK_OPAQUE(FXMDIDeleteButton)
  FXMDIDeleteButton* FXMDIDeleteButton_new(FXComposite* prt);
  FOXTK_OPAQUE(FXMDIMaximizeButton)
  FXMDIMaximizeButton* FXMDIMaximizeButton_new(FXComposite* prt);
  FOXTK_OPAQUE(FXMDIMenu)
  FXMDIMenu* FXMDIMenu_new(FXComposite* prt);
  FOXTK_OPAQUE(FXMDIMinimizeButton)
  FXMDIMinimizeButton* FXMDIMinimizeButton_new(FXComposite* prt);
  FOXTK_OPAQUE(FXMDIRestoreButton)
  FXMDIRestoreButton* FXMDIRestoreButton_new(FXComposite* prt);
  FOXTK_OPAQUE(FXMDIWindowButton)
  FXMDIWindowButton* FXMDIWindowButton_new(FXComposite* prt, FXPopup* pup);

  //~ FXMDIChild.h
  FOXTK_OPAQUE(FXMDIClient)
  FOXTK_OPAQUE(FXMDIChild)
  FXMDIChild* FXMDIChild_new(FXMDIClient* client, const char* title);

  //~ FXMDIClient.h
  FXMDIClient* FXMDIClient_new(FXComposite* prt);

  //~ FXToggleButton.h
  FOXTK_OPAQUE(FXToggleButton)
  FXToggleButton* FXToggleButton_new(FXComposite* prt,
                                     const char* text1,
                                     const char* text2);
  EXT_STATE(FXToggleButton)

  //~ FXRadioButton.h
  FOXTK_OPAQUE(FXRadioButton)
  FXRadioButton* FXRadioButton_new(FXComposite* prt, const char* title);
  EXT_CHECK(FXRadioButton)
  EXT_TEXT(FXRadioButton)

  //~ FXTopWindow.h
  FOXTK_OPAQUE(FXTopWindow)
  void FXTopWindow_set_hspacing(FXTopWindow* self, int hspacing);
  void FXTopWindow_set_vspacing(FXTopWindow* self, int vspacing);

  //~ FXSplashWindow.h
  FOXTK_OPAQUE(FXSplashWindow)
  FXSplashWindow* FXSplashWindow_new(FXApp* app);

  //~ FXMainWindow.h
  FOXTK_OPAQUE(FXMainWindow)
  FXMainWindow* FXMainWindow_new(FXApp* app,
                                 const char* title,
                                 int width,
                                 int height);
  void FXMainWindow_show(FXMainWindow* self);

  //~ FXPacker.h
  FOXTK_OPAQUE(FXPacker)
  FXPacker* FXPacker_new(FXComposite* prt);
  void FXPacker_set_hspacing(FXPacker* self, int val);
  void FXPacker_set_vspacing(FXPacker* self, int val);

  //~ FXMatrix.h
  FOXTK_OPAQUE(FXMatrix)
  FXMatrix* FXMatrix_new(FXComposite* prt, int rows, unsigned opts);
  int FXMatrix_get_num_rows(const FXMatrix* self);
  int FXMatrix_get_num_columns(const FXMatrix* self);
  void FXMatrix_set_num_rows(FXMatrix* self, int rows);
  void FXMatrix_set_num_columns(FXMatrix* self, int cols);

  //~ FXHeader.h
  FOXTK_OPAQUE(FXHeader)
  FXHeader* FXHeader_new(FXComposite* prt);

  //~ FXRuler.h
  FOXTK_OPAQUE(FXRuler)
  FXRuler* FXRuler_new(FXComposite* prt, unsigned orientation);

  //~ FXSpring.h
  FOXTK_OPAQUE(FXSpring)
  FXSpring* FXSpring_new(FXComposite* prt);

  //~ FXSeparator.h
  FOXTK_OPAQUE(FXSeparator)
  FXSeparator* FXSeparator_new(FXComposite* prt);
  EXT_STYLE(FXSeparator)

  //~ FXSplitter.h
  FOXTK_OPAQUE(FXSplitter)
  FXSplitter* FXSplitter_new(FXComposite* prt, unsigned opts);
  int FXSplitter_get_split(const FXSplitter* self, int index);
  int FXSplitter_get_bar_size(const FXSplitter* self);
  void FXSplitter_set_split(FXSplitter* self, int index, int size);
  void FXSplitter_set_bar_size(FXSplitter* self, int size);
  EXT_STYLE(FXSplitter)

  //~ FXGroupBox.h
  FOXTK_OPAQUE(FXGroupBox)
  FXGroupBox* FXGroupBox_new(FXComposite* prt, const char* title);
  EXT_STYLE(FXGroupBox)

  //~ FXVerticalFrame.h
  FOXTK_OPAQUE(FXVerticalFrame)
  FXVerticalFrame* FXVerticalFrame_new(FXComposite* prt);

  //~ FXHorizontalFrame.h
  FOXTK_OPAQUE(FXHorizontalFrame)
  FXHorizontalFrame* FXHorizontalFrame_new(FXComposite* prt);

  //~ FXSwitcher.h
  FOXTK_OPAQUE(FXSwitcher)
  FXSwitcher* FXSwitcher_new(FXComposite* prt);
  void FXSwitcher_set_current(FXSwitcher* self, int index);

  //~ FXShutter.h
  FOXTK_OPAQUE(FXShutter)
  FXShutter* FXShutter_new(FXComposite* prt);
  int FXShutter_get_current(const FXShutter* self);
  void FXShutter_set_current(FXShutter* self, int panel);

  // FXShutterItem is itself a composite (FXVerticalFrame) that other
  // widgets can be added into. Unlike the FXTopWindow_set_hspacing note
  // above (a general limitation of this API's distinct opaque types),
  // this one has a real fix: FXShutterItem_get_content below returns the
  // item's content pane upcast to FXComposite*, so it's usable directly
  // as another widget's parent — e.g. FXButton_new(content, "OK").
  FOXTK_OPAQUE(FXShutterItem)
  FXShutterItem* FXShutterItem_new(FXShutter* prt, const char* text);
  FXComposite* FXShutterItem_get_content(const FXShutterItem* self);
  EXT_HELP(FXShutterItem)

  //~ FXComboBox.h
  FOXTK_OPAQUE(FXComboBox)
  FXComboBox* FXComboBox_new(FXComposite* prt, int cols);
  EXT_JUSTIFY(FXComboBox)
  EXT_SELECTABLE(FXComboBox)
  EXT_EDITABLE(FXComboBox)
  EXT_HELP(FXComboBox)

  //~ FXList.h
  FOXTK_OPAQUE(FXList)
  FXList* FXList_new(FXComposite* prt);
  EXT_SELECTABLE(FXList)
  EXT_STYLE(FXList)

  //~ FXListBox.h
  FOXTK_OPAQUE(FXListBox)
  FXListBox* FXListBox_new(FXComposite* prt);
  EXT_SELECTABLE(FXListBox)

  //~ FXTreeList.h
  FOXTK_OPAQUE(FXTreeItem)
  FOXTK_OPAQUE(FXTreeList)
  FXTreeList* FXTreeList_new(FXComposite* prt);
  FXTreeItem* FXTreeList_append_item(FXTreeList* self,
                                     FXTreeItem* parent,
                                     const char* text);
  void FXTreeList_clear_items(FXTreeList* self);

  //~ FXTable.h
  FOXTK_OPAQUE(FXTable)
  FXTable* FXTable_new(FXComposite* prt);
  const char* FXTable_get_item_text(const FXTable* self, int r, int c);
  void FXTable_set_table_size(FXTable* self, int nr, int nc);
  void FXTable_set_item_text(FXTable* self, int r, int c, const char* text);
  EXT_JUSTIFY(FXTable)

  //~ FXTableItem.h
  FOXTK_OPAQUE(FXTableItem)
  FXTableItem* FXTableItem_new(FXTable* tbl, const char* text);

  //~ FXCanvas.h
  FOXTK_OPAQUE(FXCanvas)
  typedef long (
    *CbMouse)(FXCanvas* widget, int event_code, int x, int y, void* context);
  FXCanvas* FXCanvas_new(FXComposite* prt);
  void FXCanvas_set_mouse_callback(FXCanvas* self, CbMouse cb, void* ctx);

  //~ FXGLVisual.h
  FOXTK_OPAQUE(FXGLVisual)
  FXGLVisual* FXGLVisual_new(FXApp* app);

  //~ FXGLCanvas.h
  FOXTK_OPAQUE(FXGLCanvas)
  FXGLCanvas* FXGLCanvas_new(FXComposite* prt, FXGLVisual* visual);

  //~ FXGLViewer.h
  FOXTK_OPAQUE(FXGLViewer)
  FXGLViewer* FXGLViewer_new(FXComposite* prt, FXGLVisual* visual);

  //~ FXTabBar.h
  FOXTK_OPAQUE(FXTabBar)
  FXTabBar* FXTabBar_new(FXComposite* prt);

  //~ FXTabBook.h
  FOXTK_OPAQUE(FXTabBook)
  FXTabBook* FXTabBook_new(FXComposite* prt);
  void FXTabBook_set_current(FXTabBook* self, int index);
  int FXTabBook_get_current(const FXTabBook* self);
  int FXTabBook_get_num_children(const FXTabBook* self);

  //~ FXTabItem.h
  FOXTK_OPAQUE(FXTabItem)
  FXTabItem* FXTabItem_new(FXTabBook* prt, const char* text);
  void FXTabItem_set_text(FXTabItem* self, const char* text);
  const char* FXTabItem_get_text(const FXTabItem* self);

  //~ FXScrollBar.h
  FOXTK_OPAQUE(FXScrollBar)
  FXScrollBar* FXScrollBar_new(FXComposite* prt);
  int FXScrollBar_get_position(const FXScrollBar* self);
  void FXScrollBar_set_position(FXScrollBar* self, int pos);
  void FXScrollBar_set_range(FXScrollBar* self, int hi);

  //~ FXScrollWindow.h
  FOXTK_OPAQUE(FXScrollWindow)
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
  FOXTK_OPAQUE(FXMenuBar)
  FXMenuBar* FXMenuBar_new(FXComposite* prt);

  //~ FXMenuButton.h
  FOXTK_OPAQUE(FXMenuButton)
  FXMenuButton* FXMenuButton_new(FXComposite* prt,
                                 const char* title,
                                 FXPopup* pop);
  void FXMenuButton_set_popup_style(FXMenuButton* self, unsigned style);
  void FXMenuButton_set_attachment(FXMenuButton* self, unsigned attachment);
  EXT_STYLE(FXMenuButton)

  //~ FXMenuCaption.h
  FOXTK_OPAQUE(FXMenuCaption)
  FXMenuCaption* FXMenuCaption_new(FXComposite* prt, const char* text);

  //~ FXMenuCascade.h
  FOXTK_OPAQUE(FXMenuCascade)
  FXMenuCascade* FXMenuCascade_new(FXComposite* prt, const char* text);

  //~ FXMenuPane.h
  FOXTK_OPAQUE(FXMenuPane)
  FXMenuPane* FXMenuPane_new(FXWindow* prt);

  //~ FXMenuTitle.h
  FOXTK_OPAQUE(FXMenuTitle)
  FXMenuTitle* FXMenuTitle_new(FXComposite* prt,
                               const char* text,
                               FXPopup* pop);

  //~ FXMenuCommand.h
  FOXTK_OPAQUE(FXMenuCommand)
  FXMenuCommand* FXMenuCommand_new(FXComposite* prt, const char* text);
  void FXMenuCommand_set_accel_text(FXMenuCommand* self, const char* text);
  const char* FXMenuCommand_get_accel_text(const FXMenuCommand* self);

  //~ FXMenuSeparator.h
  FOXTK_OPAQUE(FXMenuSeparator)
  FXMenuSeparator* FXMenuSeparator_new(FXComposite* prt);

  //~ FXMenuRadio.h
  FOXTK_OPAQUE(FXMenuRadio)
  FXMenuRadio* FXMenuRadio_new(FXComposite* prt, const char* text);
  EXT_CHECK(FXMenuRadio)

  //~ FXMenuCheck.h
  FOXTK_OPAQUE(FXMenuCheck)
  FXMenuCheck* FXMenuCheck_new(FXComposite* prt, const char* text);
  EXT_CHECK(FXMenuCheck)

  //~ FXStatusLine.h
  FOXTK_OPAQUE(FXStatusLine)
  FXStatusLine* FXStatusLine_new(FXComposite* prt);

  //~ FXStatusBar.h
  FOXTK_OPAQUE(FXStatusBar)
  FXStatusBar* FXStatusBar_new(FXComposite* prt);
  const char* FXStatusBar_get_text(const FXStatusBar* self);
  void FXStatusBar_set_help_text(FXStatusBar* self, const char* text);
  const char* FXStatusBar_get_help_text(const FXStatusBar* self);
  EXT_TEXT(FXStatusBar)

  //~ FXOption.h
  FOXTK_OPAQUE(FXOption)
  FXOption* FXOption_new(FXComposite* prt, const char* text);

  //~ FXOptionMenu.h
  FOXTK_OPAQUE(FXOptionMenu)
  FXOptionMenu* FXOptionMenu_new(FXComposite* prt);

  //~ FXToolBar.h
  FOXTK_OPAQUE(FXToolBar)
  FXToolBar* FXToolBar_new(FXComposite* prt);

  //~ FXToolBarGrip.h
  FOXTK_OPAQUE(FXToolBarGrip)
  FXToolBarGrip* FXToolBarGrip_new(FXToolBar* toolbar);

  //~ FXToolBarTab.h
  FOXTK_OPAQUE(FXToolBarTab)
  FXToolBarTab* FXToolBarTab_new(FXToolBar* toolbar);

#ifdef __cplusplus
}
#endif
#endif
