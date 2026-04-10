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
  long selCommand(FXObject* wgt, FXSelector, void*) {
    long result = 1;
    if (this -> callback) result = this -> callback(wgt, this -> context);
    return result;
  }
};

FXDEFMAP(CTarget) CTargetMap[] = {
    FXMAPFUNC(SEL_COMMAND, CTarget::SEL_COMMAND, CTarget::selCommand),
};
FXIMPLEMENT(CTarget, FXObject, CTargetMap, ARRAYNUMBER(CTargetMap))

class CTimeout : public FXObject {
  FXDECLARE(CTimeout)
protected:
    CTimeout() {}
private:
  ObjectPtr application = nullptr;
  CTimerCb    callback = nullptr;
  unsigned int nanosec = 0;
  void*        context = nullptr;
public:
  enum { SEL_TIMEOUT, ID_LAST };
  CTimeout(ObjectPtr app, CTimerCb cb, unsigned int ns, void* ctx) {
    this -> application = app;
    this -> callback = cb;
    this -> nanosec = ns;
    static_cast<FXApp*>(app)->addTimeout(this, CTimeout::SEL_TIMEOUT, ns, ctx);
  }
  long onTimeout(FXObject*, FXSelector, void* ctx) {
        long result = 1;
        if (this -> callback) {
            result = this -> callback(this -> application, ctx);
            static_cast<FXApp*>(application)->addTimeout(this, CTimeout::SEL_TIMEOUT, nanosec, ctx);
        };
        return result;
    }
};

FXDEFMAP(CTimeout) CTimeoutMap[] = {
    FXMAPFUNC(SEL_TIMEOUT, CTimeout::SEL_TIMEOUT, CTimeout::onTimeout),
};
FXIMPLEMENT(CTimeout, FXObject, CTimeoutMap, ARRAYNUMBER(CTimeoutMap))

extern "C" {
// FXObject
    void fx_object_delete(ObjectPtr wgt) {
        delete static_cast<FXObject*>(wgt);
    }

// FXId
    ObjectPtr fx_id_get_app(ObjectPtr wgt) {
        return static_cast<FXId*>(wgt) -> getApp();
    }

// FXWindow
    void fx_window_set_target(ObjectPtr wgt_, CWidgetCb cb, void* ctx) {
        auto wgt = static_cast<FXWindow*>(wgt_);
        wgt->setTarget(static_cast<FXObject*>(new CTarget(cb, ctx)));
        wgt->setSelector(CTarget::SEL_COMMAND);
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
    ObjectPtr fx_app_add_timeout(ObjectPtr app, CTimerCb cb, unsigned int ns, void* ctx) {
        return new CTimeout(app, cb, ns, ctx);
    }

// FXLabel
    ObjectPtr fx_label_new(ObjectPtr parent, const char* title) {
        return new FXLabel(static_cast<FXComposite*>(parent), title);
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
    ObjectPtr fx_button_new(ObjectPtr parent, const char* title) {
        return new FXButton(static_cast<FXComposite*>(parent), title);
    }

// FXRadioButton
    ObjectPtr fx_radio_button_new(ObjectPtr parent, const char* title) {
        return new FXRadioButton(static_cast<FXComposite*>(parent), title);
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
    ObjectPtr fx_textfield_new(ObjectPtr parent, int ncols) {
        return new FXTextField(static_cast<FXComposite*>(parent), ncols);
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
    ObjectPtr fx_spinner_new(ObjectPtr parent, int cols) {
        return new FXSpinner(static_cast<FXComposite*>(parent), cols);
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
    ObjectPtr fx_slider_new(ObjectPtr parent) {
        return new FXSlider(static_cast<FXComposite*>(parent));
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
    ObjectPtr fx_progressbar_new(ObjectPtr parent) {
        return new FXProgressBar(static_cast<FXComposite*>(parent));
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

// FXVerticalFrame
    ObjectPtr fx_vertical_frame_new(ObjectPtr parent) {
        return new FXVerticalFrame(static_cast<FXComposite*>(parent));
    }

// FXHorizontalFrame
    ObjectPtr fx_horizontal_frame_new(ObjectPtr parent) {
        return new FXHorizontalFrame(static_cast<FXComposite*>(parent));
    }

// FXPacker
    ObjectPtr fx_packer_new(ObjectPtr parent) {
        return new FXPacker(static_cast<FXComposite*>(parent));
    }

// FXGroupBox
    ObjectPtr fx_groupbox_new(ObjectPtr parent, const char* title) {
        return new FXGroupBox(static_cast<FXComposite*>(parent), title);
    }

// FXSwitcher
    ObjectPtr fx_switcher_new(ObjectPtr parent) {
        return new FXSwitcher(static_cast<FXComposite*>(parent));
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
    void fx_combo_box_append_item(ObjectPtr wgt, const char* text, void* ptr) {
        static_cast<FXComboBox*>(wgt)->appendItem(text, ptr);
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

// FXListBox
    ObjectPtr fx_list_box_new(ObjectPtr parent) {
        return new FXListBox(static_cast<FXComposite*>(parent));
    }
    void fx_list_box_append_item(ObjectPtr wgt, const char* text, void* ptr) {
        static_cast<FXListBox*>(wgt)->appendItem(text, nullptr, ptr);
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
    ObjectPtr fx_text_new(ObjectPtr parent) {
        return new FXText(static_cast<FXComposite*>(parent));
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
    ObjectPtr fx_tab_item_new(ObjectPtr parent_, const char* text, ObjectPtr icon, unsigned int opts, int x, int y, int w, int h, int pl, int pr, int pt, int pb) {
        auto tabbar = static_cast<FXTabBar*>(parent_);
        return new FXTabItem(tabbar, text, static_cast<FXIcon*>(icon), opts, x, y, w, h, pl, pr, pt, pb);
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
    ObjectPtr fx_menu_bar_new(ObjectPtr parent_, ObjectPtr g_) {
        auto parent = static_cast<FXComposite*>(parent_);
        auto g = static_cast<FXComposite*>(g_);
        return new FXMenuBar(parent, g);
    }
    ObjectPtr fx_menu_pane_new(ObjectPtr parent, unsigned int opts) {
        return new FXMenuPane(static_cast<FXWindow*>(parent), opts);
    }
    ObjectPtr fx_menu_title_new(ObjectPtr parent_, const char* text, ObjectPtr icon, ObjectPtr pup, unsigned int opts) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXMenuTitle(parent, text, static_cast<FXIcon*>(icon), static_cast<FXPopup*>(pup), opts);
    }
    ObjectPtr fx_menu_command_new(ObjectPtr parent_, const char* text, ObjectPtr icon, ObjectPtr tgt, int sel, unsigned int opts) {
        auto parent = static_cast<FXComposite*>(parent_);
        return new FXMenuCommand(parent, text, static_cast<FXIcon*>(icon), static_cast<FXObject*>(tgt), sel, opts);
    }
}
