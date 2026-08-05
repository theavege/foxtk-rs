#include <fx.h>
#include <fx3d.h>
#include <FXDockHandler.h>
#include <FXStatusBar.h>
#include <FXStatusLine.h>
#include <FXToolBar.h>
#include <FXDockBar.h>
#include <FXBitmapFrame.h>
#include <FXImageFrame.h>
#include <FXMDIButton.h>
#include <FXTable.h>
#include <cstdio>

// ============================================================================
// ERROR HANDLING MACROS
// ============================================================================

/// Validates that a pointer is not null, logs error and returns nullptr if null
#define VALIDATE_POINTER(ptr, name) \
  if (!ptr) { \
    fprintf(stderr, "%s: %s is null at %s:%d\n", __func__, name, __FILE__, __LINE__); \
    return nullptr; \
  }

/// Validates parent pointer for widget construction
#define VALIDATE_PARENT(ptr) VALIDATE_POINTER(ptr, "parent")

/// Validates self pointer for widget methods
#define VALIDATE_SELF(ptr) VALIDATE_POINTER(ptr, "self")

// ============================================================================
// STRING UTILITIES
// ============================================================================

template<typename Value>
inline const char*
string_result(const Value& value)
{
  static thread_local FXString buffer;
  buffer = value;
  return buffer.text();
}

// ============================================================================
// WIDGET CONSTRUCTION TEMPLATES
// ============================================================================

/// Generic widget construction template
/// 
/// @tparam Widget The widget type to construct
/// @tparam Parent The parent type (must inherit from FXObject)
/// @tparam Args Variadic template arguments for additional constructor parameters
/// @param parent The parent widget (must not be null)
/// @param args Additional constructor arguments
/// @return New widget instance or nullptr if parent is null
template<typename Widget, typename Parent, typename... Args>
inline Widget*
make_widget(FXObject* parent, Args&&... args)
{
  VALIDATE_PARENT(parent);
  return new Widget(static_cast<Parent*>(parent), std::forward<Args>(args)...);
}

// ============================================================================
// EXTENSION MACROS FOR COMMON WIDGET PATTERNS
// ============================================================================


#define EXT_TEXT(widget)                                                       \
  const char* widget##_get_text(const widget* self)                            \
  {                                                                            \
    return string_result(self->getText());                                     \
  }                                                                            \
  void widget##_set_text(widget* self, const char* text)                       \
  {                                                                            \
    self->setText(text);                                                       \
  }                                                                            \
  void widget##_set_text_color(widget* self, unsigned color)                   \
  {                                                                            \
    self->setTextColor(color);                                                 \
  }                                                                            \
  void widget##_set_font(widget* self, const char* family, int size)           \
  {                                                                            \
    auto old_font = self->getFont();                                           \
    auto new_font = new FXFont(self->getApp(), family, size, 0, 0);            \
    self->setFont(new_font);                                                   \
    if (old_font && old_font != self->getApp()->getNormalFont()) {             \
      delete old_font;                                                         \
    }                                                                          \
  }

#define EXT_HELP(widget)                                                       \
  const char* widget##_get_help_text(const widget* self)                       \
  {                                                                            \
    return string_result(self->getHelpText());                                 \
  }                                                                            \
  const char* widget##_get_tip_text(const widget* self)                        \
  {                                                                            \
    return string_result(self->getTipText());                                  \
  }                                                                            \
  void widget##_set_help_text(widget* self, const char* text)                  \
  {                                                                            \
    self->setHelpText(text);                                                   \
  }                                                                            \
  void widget##_set_tip_text(widget* self, const char* text)                   \
  {                                                                            \
    self->setTipText(text);                                                    \
  }

#define EXT_RANGE(widget, type)                                                \
  int widget##_get_value(const widget* self)                                   \
  {                                                                            \
    return self->getValue();                                                   \
  }                                                                            \
  void widget##_get_range(const widget* self, type* lo, type* hi)              \
  {                                                                            \
    FXint lower, upper;                                                        \
    self->getRange(lower, upper);                                              \
    if (lo)                                                                    \
      *lo = lower;                                                             \
    if (hi)                                                                    \
      *hi = upper;                                                             \
  }                                                                            \
  void widget##_set_value(widget* self, type value)                            \
  {                                                                            \
    self->setValue(value);                                                     \
  }                                                                            \
  void widget##_set_range(widget* self, type lo, type hi)                      \
  {                                                                            \
    self->setRange(lo, hi);                                                    \
  }

#define EXT_SELECTABLE(widget)                                                 \
  void widget##_append_item(widget* self, const char* text)                    \
  {                                                                            \
    self->appendItem(text);                                                    \
  }                                                                            \
  void widget##_clear_items(widget* self)                                      \
  {                                                                            \
    self->clearItems();                                                        \
  }                                                                            \
  void widget##_set_current_item(widget* self, int index)                      \
  {                                                                            \
    self->setCurrentItem(index);                                               \
  }                                                                            \
  void widget##_set_num_visible(widget* self, int nvis)                        \
  {                                                                            \
    self->setNumVisible(nvis);                                                 \
  }                                                                            \
  const char* widget##_get_item_text(const widget* self, int index)            \
  {                                                                            \
    return string_result(self->getItemText(index));                            \
  }                                                                            \
  int widget##_get_current_item(const widget* self)                            \
  {                                                                            \
    return self->getCurrentItem();                                             \
  }                                                                            \
  int widget##_get_num_items(const widget* self)                               \
  {                                                                            \
    return self->getNumItems();                                                \
  }

//~ OPAQUE HANDLES

typedef long (*CbWidget)(FXObject* wgt, void* ctx);
typedef long (*CbTimer)(FXApp* app, void* c);

//~ CALLBACK BRIDGE

class CTarget : public FXObject
{
  FXDECLARE(CTarget)
protected:
  CTarget() {}

private:
  CbWidget callback = nullptr;
  void* context = nullptr;

public:
  enum
  {
    SEL_COMMAND,
    SEL_CHANGED
  };
  CTarget(CbWidget cb, void* ctx)
    : callback(cb)
    , context(ctx)
  {
  }
  long callBack(FXObject* wgt, FXSelector, void*)
  {
    long result = 0;
    if (this->callback)
      result = this->callback(wgt, this->context);
    return result;
  }
};

