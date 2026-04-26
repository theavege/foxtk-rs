#include <fx.h>

typedef void* ObjectPtr;
typedef long (*CWidgetCb)(ObjectPtr widget, void* context);
typedef long (*CTimerCb)(ObjectPtr application, void* context);

// CALLBACK BRIDGE

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
            static_cast<FXApp*>(app)->addTimeout(this, CTimeout::SEL_TIMEOUT, nanosec, ctx);
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

extern "C" {
// FXObject
    void fx_object_delete(ObjectPtr wgt) {
        if (wgt) delete static_cast<FXObject*>(wgt);
    }

// FXId
    ObjectPtr fx_id_get_app(ObjectPtr wgt) {
        return static_cast<FXId*>(wgt) -> getApp();
    }

// FXWindow
    void fx_window_set_target(ObjectPtr wgt, CWidgetCb cb, void* ctx) {
        static_cast<FXWindow*>(wgt)->setTarget(static_cast<FXObject*>(new CTarget(cb, ctx)));
    }
    void fx_window_set_selector(ObjectPtr wgt_, int val) {
        auto wgt = static_cast<FXWindow*>(wgt_);
        if (val == 0) wgt->setSelector(CTarget::SEL_COMMAND);
        if (val == 1) wgt->setSelector(CTarget::SEL_CHANGED);
    }
    void fx_window_set_width(ObjectPtr wgt, int val) {
        static_cast<FXWindow*>(wgt)->setWidth(val);
    }
    void fx_window_set_height(ObjectPtr wgt, int val) {
        static_cast<FXWindow*>(wgt)->setHeight(val);
    }
    void fx_window_set_layout_hints(ObjectPtr wgt, unsigned int val) {
        static_cast<FXWindow*>(wgt)->setLayoutHints(val);
    }
    FXWindow* fx_window_get_parent(ObjectPtr wgt) {
        return static_cast<FXWindow*>(wgt)->getParent();
    }

// FXApp
    ObjectPtr fx_app_new(const char* name, const char* vendor, int argc, char** argv) {
        auto app = new FXApp(name, vendor);
        app->init(argc, argv);
        return app;
    }
    int fx_app_run(ObjectPtr app_) {
        auto app = static_cast<FXApp*>(app_);
        app->create();
        return app->run();
    }
    void fx_app_add_timeout(ObjectPtr app, CTimerCb cb, unsigned int ns, void* ctx) {
        static_cast<FXApp*>(app)->addTimeout(new CTimeout(cb, ns), CTimeout::SEL_TIMEOUT, ns, ctx);
    }
    void fx_app_add_chore(ObjectPtr app, CTimerCb cb, void* ctx) {
        static_cast<FXApp*>(app)->addChore(new CTimeout(cb, 0), CTimeout::SEL_CHORE, ctx);
    }

// FXLabel
    ObjectPtr fx_label_new(ObjectPtr prt, const char* title) {
        auto wgt = new FXLabel(static_cast<FXComposite*>(prt), title);
        wgt -> setLayoutHints(LAYOUT_FILL_X);
        return wgt;
    }
    const char* fx_label_get_text(ObjectPtr wgt) {
        static thread_local FXString buffer;
        buffer = static_cast<FXLabel*>(wgt)->getText();
        return buffer.text();
    }
    void fx_label_set_text(ObjectPtr wgt, const char* text) {
        static_cast<FXLabel*>(wgt) -> setText(text);
    }
    unsigned int fx_label_get_justify(ObjectPtr wgt) {
        return static_cast<FXLabel*>(wgt)->getJustify();
    }
    void fx_label_set_justify(ObjectPtr wgt, unsigned int justify) {
        static_cast<FXLabel*>(wgt) -> setJustify(justify);
    }

// FXButton
    ObjectPtr fx_button_new(ObjectPtr prt, const char* title) {
        auto wgt = new FXButton(static_cast<FXComposite*>(prt), title);
        wgt -> setLayoutHints(LAYOUT_FILL_X);
        return wgt;    }

// FXRadioButton
    ObjectPtr fx_radio_button_new(ObjectPtr prt, const char* title) {
        auto wgt = new FXRadioButton(static_cast<FXComposite*>(prt), title);
        wgt -> setLayoutHints(LAYOUT_FILL_X);
        return wgt;
    }
    unsigned char fx_radio_button_get_check(ObjectPtr wgt) {
        return static_cast<FXRadioButton*>(wgt)->getCheck();
    }
    void fx_radio_button_set_check(ObjectPtr wgt) {
        static_cast<FXRadioButton*>(wgt)->setCheck();
    }

// FXCheckButton
    ObjectPtr fx_check_button_new(ObjectPtr parent, const char* title) {
        return new FXCheckButton(static_cast<FXComposite*>(parent), title);
    }
    unsigned char fx_check_button_get_check(ObjectPtr wgt) {
        return static_cast<FXCheckButton*>(wgt)->getCheck();
    }
    void fx_check_button_set_check(ObjectPtr wgt, unsigned char check) {
        static_cast<FXCheckButton*>(wgt)->setCheck(check);
    }

// FXTextField
    ObjectPtr fx_textfield_new(ObjectPtr parent) {
        auto wgt = new FXTextField(static_cast<FXComposite*>(parent), 8);
        wgt -> setLayoutHints(LAYOUT_FILL_X);
        return wgt;
    }
    const char* fx_textfield_get_text(ObjectPtr wgt) {
        static thread_local FXString buffer;
        buffer = static_cast<FXTextField*>(wgt)->getText();
        return buffer.text();
    }
    void fx_textfield_set_text(ObjectPtr wgt, const char* text) {
        static_cast<FXTextField*>(wgt) -> setText(text);
    }

// FXSpinner
    ObjectPtr fx_spinner_new(ObjectPtr prt) {
        auto wgt = new FXSpinner(static_cast<FXComposite*>(prt), 8);
        wgt -> setLayoutHints(LAYOUT_FILL_X);
        return wgt;
    }
    int fx_spinner_get_value(ObjectPtr wgt) {
        return static_cast<FXSpinner*>(wgt)->getValue();
    }
    void fx_spinner_set_value(ObjectPtr wgt, int value) {
        static_cast<FXSpinner*>(wgt)->setValue(value);
    }
    void fx_spinner_get_range(ObjectPtr wgt, int* lo, int* hi) {
        FXint lower, upper;
        static_cast<FXSpinner*>(wgt)->getRange(lower, upper);
        if (lo) *lo = lower;
        if (hi) *hi = upper;
    }
    void fx_spinner_set_range(ObjectPtr wgt, int lo, int hi) {
        static_cast<FXSpinner*>(wgt)->setRange(lo, hi);
    }
    void fx_spinner_set_increment(ObjectPtr wgt, int inc) {
        static_cast<FXSpinner*>(wgt)->setIncrement(inc);
    }
    void fx_spinner_increment(ObjectPtr wgt) {
        static_cast<FXSpinner*>(wgt)->increment();
    }
    void fx_spinner_decrement(ObjectPtr wgt) {
        static_cast<FXSpinner*>(wgt)->decrement();
    }

// FXSlider
    ObjectPtr fx_slider_new(ObjectPtr prt) {
        auto wgt = new FXSlider(static_cast<FXComposite*>(prt));
        wgt -> setLayoutHints(LAYOUT_FILL_X);
        return wgt;
    }
    int fx_slider_get_value(ObjectPtr wgt) {
        return static_cast<FXSlider*>(wgt)->getValue();
    }
    void fx_slider_set_value(ObjectPtr wgt, int value) {
        static_cast<FXSlider*>(wgt)->setValue(value);
    }
    void fx_slider_get_range(ObjectPtr wgt, int* lo, int* hi) {
        FXint lower, upper;
        static_cast<FXSlider*>(wgt)->getRange(lower, upper);
        if (lo) *lo = lower;
        if (hi) *hi = upper;
    }
    void fx_slider_set_range(ObjectPtr wgt, int lo, int hi) {
        static_cast<FXSlider*>(wgt)->setRange(lo, hi);
    }
    int fx_slider_get_increment(ObjectPtr wgt) {
        return static_cast<FXSlider*>(wgt)->getIncrement();
    }
    void fx_slider_set_increment(ObjectPtr wgt, int inc) {
        static_cast<FXSlider*>(wgt)->setIncrement(inc);
    }

// FXProgressBar
    ObjectPtr fx_progressbar_new(ObjectPtr prt) {
        auto wgt = new FXProgressBar(static_cast<FXComposite*>(prt));
        wgt -> setLayoutHints(LAYOUT_FILL_X);
        return wgt;
    }
    void fx_progressbar_set_progress(ObjectPtr wgt, unsigned int value) {
        static_cast<FXProgressBar*>(wgt)->setProgress(value);
    }
    unsigned int fx_progressbar_get_progress(ObjectPtr wgt) {
        return static_cast<FXProgressBar*>(wgt)->getProgress();
    }
    void fx_progressbar_set_total(ObjectPtr wgt, unsigned int value) {
        static_cast<FXProgressBar*>(wgt)->setTotal(value);
    }
    unsigned int fx_progressbar_get_total(ObjectPtr wgt) {
        return static_cast<FXProgressBar*>(wgt)->getTotal();
    }
    void fx_progressbar_increment(ObjectPtr wgt, unsigned int value) {
        static_cast<FXProgressBar*>(wgt)->increment(value);
    }
    void fx_progressbar_show_number(ObjectPtr wgt) {
        static_cast<FXProgressBar*>(wgt)->showNumber();
    }
    void fx_progressbar_hide_number(ObjectPtr wgt) {
        static_cast<FXProgressBar*>(wgt)->hideNumber();
    }
    void fx_progressbar_set_bar_size(ObjectPtr wgt, int size) {
        static_cast<FXProgressBar*>(wgt)->setBarSize(size);
    }
    int fx_progressbar_get_bar_size(ObjectPtr wgt) {
        return static_cast<FXProgressBar*>(wgt)->getBarSize();
    }

// FXPacker
    ObjectPtr fx_packer_new(ObjectPtr parent) {
        return new FXPacker(static_cast<FXComposite*>(parent));
    }
    void fx_packer_set_hspacing(ObjectPtr wgt, int val) {
        static_cast<FXPacker*>(wgt)->setHSpacing(val);
    }
    void fx_packer_set_vspacing(ObjectPtr wgt, int val) {
        static_cast<FXPacker*>(wgt)->setVSpacing(val);
    }

// FXGroupBox
    ObjectPtr fx_groupbox_new(ObjectPtr prt, const char* title) {
        auto wgt = new FXGroupBox(static_cast<FXComposite*>(prt), title);
        wgt -> setLayoutHints(LAYOUT_FILL);
        return wgt;
    }
    void fx_groupbox_set_style(ObjectPtr wgt, unsigned int val) {
        static_cast<FXGroupBox*>(wgt)->setGroupBoxStyle(val);
    }

// FXVerticalFrame
    ObjectPtr fx_vertical_frame_new(ObjectPtr prt) {
        return new FXVerticalFrame(static_cast<FXComposite*>(prt));
    }

// FXHorizontalFrame
    ObjectPtr fx_horizontal_frame_new(ObjectPtr prt) {
        auto wgt = new FXHorizontalFrame(static_cast<FXComposite*>(prt));
        wgt -> setLayoutHints(LAYOUT_FILL_X);
        return wgt;
    }

// FXSpring
    ObjectPtr fx_spring_new(ObjectPtr parent) {
        return new FXSpring(static_cast<FXComposite*>(parent));
    }

// FXSwitcher
    ObjectPtr fx_switcher_new(ObjectPtr prt) {
        return new FXSwitcher(static_cast<FXComposite*>(prt));
    }

    void fx_switcher_set_current(ObjectPtr wgt, int index) {
        static_cast<FXSwitcher*>(wgt)->setCurrent(index);
    }

// FXMainWindow
    ObjectPtr fx_main_window_new(ObjectPtr app_, const char* title, int width, int height) {
        auto obj = static_cast<FXApp*>(app_);
        return new FXMainWindow(obj, title, nullptr, nullptr, DECOR_ALL, 0, 0, width, height);
    }
    void fx_main_window_show(ObjectPtr wgt) {
        static_cast<FXMainWindow*>(wgt)-> show(PLACEMENT_SCREEN);
    }

// FXComboBox
    ObjectPtr fx_combo_box_new(ObjectPtr parent, int cols) {
        return new FXComboBox(static_cast<FXComposite*>(parent), cols);
    }
    void fx_combo_box_append_item(ObjectPtr wgt, const char* text) {
        static_cast<FXComboBox*>(wgt)->appendItem(text);
    }
    void fx_combo_box_clear_items(ObjectPtr wgt) {
        static_cast<FXComboBox*>(wgt)->clearItems();
    }
    int fx_combo_box_get_current_item(ObjectPtr wgt) {
        return static_cast<FXComboBox*>(wgt)->getCurrentItem();
    }
    void fx_combo_box_set_current_item(ObjectPtr wgt, int index) {
        static_cast<FXComboBox*>(wgt)->setCurrentItem(index);
    }
    const char* fx_combo_box_get_item_text(ObjectPtr wgt, int index) {
        static thread_local FXString buffer;
        buffer = static_cast<FXComboBox*>(wgt)->getItemText(index);
        return buffer.text();
    }
    int fx_combo_box_get_num_items(ObjectPtr wgt) {
        return static_cast<FXComboBox*>(wgt)->getNumItems();
    }

// FXList
    ObjectPtr fx_list_new(ObjectPtr parent) {
        return new FXList(static_cast<FXComposite*>(parent));
    }
    void fx_list_append_item(ObjectPtr wgt, const char* text) {
        static_cast<FXList*>(wgt)->appendItem(text);
    }
    void fx_list_clear_items(ObjectPtr wgt) {
        static_cast<FXList*>(wgt)->clearItems();
    }
    int fx_list_get_current_item(ObjectPtr wgt) {
        return static_cast<FXList*>(wgt)->getCurrentItem();
    }
    void fx_list_set_current_item(ObjectPtr wgt, int index) {
        static_cast<FXList*>(wgt)->setCurrentItem(index);
    }
    const char* fx_list_get_item_text(ObjectPtr wgt, int index) {
        static thread_local FXString buffer;
        buffer = static_cast<FXList*>(wgt)->getItemText(index);
        return buffer.text();
    }
    int fx_list_get_num_items(ObjectPtr wgt) {
        return static_cast<FXList*>(wgt)->getNumItems();
    }

// FXListBox
    ObjectPtr fx_list_box_new(ObjectPtr parent) {
        return new FXListBox(static_cast<FXComposite*>(parent));
    }
    void fx_list_box_append_item(ObjectPtr wgt, const char* text) {
        static_cast<FXListBox*>(wgt)->appendItem(text);
    }
    void fx_list_box_clear_items(ObjectPtr wgt) {
        static_cast<FXListBox*>(wgt)->clearItems();
    }
    int fx_list_box_get_current_item(ObjectPtr wgt) {
        return static_cast<FXListBox*>(wgt)->getCurrentItem();
    }
    void fx_list_box_set_current_item(ObjectPtr wgt, int index) {
        static_cast<FXListBox*>(wgt)->setCurrentItem(index);
    }
    const char* fx_list_box_get_item_text(ObjectPtr wgt, int index) {
        static thread_local FXString buffer;
        buffer = static_cast<FXListBox*>(wgt)->getItemText(index);
        return buffer.text();
    }
    int fx_list_box_get_num_items(ObjectPtr wgt) {
        return static_cast<FXListBox*>(wgt)->getNumItems();
    }

// FXText
    ObjectPtr fx_text_new(ObjectPtr prt) {
        return new FXText(static_cast<FXComposite*>(prt));
    }
    void fx_text_set_text(ObjectPtr wgt, const char* text) {
        static_cast<FXText*>(wgt)->setText(text);
    }
    const char* fx_text_get_text(ObjectPtr wgt) {
        static thread_local FXString buffer;
        buffer = static_cast<FXText*>(wgt)->getText();
        return buffer.text();
    }

// FXTreeList
    ObjectPtr fx_tree_list_new(ObjectPtr parent) {
        return new FXTreeList(static_cast<FXComposite*>(parent));
    }
    ObjectPtr fx_tree_list_append_item(ObjectPtr wgt, ObjectPtr parent_item, const char* text, void* openicon, void* closedicon, void* ptr) {
        return static_cast<FXTreeList*>(wgt)->appendItem(static_cast<FXTreeItem*>(parent_item), text, static_cast<FXIcon*>(openicon), static_cast<FXIcon*>(closedicon), ptr);
    }
    void fx_tree_list_clear_items(ObjectPtr wgt) {
        static_cast<FXTreeList*>(wgt)->clearItems();
    }

// FXTable
    ObjectPtr fx_table_new(ObjectPtr parent) {
        return new FXTable(static_cast<FXComposite*>(parent));
    }
    void fx_table_set_table_size(ObjectPtr wgt, int nr, int nc) {
        static_cast<FXTable*>(wgt)->setTableSize(nr, nc);
    }
    void fx_table_set_item_text(ObjectPtr wgt, int r, int c, const char* text) {
        static_cast<FXTable*>(wgt)->setItemText(r, c, text);
    }
    const char* fx_table_get_item_text(ObjectPtr wgt, int r, int c) {
        static thread_local FXString buffer;
        buffer = static_cast<FXTable*>(wgt)->getItemText(r, c);
        return buffer.text();
    }

// FXCanvas
    ObjectPtr fx_canvas_new(ObjectPtr parent) {
        return new FXCanvas(static_cast<FXComposite*>(parent));
    }

// FXTabBook
    ObjectPtr fx_tab_book_new(ObjectPtr parent) {
        return new FXTabBook(static_cast<FXComposite*>(parent));
    }
    ObjectPtr fx_tab_item_new(ObjectPtr parent, const char* text) {
        return new FXTabItem(static_cast<FXTabBar*>(parent), text);
    }

// FXScrollBar
    ObjectPtr fx_scroll_bar_new(ObjectPtr parent) {
        return new FXScrollBar(static_cast<FXComposite*>(parent));
    }
    int fx_scroll_bar_get_position(ObjectPtr wgt) {
        return static_cast<FXScrollBar*>(wgt)->getPosition();
    }
    void fx_scroll_bar_set_position(ObjectPtr wgt, int pos) {
        static_cast<FXScrollBar*>(wgt)->setPosition(pos);
    }
    void fx_scroll_bar_set_range(ObjectPtr wgt, int hi) {
        static_cast<FXScrollBar*>(wgt)->setRange(hi);
    }

// FXMenuBar
    ObjectPtr fx_menu_bar_new(ObjectPtr parent) {
        return new FXMenuBar(static_cast<FXComposite*>(parent), nullptr);
    }

// FXMenuPane
    ObjectPtr fx_menu_pane_new(ObjectPtr parent) {
        return new FXMenuPane(static_cast<FXWindow*>(parent));
    }

// FXMenuTitle
    ObjectPtr fx_menu_title_new(ObjectPtr prt, const char* text, ObjectPtr pop) {
        auto wgt = new FXMenuTitle(static_cast<FXComposite*>(prt), text);
        wgt -> setMenu(static_cast<FXPopup*>(pop));
        return wgt;
    }

    // FXMenuCommand
    ObjectPtr fx_menu_command_new(ObjectPtr parent, const char* text) {
        return new FXMenuCommand(static_cast<FXComposite*>(parent), text);
    }
    void fx_menu_command_set_accel_text(ObjectPtr parent, const char* text) {
        static_cast<FXMenuCommand*>(parent)->setAccelText(text);
    }
    const char* fx_menu_command_get_accel_text(ObjectPtr parent) {
        static thread_local FXString buffer;
        buffer = static_cast<FXMenuCommand*>(parent)->getAccelText();
        return buffer.text();
    }
}