FXDEFMAP(CTarget)
CTargetMap[] = {
  FXMAPFUNC(SEL_COMMAND, CTarget::SEL_COMMAND, CTarget::callBack),
  FXMAPFUNC(SEL_CHANGED, CTarget::SEL_CHANGED, CTarget::callBack),
};
FXIMPLEMENT(CTarget, FXObject, CTargetMap, ARRAYNUMBER(CTargetMap))

class CTimeout : public FXObject
{
  FXDECLARE(CTimeout)
protected:
  CTimeout() {}

private:
  CbTimer callback = nullptr;
  unsigned nanosec = 0;

public:
  enum
  {
    SEL_TIMEOUT,
    SEL_CHORE
  };
  CTimeout(CbTimer cb, unsigned ns)
    : callback(cb)
    , nanosec(ns)
  {
  }
  long onTimeout(FXObject* app, FXSelector, void* ctx)
  {
    long result = 0;
    if (this->callback) {
      result = this->callback(static_cast<FXApp*>(app), ctx);
      static_cast<FXApp*>(app)->addTimeout(
        this, CTimeout::SEL_TIMEOUT, nanosec, ctx);
    };
    return result;
  }
  long onChore(FXObject* app, FXSelector, void* ctx)
  {
    long result = 0;
    if (this->callback)
      result = this->callback(static_cast<FXApp*>(app), ctx);
    return result;
  }
};

FXDEFMAP(CTimeout)
CTimeoutMap[] = {
  FXMAPFUNC(SEL_TIMEOUT, CTimeout::SEL_TIMEOUT, CTimeout::onTimeout),
  FXMAPFUNC(SEL_CHORE, CTimeout::SEL_CHORE, CTimeout::onChore),
};
FXIMPLEMENT(CTimeout, FXObject, CTimeoutMap, ARRAYNUMBER(CTimeoutMap))

// CMouseTarget bridge (defined outside extern "C")
class CMouseTarget : public FXObject
{
  FXDECLARE(CMouseTarget)
protected:
  CMouseTarget() {}

private:
  long (*callback)(FXObject*, int, int, int, void*) = nullptr;
  void* context = nullptr;

public:
  enum
  {
    SEL_LBP = SEL_LEFTBUTTONPRESS,
    SEL_LBR = SEL_LEFTBUTTONRELEASE,
    SEL_MOT = SEL_MOTION,
    SEL_RBP = SEL_RIGHTBUTTONPRESS,
    SEL_RBR = SEL_RIGHTBUTTONRELEASE
  };
  CMouseTarget(long (*cb)(FXObject*, int, int, int, void*), void* ctx)
    : callback(cb)
    , context(ctx)
  {
  }
  long callBack(FXObject* wgt, FXSelector sel, void* ptr)
  {
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
      if (sel == SEL_LEFTBUTTONPRESS)
        code = 1;
      else if (sel == SEL_LEFTBUTTONRELEASE)
        code = 2;
      else if (sel == SEL_MOTION)
        code = 3;
      else if (sel == SEL_RIGHTBUTTONPRESS)
        code = 4;
      else if (sel == SEL_RIGHTBUTTONRELEASE)
        code = 5;
      result = this->callback(wgt, code, x, y, this->context);
    }
    return result;
  }
};

FXDEFMAP(CMouseTarget)
CMouseTargetMap[] = {
  FXMAPFUNC(SEL_LEFTBUTTONPRESS, CMouseTarget::SEL_LBP, CMouseTarget::callBack),
  FXMAPFUNC(SEL_LEFTBUTTONRELEASE,
            CMouseTarget::SEL_LBR,
            CMouseTarget::callBack),
  FXMAPFUNC(SEL_MOTION, CMouseTarget::SEL_MOT, CMouseTarget::callBack),
  FXMAPFUNC(SEL_RIGHTBUTTONPRESS,
            CMouseTarget::SEL_RBP,
            CMouseTarget::callBack),
  FXMAPFUNC(SEL_RIGHTBUTTONRELEASE,
            CMouseTarget::SEL_RBR,
            CMouseTarget::callBack),
};
FXIMPLEMENT(CMouseTarget,
            FXObject,
            CMouseTargetMap,
            ARRAYNUMBER(CMouseTargetMap))

class CDockHandler : public FXDockHandler
{
  FXDECLARE(CDockHandler)
protected:
  CDockHandler()
    : FXDockHandler(nullptr, nullptr, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
  {
  }
public:
  explicit CDockHandler(FXDockSite* docksite)
    : FXDockHandler(docksite, nullptr, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
  {
  }
};

FXDEFMAP(CDockHandler)
CDockHandlerMap[] = {};
FXIMPLEMENT(CDockHandler,
            FXDockHandler,
            CDockHandlerMap,
            ARRAYNUMBER(CDockHandlerMap))

#define ASSERT_NOT_NULL(ptr, result)                                           \
  if (!ptr)                                                                    \
  return result

extern "C"
{
  //~ fxdefs.h
  unsigned fx_rgb(unsigned r, unsigned g, unsigned b)
  {
    return FXRGB(r, g, b);
  }
  unsigned fx_rgba(unsigned r, unsigned g, unsigned b, unsigned a)
  {
    return FXRGBA(r, g, b, a);
  }
  unsigned fx_red_val(unsigned rgba)
  {
    return FXREDVAL(rgba);
  }
  unsigned fx_green_val(unsigned rgba)
  {
    return FXGREENVAL(rgba);
  }
  unsigned fx_blue_val(unsigned rgba)
  {
    return FXBLUEVAL(rgba);
  }
  unsigned fx_alpha_val(unsigned rgba)
  {
    return FXALPHAVAL(rgba);
  }

  // ============================================================================
// BASE WIDGETS
// ============================================================================
//~ FXObject.h
  void FXObject_delete(FXObject* self)
  {
    delete self;
  }

  //~ FXId.h
  FXApp* FXId_get_app(const FXId* self)
  {
    ASSERT_NOT_NULL(self, nullptr);
    return self->getApp();
  }
  FXID FXId_get_id(const FXId* self)
  {
    return self->id();
  }

  // ============================================================================
// DRAWING
// ============================================================================
//~ FXDrawable.h
  int FXDrawable_get_height(const FXDrawable* self)
  {
    return self->getHeight();
  }
  int FXDrawable_get_width(const FXDrawable* self)
  {
    return self->getWidth();
  }

  // ============================================================================
// WINDOW MANAGEMENT
// ============================================================================
//~ FXWindow.h
  FXWindow* FXWindow_get_parent(const FXWindow* self)
  {
    ASSERT_NOT_NULL(self, nullptr);
    return self->getParent();
  }
  FXWindow* FXWindow_get_root(const FXWindow* self)
  {
    ASSERT_NOT_NULL(self, nullptr);
    return self->getRoot();
  }
  long FXWindow_has_focus(const FXWindow* self)
  {
    return self->hasFocus();
  }
  void FXWindow_set_target(FXWindow* self, CbWidget cb, void* ctx)
  {
    if (auto old = dynamic_cast<CTarget*>(self->getTarget()))
      delete old;
    self->setTarget(new CTarget(cb, ctx));
  }
  void FXWindow_set_selector(FXWindow* self, int val)
  {
    if (val == 0)
      self->setSelector(CTarget::SEL_COMMAND);
    else if (val == 1)
      self->setSelector(CTarget::SEL_CHANGED);
  }
  void FXWindow_set_width(FXWindow* self, int width)
  {
    self->setWidth(width);
  }
  void FXWindow_set_x(FXWindow* self, int x)
  {
    self->setX(x);
  }
  void FXWindow_set_y(FXWindow* self, int y)
  {
    self->setY(y);
  }
  void FXWindow_set_height(FXWindow* self, int height)
  {
    self->setHeight(height);
  }
  void FXWindow_set_layout_hints(FXWindow* self, unsigned val)
  {
    self->setLayoutHints(val);
  }
  void FXWindow_disable(FXWindow* self)
  {
    self->disable();
  }
  void FXWindow_enable(FXWindow* self)
  {
    self->enable();
  }

  //~ FXComposite.h
  int FXComposite_child_width(const FXComposite* self)
  {
    return self->maxChildWidth();
  }
  int FXComposite_child_height(const FXComposite* self)
  {
    return self->maxChildHeight();
  }

  // ============================================================================
// APPLICATION
// ============================================================================
//~ FXApp.h
  FXApp* FXApp_new(const char* name, const char* vendor, int argc, char** argv)
  {
    auto app = new FXApp(name, vendor);
    app->init(argc, argv);
    return app;
  }
  int FXApp_run(FXApp* self)
  {
    self->create();
    return self->run();
  }
  void FXApp_add_timeout(FXApp* self, CbTimer cb, unsigned ns, void* ctx)
  {
    self->addTimeout(new CTimeout(cb, ns), CTimeout::SEL_TIMEOUT, ns, ctx);
  }

  //~ FXFrame.h
  void FXFrame_set_style(FXFrame* self, unsigned style)
  {
    self->setFrameStyle(style);
  }
  void FXFrame_set_pad_bottom(FXFrame* self, int pad)
  {
    self->setPadBottom(pad);
  }
  void FXFrame_set_pad_left(FXFrame* self, int pad)
  {
    self->setPadLeft(pad);
  }
  void FXFrame_set_pad_right(FXFrame* self, int pad)
  {
    self->setPadRight(pad);
  }
  void FXFrame_set_pad_top(FXFrame* self, int pad)
  {
    self->setPadTop(pad);
  }
  void FXFrame_set_base_color(FXFrame* self, unsigned color)
  {
    self->setBaseColor(color);
  }
  void FXFrame_set_border_color(FXFrame* self, unsigned color)
  {
    self->setBorderColor(color);
  }
  void FXFrame_set_hilite_color(FXFrame* self, unsigned color)
  {
    self->setHiliteColor(color);
  }
  void FXFrame_set_shadow_color(FXFrame* self, unsigned color)
  {
    self->setShadowColor(color);
  }

  //~ FXKnob.h
  FXKnob* FXKnob_new(FXComposite* parent)
  {
    ASSERT_NOT_NULL(parent, nullptr);
    return make_widget<FXKnob, FXComposite>(parent);
  }
  EXT_RANGE(FXKnob, int)
  EXT_HELP(FXKnob)

  //~ FXLabel.h
  FXLabel* FXLabel_new(FXComposite* parent, const char* title)
  {
    ASSERT_NOT_NULL(parent, nullptr);
    return make_widget<FXLabel, FXComposite>(parent, title);
  }
  void FXLabel_set_justify(FXLabel* self, unsigned justify)
  {
    self->setJustify(justify);
  }
  EXT_TEXT(FXLabel)

  //~ FXArrowButton.h
  FXArrowButton* FXArrowButton_new(FXComposite* parent)
  {
    ASSERT_NOT_NULL(parent, nullptr);
    return make_widget<FXArrowButton, FXComposite>(parent);
  }
  void FXArrowButton_set_arrow_size(FXArrowButton* self, int size)
  {
    self->setArrowSize(size);
  }
  void FXArrowButton_set_arrow_color(FXArrowButton* self, unsigned color)
  {
    self->setArrowColor(color);
  }
  EXT_HELP(FXArrowButton)

  //~ FXMessageBox.h
  unsigned FXMessageBox_error(FXWindow* owner,
                              unsigned opts,
                              const char* caption,
                              const char* message)
  {
    return FXMessageBox::error(owner, opts, caption, "%s", message);
  }
  unsigned FXMessageBox_warning(FXWindow* owner,
                                unsigned opts,
                                const char* caption,
                                const char* message)
  {
    return FXMessageBox::warning(owner, opts, caption, "%s", message);
  }
  unsigned FXMessageBox_question(FXWindow* owner,
                                 unsigned opts,
                                 const char* caption,
                                 const char* message)
  {
    return FXMessageBox::question(owner, opts, caption, "%s", message);
  }
  unsigned FXMessageBox_information(FXWindow* owner,
                                    unsigned opts,
                                    const char* caption,
                                    const char* message)
  {
    return FXMessageBox::information(owner, opts, caption, "%s", message);
  }

  //~ FXChoiceBox.h
  int FXChoiceBox_ask(FXWindow* owner,
                      unsigned opts,
                      const char* caption,
                      const char* text,
                      FXIcon* icon,
                      const char** choices)
  {
    return FXChoiceBox::ask(owner, opts, caption, text, icon, choices);
  }

  //~ FXPrintDialog.h
  FXPrintDialog* FXPrintDialog_new(FXWindow* owner, const char* title)
  {
    return make_widget<FXPrintDialog, FXWindow>(owner, title);
  }

  //~ FXTriStateButton.h
  FXTriStateButton* FXTriStateButton_new(FXComposite* prt,
                                         const char* text1,
                                         const char* text2,
                                         const char* text3)
  {
    return make_widget<FXTriStateButton, FXComposite>(prt, text1, text2, text3);
  }

  //~ FXTreeListBox.h
  FXTreeListBox* FXTreeListBox_new(FXComposite* prt)
  {
    return make_widget<FXTreeListBox, FXComposite>(prt);
  }

  //~ FXDriveBox.h
  FXDriveBox* FXDriveBox_new(FXComposite* prt)
  {
    return make_widget<FXDriveBox, FXComposite>(prt);
  }

  //~ FXDirBox.h
  FXDirBox* FXDirBox_new(FXComposite* prt)
  {
    return make_widget<FXDirBox, FXComposite>(prt);
  }
  FXDirList* FXDirList_new(FXComposite* prt)
  {
    return make_widget<FXDirList, FXComposite>(prt);
  }
  FXDirSelector* FXDirSelector_new(FXComposite* prt)
  {
    return make_widget<FXDirSelector, FXComposite>(prt);
  }

  //~ FXFileSelector.h
  FXFileSelector* FXFileSelector_new(FXComposite* prt)
  {
    return make_widget<FXFileSelector, FXComposite>(prt);
  }
  FXFileList* FXFileList_new(FXComposite* prt)
  {
    return make_widget<FXFileList, FXComposite>(prt);
  }

  //~ FXFontSelector.h
  FXFontSelector* FXFontSelector_new(FXComposite* prt)
  {
    return make_widget<FXFontSelector, FXComposite>(prt);
  }

  //~ FXColorSelector.h
  FXColorSelector* FXColorSelector_new(FXComposite* prt)
  {
    return make_widget<FXColorSelector, FXComposite>(prt);
  }

  //~ FXDial.h
  FXDial* FXDial_new(FXComposite* prt)
  {
    return make_widget<FXDial, FXComposite>(prt);
  }
  EXT_RANGE(FXDial, int)
  EXT_HELP(FXDial)

  //~ FXColorWell.h
  FXColorWell* FXColorWell_new(FXComposite* prt)
  {
    return make_widget<FXColorWell, FXComposite>(prt);
  }

  //~ FXColorWheel.h
  FXColorWheel* FXColorWheel_new(FXComposite* prt)
  {
    return make_widget<FXColorWheel, FXComposite>(prt);
  }

  //~ FXColorRing.h
  FXColorRing* FXColorRing_new(FXComposite* prt)
  {
    return make_widget<FXColorRing, FXComposite>(prt);
  }

  //~ FXColorBar.h
  FXColorBar* FXColorBar_new(FXComposite* prt)
  {
    return make_widget<FXColorBar, FXComposite>(prt);
  }

  //~ FX7Segment.h
  FX7Segment* FX7Segment_new(FXComposite* prt, const char* text)
  {
    return make_widget<FX7Segment, FXComposite>(prt, text);
  }

  //~ FXColorDialog.h
  FXColorDialog* FXColorDialog_new(FXWindow* owner, const char* title)
  {
    return make_widget<FXColorDialog, FXWindow>(owner, title);
  }

  //~ FXDialogBox.h
  FXDialogBox* FXDialogBox_new(FXWindow* owner, const char* title)
  {
    return make_widget<FXDialogBox, FXWindow>(owner, title);
  }
  void FXDialogBox_show(FXDialogBox* self)
  {
    self->show();
  }
  void FXDialogBox_hide(FXDialogBox* self)
  {
    self->hide();
  }
  unsigned char FXDialogBox_shown(const FXDialogBox* self)
  {
    return self->shown();
  }

  //~ FXFileDialog.h
  FXFileDialog* FXFileDialog_new(FXWindow* owner, const char* title)
  {
    return make_widget<FXFileDialog, FXWindow>(owner, title);
  }
  const char* FXFileDialog_get_open_filename(FXWindow* owner,
                                             const char* caption,
                                             const char* path,
                                             const char* patterns,
                                             int initial)
  {
    return string_result(
      FXFileDialog::getOpenFilename(owner, caption, path, patterns, initial));
  }
  const char* FXFileDialog_get_save_filename(FXWindow* owner,
                                             const char* caption,
                                             const char* path,
                                             const char* patterns,
                                             int initial)
  {
    return string_result(
      FXFileDialog::getSaveFilename(owner, caption, path, patterns, initial));
  }
  void FXFileDialog_set_directory(FXFileDialog* self, const char* directory)
  {
    self->setDirectory(directory);
  }
  const char* FXFileDialog_get_directory(const FXFileDialog* self)
  {
    return string_result(self->getDirectory());
  }
  void FXFileDialog_set_filename(FXFileDialog* self, const char* filename)
  {
    self->setFilename(filename);
  }
  const char* FXFileDialog_get_filename(const FXFileDialog* self)
  {
    return string_result(self->getFilename());
  }
  void FXFileDialog_set_pattern(FXFileDialog* self, const char* pattern)
  {
    self->setPattern(pattern);
  }
  const char* FXFileDialog_get_pattern(const FXFileDialog* self)
  {
    return string_result(self->getPattern());
  }

  // ============================================================================
// INPUT WIDGETS - BUTTONS
// ============================================================================
//~ FXButton.h
  FXButton* FXButton_new(FXComposite* prt, const char* title)
  {
    return make_widget<FXButton, FXComposite>(prt, title);
  }
  void FXButton_set_state(FXButton* self, unsigned state)
  {
    self->setState(state);
  }
  void FXButton_set_style(FXButton* self, unsigned style)
  {
    self->setButtonStyle(style);
  }
  EXT_TEXT(FXButton)

  //~ FXCheckButton.h
  FXCheckButton* FXCheckButton_new(FXComposite* prt, const char* title)
  {
    return make_widget<FXCheckButton, FXComposite>(prt, title);
  }
  unsigned char FXCheckButton_get_check(const FXCheckButton* self)
  {
    return self->getCheck();
  }
  void FXCheckButton_set_check(FXCheckButton* self, unsigned char check)
  {
    self->setCheck(check);
  }

  //~ FXRadioButton.h
  FXRadioButton* FXRadioButton_new(FXComposite* prt, const char* title)
  {
    return make_widget<FXRadioButton, FXComposite>(prt, title);
  }
  unsigned char FXRadioButton_get_check(const FXRadioButton* self)
  {
    return self->getCheck();
  }
  void FXRadioButton_set_check(FXRadioButton* self)
  {
    self->setCheck();
  }
  EXT_TEXT(FXRadioButton)

  //~ FXToggleButton.h
  FXToggleButton* FXToggleButton_new(FXComposite* prt,
                                     const char* text1,
                                     const char* text2)
  {
    return make_widget<FXToggleButton, FXComposite>(prt, text1, text2);
  }

  //~ FXText.h
  FXText* FXText_new(FXComposite* prt)
  {
    return make_widget<FXText, FXComposite>(prt);
  }
  void FXText_set_editable(FXText* self, long editable)
  {
    self->setEditable(editable != 0);
  }
  EXT_TEXT(FXText)

  //~ FXTextField.h
  FXTextField* FXTextField_new(FXComposite* prt)
  {
    return make_widget<FXTextField, FXComposite>(prt, 8);
  }
  void FXTextField_set_editable(FXTextField* self, long val)
  {
    self->setEditable(val != 0);
  }
  EXT_TEXT(FXTextField)

  //~ FXSlider.h
  FXSlider* FXSlider_new(FXComposite* parent)
  {
    return make_widget<FXSlider, FXComposite>(parent);
  }
  EXT_RANGE(FXSlider, int)

  //~ FXRealSlider.h
  FXRealSlider* FXRealSlider_new(FXComposite* parent);

  //~ FXRealSpinner.h
  FXRealSpinner* FXRealSpinner_new(FXComposite* parent);

  //~ FXSpinner.h
  FXSpinner* FXSpinner_new(FXComposite* parent)
  {
    return make_widget<FXSpinner, FXComposite>(parent, 6);
  }
  void FXSpinner_decrement(FXSpinner* self)
  {
    self->decrement();
  }
  EXT_RANGE(FXSpinner, int)

  //~ FXProgressBar.h
  FXProgressBar* FXProgressBar_new(FXComposite* prt)
  {
    return make_widget<FXProgressBar, FXComposite>(prt);
  }
  void FXProgressBar_set_progress(FXProgressBar* self, unsigned value)
  {
    self->setProgress(value);
  }
  unsigned FXProgressBar_get_progress(const FXProgressBar* self)
  {
    return self->getProgress();
  }
  void FXProgressBar_set_total(FXProgressBar* self, unsigned value)
  {
    self->setTotal(value);
  }
  unsigned FXProgressBar_get_total(const FXProgressBar* self)
  {
    return self->getTotal();
  }
  void FXProgressBar_increment(FXProgressBar* self, unsigned value)
  {
    self->increment(value);
  }
  void FXProgressBar_show_number(FXProgressBar* self)
  {
    self->showNumber();
  }
  void FXProgressBar_hide_number(FXProgressBar* self)
  {
    self->hideNumber();
  }
  void FXProgressBar_set_bar_size(FXProgressBar* self, int size)
  {
    self->setBarSize(size);
  }
  int FXProgressBar_get_bar_size(const FXProgressBar* self)
  {
    return self->getBarSize();
  }

  // ============================================================================
// LAYOUT WIDGETS
// ============================================================================
//~ FXPacker.h
  FXPacker* FXPacker_new(FXComposite* prt)
  {
    return make_widget<FXPacker, FXComposite>(prt);
  }
  void FXPacker_set_hspacing(FXPacker* self, int val)
  {
    self->setHSpacing(val);
  }
  void FXPacker_set_vspacing(FXPacker* self, int val)
  {
    self->setVSpacing(val);
  }

  //~ FXMatrix.h
  FXMatrix* FXMatrix_new(FXComposite* prt, int rows, unsigned opts)
  {
    return make_widget<FXMatrix, FXComposite>(prt, rows, opts);
  }
  void FXMatrix_set_matrix_style(FXMatrix* self, unsigned style)
  {
    self->setMatrixStyle(style);
  }
  void FXMatrix_set_num_rows(FXMatrix* self, int rows)
  {
    self->setNumRows(rows);
  }
  void FXMatrix_set_num_columns(FXMatrix* self, int cols)
  {
    self->setNumColumns(cols);
  }
  unsigned FXMatrix_get_matrix_style(const FXMatrix* self)
  {
    return self->getMatrixStyle();
  }
  int FXMatrix_get_num_rows(const FXMatrix* self)
  {
    return self->getNumRows();
  }
  int FXMatrix_get_num_columns(const FXMatrix* self)
  {
    return self->getNumColumns();
  }

  //~ FXHeader.h
  FXHeader* FXHeader_new(FXComposite* prt)
  {
    return make_widget<FXHeader, FXComposite>(prt);
  }

  //~ FXRuler.h
  FXRuler* FXRuler_new(FXComposite* prt, unsigned orientation)
  {
    return make_widget<FXRuler, FXComposite>(prt, nullptr, 0, orientation);
  }

  //~ FXSpring.h
  FXSpring* FXSpring_new(FXComposite* prt)
  {
    return make_widget<FXSpring, FXComposite>(prt);
  }

  //~ FXSplitter.h
  FXSplitter* FXSplitter_new(FXComposite* prt, unsigned opts)
  {
    return make_widget<FXSplitter, FXComposite>(prt, opts);
  }
  int FXSplitter_get_split(const FXSplitter* self, int index)
  {
    return self->getSplit(index);
  }
  void FXSplitter_set_split(FXSplitter* self, int index, int size)
  {
    self->setSplit(index, size);
  }
  void FXSplitter_set_splitter_style(FXSplitter* self, unsigned style)
  {
    self->setSplitterStyle(style);
  }
  unsigned FXSplitter_get_splitter_style(const FXSplitter* self)
  {
    return self->getSplitterStyle();
  }
  void FXSplitter_set_bar_size(FXSplitter* self, int size)
  {
    self->setBarSize(size);
  }
  int FXSplitter_get_bar_size(const FXSplitter* self)
  {
    return self->getBarSize();
  }

  // ============================================================================
// DOCKING WIDGETS
// ============================================================================
//~ FXDockBar.h
  FXDockBar* FXDockBar_new(FXComposite* prt)
  {
    return make_widget<FXDockBar, FXComposite>(prt, LAYOUT_TOP | LAYOUT_LEFT | LAYOUT_FILL_X);
  }

  //~ FXDockHandler.h
  FXDockHandler* FXDockHandler_new(FXDockSite* docksite)
  {
    return new CDockHandler(docksite);
  }

  //~ FXDockSite.h
  FXDockSite* FXDockSite_new(FXComposite* prt)
  {
    return make_widget<FXDockSite, FXComposite>(prt);
  }

  //~ FXDockTitle.h
  FXDockTitle* FXDockTitle_new(FXDockBar* bar, const char* title)
  {
    return new FXDockTitle(bar, title);
  }
  void FXDockTitle_set_justify(FXDockTitle* self, unsigned justify)
  {
    self->setJustify(justify);
  }
  unsigned FXDockTitle_get_justify(const FXDockTitle* self)
  {
    return self->getJustify();
  }

  //~ FXScrollWindow.h
  FXScrollWindow* FXScrollWindow_new(FXComposite* prt,
                                     unsigned opts,
                                     int x,
                                     int y,
                                     int w,
                                     int h)
  {
    return make_widget<FXScrollWindow, FXComposite>(prt, opts, x, y, w, h);
  }

  //~ FXGroupBox.h
  FXGroupBox* FXGroupBox_new(FXComposite* prt, const char* title)
  {
    return make_widget<FXGroupBox, FXComposite>(prt, title);
  }
  void FXGroupBox_set_style(FXGroupBox* self, unsigned style)
  {
    self->setGroupBoxStyle(style);
  }
  void FXGroupBox_set_text(FXGroupBox* self, const char* text)
  {
    self->setText(text);
  }

  //~ FXVerticalFrame.h
  FXVerticalFrame* FXVerticalFrame_new(FXComposite* prt)
  {
    return make_widget<FXVerticalFrame, FXComposite>(prt);
  }

  //~ FXHorizontalFrame.h
  FXHorizontalFrame* FXHorizontalFrame_new(FXComposite* prt)
  {
    return make_widget<FXHorizontalFrame, FXComposite>(prt);
  }

  //~ FXSwitcher.h
  FXSwitcher* FXSwitcher_new(FXComposite* prt)
  {
    return make_widget<FXSwitcher, FXComposite>(prt);
  }

  void FXSwitcher_set_current(FXSwitcher* self, int index)
  {
    self->setCurrent(index);
  }

  //~ FXDCWindow.h
  FXDCWindow* FXDCWindow_new(FXDrawable* drawable)
  {
    return make_widget<FXDCWindow, FXDrawable>(drawable);
  }

  //~ FXDC.h
  void FXDC_set_foreground(FXDCWindow* self, unsigned color)
  {
    self->setForeground(color);
  }
  void FXDC_set_line_width(FXDCWindow* self, int width)
  {
    self->setLineWidth(width);
  }
  void FXDC_draw_line(FXDCWindow* self, int x1, int y1, int x2, int y2)
  {
    self->drawLine(x1, y1, x2, y2);
  }
  void FXDC_draw_point(FXDCWindow* self, int x, int y)
  {
    self->drawPoint(x, y);
  }
  void FXDC_draw_rect(FXDCWindow* self, int x, int y, int w, int h)
  {
    self->drawRectangle(x, y, w, h);
  }
  void FXDC_fill_rect(FXDCWindow* self, int x, int y, int w, int h)
  {
    self->fillRectangle(x, y, w, h);
  }

  //~ FXSplashWindow.h
  FXSplashWindow* FXSplashWindow_new(FXApp* app)
  {
    return make_widget<FXSplashWindow, FXApp>(app, nullptr);
  }

  //~ FXMainWindow.h
  FXMainWindow* FXMainWindow_new(FXApp* app,
                                 const char* title,
                                 int width,
                                 int height)
  {
    return make_widget<FXMainWindow, FXApp>(
      app, title, nullptr, nullptr, DECOR_ALL, 0, 0, width, height);
  }
  void FXMainWindow_show(FXMainWindow* self)
  {
    self->show(PLACEMENT_SCREEN);
  }

  //~ FXComboBox.h
  FXComboBox* FXComboBox_new(FXComposite* prt, int cols)
  {
    return make_widget<FXComboBox, FXComposite>(prt, cols);
  }
  EXT_SELECTABLE(FXComboBox)
  EXT_HELP(FXComboBox)

  //~ FXList.h
  FXList* FXList_new(FXList* prt)
  {
    return make_widget<FXList, FXComposite>(prt);
  }
  void FXList_set_style(FXList* self, unsigned style)
  {
    self->setListStyle(style);
  }
  EXT_SELECTABLE(FXList)

  //~ FXListBox.h
  FXListBox* FXListBox_new(FXListBox* prt)
  {
    return make_widget<FXListBox, FXComposite>(prt);
  }
  EXT_SELECTABLE(FXListBox)

  //~ FXTreeList.h
  FXTreeList* FXTreeList_new(FXComposite* prt)
  {
    return make_widget<FXTreeList, FXComposite>(prt);
  }
  FXTreeItem* FXTreeList_append_item(FXTreeList* self,
                                     FXTreeItem* item,
                                     const char* text)
  {
    return self->appendItem(item, text);
  }
  void FXTreeList_clear_items(FXTreeList* self)
  {
    self->clearItems();
  }

  //~ FXTable.h
  FXTable* FXTable_new(FXComposite* prt)
  {
    return make_widget<FXTable, FXComposite>(prt);
  }
  void FXTable_set_table_size(FXTable* self, int nr, int nc)
  {
    self->setTableSize(nr, nc);
  }
  void FXTable_set_item_text(FXTable* self, int r, int c, const char* text)
  {
    self->setItemText(r, c, text);
  }
  const char* FXTable_get_item_text(const FXTable* self, int r, int c)
  {
    return string_result(self->getItemText(r, c));
  }

  // ============================================================================
// DRAWING WIDGETS
// ============================================================================
//~ FXCanvas.h
  FXCanvas* FXCanvas_new(FXComposite* prt)
  {
    return make_widget<FXCanvas, FXComposite>(prt);
  }
  void FXCanvas_set_mouse_callback(FXCanvas* self,
                                   long (*cb)(FXObject*, int, int, int, void*),
                                   void* ctx)
  {
    auto old = self->getTarget();
    if (static_cast<CMouseTarget*>(old))
      delete old;
    self->setTarget(static_cast<FXObject*>(new CMouseTarget(cb, ctx)));
  }

  //~ FXGLVisual.h
  FXGLVisual* FXGLVisual_new(FXApp* app)
  {
    return make_widget<FXGLVisual, FXApp>(app, 0);
  }

  // ============================================================================
// OPENGL WIDGETS
// ============================================================================
//~ FXGLCanvas.h
  FXGLCanvas* FXGLCanvas_new(FXComposite* prt, FXGLVisual* visual)
  {
    return make_widget<FXGLCanvas, FXComposite>(prt, visual);
  }

  //~ FXGLViewer.h
  FXGLViewer* FXGLViewer_new(FXComposite* prt, FXGLVisual* visual)
  {
    return make_widget<FXGLViewer, FXComposite>(prt, visual);
  }

  //~ FXTabBar.h
  FXTabBar* FXTabBar_new(FXComposite* prt)
  {
    return make_widget<FXTabBar, FXComposite>(prt);
  }

  //~ FXTabBook.h
  FXTabBook* FXTabBook_new(FXComposite* prt)
  {
    return make_widget<FXTabBook, FXComposite>(prt);
  }
  void FXTabBook_set_current(FXTabBook* self, int index)
  {
    self->setCurrent(index);
  }
  int FXTabBook_get_current(const FXTabBook* self)
  {
    return self->getCurrent();
  }
  int FXTabBook_get_num_children(const FXTabBook* self)
  {
    return self->numChildren();
  }

  //~ FXTabItem.h
  FXTabItem* FXTabItem_new(FXTabBook* prt, const char* text)
  {
    return make_widget<FXTabItem, FXTabBook>(prt, text);
  }
  void FXTabItem_set_text(FXTabItem* self, const char* text)
  {
    self->setText(text);
  }
  const char* FXTabItem_get_text(const FXTabItem* self)
  {
    return string_result(self->getText());
  }

  //~ FXScrollBar.h
  FXScrollBar* FXScrollBar_new(FXComposite* prt)
  {
    return make_widget<FXScrollBar, FXComposite>(prt);
  }
  int FXScrollBar_get_position(const FXScrollBar* self)
  {
    return self->getPosition();
  }
  void FXScrollBar_set_position(FXScrollBar* self, int pos)
  {
    self->setPosition(pos);
  }
  void FXScrollBar_set_range(FXScrollBar* self, int hi)
  {
    self->setRange(hi);
  }

  // ============================================================================
// MENU WIDGETS
// ============================================================================
//~ FXMenuBar.h
  FXMenuBar* FXMenuBar_new(FXComposite* prt)
  {
    return make_widget<FXMenuBar, FXComposite>(prt, nullptr);
  }

  //~ FXMenuPane.h
  FXMenuPane* FXMenuPane_new(FXWindow* prt)
  {
    return make_widget<FXMenuPane, FXWindow>(prt);
  }

  //~ FXPopup.h
  FXPopup* FXPopup_new(FXWindow* owner)
  {
    return make_widget<FXPopup, FXWindow>(owner);
  }

  //~ FXMenuButton.h
  FXMenuButton* FXMenuButton_new(FXComposite* prt,
                                 const char* title,
                                 FXPopup* pop)
  {
    auto wgt = make_widget<FXMenuButton, FXComposite>(prt, title);
    wgt->setMenu(pop);
    return wgt;
  }
  void FXMenuButton_set_style(FXMenuButton* self, FXuint style)
  {
    self->setButtonStyle(style);
  }
  void FXMenuButton_set_popup_style(FXMenuButton* self, FXuint style)
  {
    self->setPopupStyle(style);
  }
  void FXMenuButton_set_attachment(FXMenuButton* self, FXuint attachment)
  {
    self->setAttachment(attachment);
  }

  //~ FXMenuTitle.h
  FXMenuTitle* FXMenuTitle_new(FXComposite* prt, const char* text, FXPopup* pop)
  {
    auto wgt = make_widget<FXMenuTitle, FXComposite>(prt, text);
    wgt->setMenu(pop);
    return wgt;
  }

  //~ FXMenuCaption.h
  FXMenuCaption* FXMenuCaption_new(FXComposite* prt, const char* text)
  {
    return make_widget<FXMenuCaption, FXComposite>(prt, text);
  }

  //~ FXMenuCascade.h
  FXMenuCascade* FXMenuCascade_new(FXComposite* prt, const char* text)
  {
    return make_widget<FXMenuCascade, FXComposite>(prt, text);
  }

  //~ FXMenuRadio.h
  FXMenuRadio* FXMenuRadio_new(FXComposite* prt, const char* text)
  {
    return make_widget<FXMenuRadio, FXComposite>(prt, text);
  }
  unsigned char FXMenuRadio_get_check(const FXMenuRadio* self)
  {
    return self->getCheck();
  }
  void FXMenuRadio_set_check(FXMenuRadio* self)
  {
    self->setCheck();
  }

  //~ FXMenuCheck.h
  FXMenuCheck* FXMenuCheck_new(FXComposite* prt, const char* text)
  {
    return make_widget<FXMenuCheck, FXComposite>(prt, text);
  }
  unsigned char FXMenuCheck_get_check(const FXMenuCheck* self)
  {
    return self->getCheck();
  }
  void FXMenuCheck_set_check(FXMenuCheck* self, unsigned char check)
  {
    self->setCheck(check);
  }

  //~ FXMenuSeparator.h
  FXMenuSeparator* FXMenuSeparator_new(FXComposite* prt)
  {
    return make_widget<FXMenuSeparator, FXComposite>(prt);
  }

  //~ FXMenuCommand.h
  FXMenuCommand* FXMenuCommand_new(FXComposite* prt, const char* text)
  {
    return make_widget<FXMenuCommand, FXComposite>(prt, text);
  }
  void FXMenuCommand_set_accel_text(FXMenuCommand* self, const char* text)
  {
    self->setAccelText(text);
  }
  const char* FXMenuCommand_get_accel_text(const FXMenuCommand* self)
  {
    return string_result(self->getAccelText());
  }

  //~ FXStatusLine.h
  FXStatusLine* FXStatusLine_new(FXComposite* prt)
  {
    return make_widget<FXStatusLine, FXComposite>(prt);
  }
  const char* FXStatusLine_get_text(const FXStatusLine* self)
  {
    return string_result(self->getText());
  }
  void FXStatusLine_set_text(FXStatusLine* self, const char* text)
  {
    self->setText(text);
  }

  // ============================================================================
// STATUS WIDGETS
// ============================================================================
//~ FXStatusBar.h
  FXStatusBar* FXStatusBar_new(FXComposite* prt)
  {
    return make_widget<FXStatusBar, FXComposite>(prt);
  }
  void FXStatusBar_set_text(FXStatusBar* self, const char* text)
  {
    if (auto status = self->getStatusLine())
      status->setText(text);
  }
  const char* FXStatusBar_get_text(const FXStatusBar* self)
  {
    if (auto status = self->getStatusLine())
      return string_result(status->getText());
    return "";
  }
  void FXStatusBar_set_help_text(FXStatusBar* self, const char* text)
  {
    if (auto status = self->getStatusLine())
      status->setNormalText(text);
  }
  const char* FXStatusBar_get_help_text(const FXStatusBar* self)
  {
    if (auto status = self->getStatusLine())
      return string_result(status->getNormalText());
    return "";
  }

  //~ FXOption.h
  FXOption* FXOption_new(FXComposite* prt, const char* text)
  {
    return make_widget<FXOption, FXComposite>(prt, text);
  }

  //~ FXOptionMenu.h
  FXOptionMenu* FXOptionMenu_new(FXComposite* prt)
  {
    return make_widget<FXOptionMenu, FXComposite>(prt);
  }

  // ============================================================================
// TOOLBAR WIDGETS
// ============================================================================
//~ FXToolBar.h
  FXToolBar* FXToolBar_new(FXComposite* prt)
  {
    return make_widget<FXToolBar, FXComposite>(prt, LAYOUT_TOP | LAYOUT_LEFT | LAYOUT_FILL_X);
  }

  //~ FXToolBarGrip.h
  FXToolBarGrip* FXToolBarGrip_new(FXToolBar* toolbar)
  {
    return new FXToolBarGrip(toolbar);
  }

  //~ FXToolBarTab.h
  FXToolBarTab* FXToolBarTab_new(FXToolBar* toolbar)
  {
    return new FXToolBarTab(toolbar);
  }

  //~ FXBitmapFrame.h
  FXBitmapFrame* FXBitmapFrame_new(FXComposite* prt)
  {
    return make_widget<FXBitmapFrame, FXComposite>(prt, nullptr);
  }
  void FXBitmapFrame_set_justify(FXBitmapFrame* self, unsigned justify)
  {
    self->setJustify(justify);
  }
  unsigned FXBitmapFrame_get_justify(const FXBitmapFrame* self)
  {
    return self->getJustify();
  }

  //~ FXBitmapView.h
  FXBitmapView* FXBitmapView_new(FXComposite* prt)
  {
    return make_widget<FXBitmapView, FXComposite>(prt);
  }

  //~ FXImageFrame.h
  FXImageFrame* FXImageFrame_new(FXComposite* prt, FXImage* img)
  {
    return make_widget<FXImageFrame, FXComposite>(prt, img);
  }
  void FXImageFrame_set_justify(FXImageFrame* self, unsigned justify)
  {
    self->setJustify(justify);
  }
  unsigned FXImageFrame_get_justify(const FXImageFrame* self)
  {
    return self->getJustify();
  }
  void FXImageFrame_set_image(FXImageFrame* self, FXImage* img)
  {
    self->setImage(img);
  }
  FXImage* FXImageFrame_get_image(const FXImageFrame* self)
  {
    return self->getImage();
  }

  //~ FXImageView.h
  FXImageView* FXImageView_new(FXComposite* prt)
  {
    return make_widget<FXImageView, FXComposite>(prt);
  }
  void FXImageView_set_image(FXImageView* self, FXImage* img)
  {
    self->setImage(img);
  }
  FXImage* FXImageView_get_image(const FXImageView* self)
  {
    return self->getImage();
  }

  //~ FXFoldingList.h
  FXFoldingList* FXFoldingList_new(FXComposite* prt)
  {
    return make_widget<FXFoldingList, FXComposite>(prt);
  }

  //~ FXMDIChild.h
  FXMDIChild* FXMDIChild_new(FXMDIClient* client, const char* title)
  {
    return new FXMDIChild(client, title);
  }

  //~ FXMDIClient.h
  FXMDIClient* FXMDIClient_new(FXComposite* prt)
  {
    return make_widget<FXMDIClient, FXComposite>(prt);
  }

  //~ FXMDIDeleteButton.h
  FXMDIDeleteButton* FXMDIDeleteButton_new(FXComposite* prt)
  {
    return make_widget<FXMDIDeleteButton, FXComposite>(prt);
  }

  //~ FXMDIMaximizeButton.h
  FXMDIMaximizeButton* FXMDIMaximizeButton_new(FXComposite* prt)
  {
    return make_widget<FXMDIMaximizeButton, FXComposite>(prt);
  }

  //~ FXMDIMenu.h
  FXMDIMenu* FXMDIMenu_new(FXComposite* prt)
  {
    return make_widget<FXMDIMenu, FXComposite>(prt);
  }

  //~ FXMDIMinimizeButton.h
  FXMDIMinimizeButton* FXMDIMinimizeButton_new(FXComposite* prt)
  {
    return make_widget<FXMDIMinimizeButton, FXComposite>(prt);
  }

  //~ FXMDIRestoreButton.h
  FXMDIRestoreButton* FXMDIRestoreButton_new(FXComposite* prt)
  {
    return make_widget<FXMDIRestoreButton, FXComposite>(prt);
  }

  //~ FXMDIWindowButton.h
  FXMDIWindowButton* FXMDIWindowButton_new(FXComposite* prt, FXPopup* pup)
  {
    return make_widget<FXMDIWindowButton, FXComposite>(prt, pup);
  }

  //~ FXTableItem.h
  FXTableItem* FXTableItem_new(FXTable* tbl, const char* text)
  {
    return new FXTableItem(text, nullptr, tbl);
  }

}
