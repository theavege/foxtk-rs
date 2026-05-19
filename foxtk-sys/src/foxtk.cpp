#include <fx.h>

//~ OPAQUE HANDLES

typedef void* ObjectPtr;
typedef long (*CWidgetCb)(ObjectPtr widget, void* context);
typedef long (*CTimerCb)(ObjectPtr application, void* context);

//~ CALLBACK BRIDGE

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
//~ FXObject
    void fx_object_delete(ObjectPtr wgt) {
        if (wgt) delete static_cast<FXObject*>(wgt);
    }

//~ FXId
    ObjectPtr fx_id_get_app(ObjectPtr wgt) {
        return static_cast<FXId*>(wgt) -> getApp();
    }
    FXID fx_id_get_id(ObjectPtr wgt) {
        return static_cast<FXId*>(wgt) -> id();
    }

//~ FXWindow
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
    long fx_window_has_focus(ObjectPtr wgt) {
        return static_cast<FXWindow*>(wgt)->hasFocus();
    }

//~ FXApp
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

//~ FXLabel
    ObjectPtr fx_label_new(ObjectPtr prt, const char* title) {
        return new FXLabel(static_cast<FXComposite*>(prt), title);
    }
    const char* fx_label_get_text(ObjectPtr wgt) {
        static thread_local FXString buffer;
        buffer = static_cast<FXLabel*>(wgt)->getText();
        return buffer.text();
    }
    void fx_label_set_text(ObjectPtr wgt, const char* text) {
        static_cast<FXLabel*>(wgt) -> setText(text);
    }    void fx_label_set_help_text(ObjectPtr wgt, const char* text) {
        static_cast<FXLabel*>(wgt) -> setHelpText(text);
    }
    void fx_label_set_tip_text(ObjectPtr wgt, const char* text) {
        static_cast<FXLabel*>(wgt) -> setTipText(text);
    }

    unsigned int fx_label_get_justify(ObjectPtr wgt) {
        return static_cast<FXLabel*>(wgt)->getJustify();
    }
    void fx_label_set_justify(ObjectPtr wgt, unsigned int justify) {
        static_cast<FXLabel*>(wgt) -> setJustify(justify);
    }

//~ FXArrowButton.h
    ObjectPtr fx_arrow_button_new(ObjectPtr prt) {
        return new FXArrowButton(static_cast<FXComposite*>(prt));
    }

//~ FXButton.h
    ObjectPtr fx_button_new(ObjectPtr prt, const char* title) {
        return new FXButton(static_cast<FXComposite*>(prt), title);
    }

//~ FXCheckButton.h
    ObjectPtr fx_check_button_new(ObjectPtr prt, const char* title) {
        return new FXCheckButton(static_cast<FXComposite*>(prt), title);
    }
    unsigned char fx_check_button_get_check(ObjectPtr wgt) {
        return static_cast<FXCheckButton*>(wgt)->getCheck();
    }
    void fx_check_button_set_check(ObjectPtr wgt, unsigned char check) {
        static_cast<FXCheckButton*>(wgt)->setCheck(check);
    }

//~ FXRadioButton.h
    ObjectPtr fx_radio_button_new(ObjectPtr prt, const char* title) {
        return new FXRadioButton(static_cast<FXComposite*>(prt), title);
    }
    unsigned char fx_radio_button_get_check(ObjectPtr wgt) {
        return static_cast<FXRadioButton*>(wgt)->getCheck();
    }
    void fx_radio_button_set_check(ObjectPtr wgt) {
        static_cast<FXRadioButton*>(wgt)->setCheck();
    }

//~ FXToggleButton.h
    ObjectPtr fx_toggle_button_new(ObjectPtr prt, const char* text1, const char* text2) {
        return new FXToggleButton(static_cast<FXComposite*>(prt), text1, text2);
    }

//~ FXTextField
    ObjectPtr fx_textfield_new(ObjectPtr prt) {
        return new FXTextField(static_cast<FXComposite*>(prt), 8);
    }
    const char* fx_textfield_get_text(ObjectPtr wgt) {
        static thread_local FXString buffer;
        buffer = static_cast<FXTextField*>(wgt)->getText();
        return buffer.text();
    }
    void fx_textfield_set_text(ObjectPtr wgt, const char* text) {
        static_cast<FXTextField*>(wgt) -> setText(text);
    }
    void fx_textfield_set_help_text(ObjectPtr wgt, const char* text) {
        static_cast<FXTextField*>(wgt) -> setHelpText(text);
    }
    void fx_textfield_set_tip_text(ObjectPtr wgt, const char* text) {
        static_cast<FXTextField*>(wgt) -> setTipText(text);
    }

    void fx_textfield_set_editable(ObjectPtr wgt, long val) {
        static_cast<FXTextField*>(wgt) -> setEditable(val != 0);
    }

//~ FXSpinner
    ObjectPtr fx_spinner_new(ObjectPtr prt) {
        return new FXSpinner(static_cast<FXComposite*>(prt), 8);
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

//~ FXSlider
    ObjectPtr fx_slider_new(ObjectPtr prt) {
        return new FXSlider(static_cast<FXComposite*>(prt));
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

//~ FXProgressBar
    ObjectPtr fx_progressbar_new(ObjectPtr prt) {
        return new FXProgressBar(static_cast<FXComposite*>(prt));
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

//~ FXFrame
    void fx_frame_set_frame_style(ObjectPtr wgt, unsigned int style) {
        static_cast<FXFrame*>(wgt) -> setFrameStyle(style);
    }
    void fx_frame_set_pad_bottom(ObjectPtr wgt, int pad) {
        static_cast<FXFrame*>(wgt) -> setPadBottom(pad);
    }
    void fx_frame_set_pad_left(ObjectPtr wgt, int pad) {
        static_cast<FXFrame*>(wgt) -> setPadLeft(pad);
    }
    void fx_frame_set_pad_right(ObjectPtr wgt, int pad) {
        static_cast<FXFrame*>(wgt) -> setPadRight(pad);
    }
    void fx_frame_set_pad_top(ObjectPtr wgt, int pad) {
        static_cast<FXFrame*>(wgt) -> setPadTop(pad);
    }

//~ FXPacker
    ObjectPtr fx_packer_new(ObjectPtr prt) {
        return new FXPacker(static_cast<FXComposite*>(prt));
    }
    void fx_packer_set_hspacing(ObjectPtr wgt, int val) {
        static_cast<FXPacker*>(wgt)->setHSpacing(val);
    }
    void fx_packer_set_vspacing(ObjectPtr wgt, int val) {
        static_cast<FXPacker*>(wgt)->setVSpacing(val);
    }

//~ FXGroupBox
    ObjectPtr fx_groupbox_new(ObjectPtr prt, const char* title) {
        return new FXGroupBox(static_cast<FXComposite*>(prt), title);
    }
    void fx_groupbox_set_style(ObjectPtr wgt, unsigned int style) {
        static_cast<FXGroupBox*>(wgt)->setGroupBoxStyle(style);
    }
    void fx_groupbox_set_text(ObjectPtr wgt, const char* text) {
        static_cast<FXGroupBox*>(wgt)->setText(text);
    }

//~ FXVerticalFrame
    ObjectPtr fx_vertical_frame_new(ObjectPtr prt) {
        return new FXVerticalFrame(static_cast<FXComposite*>(prt));
    }

//~ FXHorizontalFrame
    ObjectPtr fx_horizontal_frame_new(ObjectPtr prt) {
        return new FXHorizontalFrame(static_cast<FXComposite*>(prt));
    }

//~ FXSpring
    ObjectPtr fx_spring_new(ObjectPtr prt) {
        return new FXSpring(static_cast<FXComposite*>(prt));
    }

//~ FXSwitcher
    ObjectPtr fx_switcher_new(ObjectPtr prt) {
        return new FXSwitcher(static_cast<FXComposite*>(prt));
    }

    void fx_switcher_set_current(ObjectPtr wgt, int index) {
        static_cast<FXSwitcher*>(wgt)->setCurrent(index);
    }

//~ FXMainWindow
    ObjectPtr fx_main_window_new(ObjectPtr app_, const char* title, int width, int height) {
        auto obj = static_cast<FXApp*>(app_);
        return new FXMainWindow(obj, title, nullptr, nullptr, DECOR_ALL, 0, 0, width, height);
    }
    void fx_main_window_show(ObjectPtr wgt) {
        static_cast<FXMainWindow*>(wgt)-> show(PLACEMENT_SCREEN);
    }

//~ FXComboBox
    ObjectPtr fx_combo_box_new(ObjectPtr prt, int cols) {
        return new FXComboBox(static_cast<FXComposite*>(prt), cols);
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

//~ FXList
    ObjectPtr fx_list_new(ObjectPtr prt) {
        return new FXList(static_cast<FXComposite*>(prt));
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

//~ FXListBox
    ObjectPtr fx_list_box_new(ObjectPtr prt) {
        return new FXListBox(static_cast<FXComposite*>(prt));
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

//~ FXText
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

//~ FXTreeList
    ObjectPtr fx_tree_list_new(ObjectPtr prt) {
        return new FXTreeList(static_cast<FXComposite*>(prt));
    }
    ObjectPtr fx_tree_list_append_item(ObjectPtr wgt, ObjectPtr prt, const char* text) {
        return static_cast<FXTreeList*>(wgt)->appendItem(static_cast<FXTreeItem*>(prt), text);
    }
    void fx_tree_list_clear_items(ObjectPtr wgt) {
        static_cast<FXTreeList*>(wgt)->clearItems();
    }

//~ FXTable
    ObjectPtr fx_table_new(ObjectPtr prt) {
        return new FXTable(static_cast<FXComposite*>(prt));
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

//~ FXCanvas
    ObjectPtr fx_canvas_new(ObjectPtr prt) {
        return new FXCanvas(static_cast<FXComposite*>(prt));
    }

//~ FXTabBook
    ObjectPtr fx_tab_book_new(ObjectPtr prt) {
        return new FXTabBook(static_cast<FXComposite*>(prt));
    }
    ObjectPtr fx_tab_item_new(ObjectPtr prt, const char* text) {
        return new FXTabItem(static_cast<FXTabBar*>(prt), text);
    }

//~ FXScrollBar
    ObjectPtr fx_scroll_bar_new(ObjectPtr prt) {
        return new FXScrollBar(static_cast<FXComposite*>(prt));
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

//~ FXMenuBar
    ObjectPtr fx_menu_bar_new(ObjectPtr prt) {
        return new FXMenuBar(static_cast<FXComposite*>(prt), nullptr);
    }

//~ FXMenuPane
    ObjectPtr fx_menu_pane_new(ObjectPtr prt) {
        return new FXMenuPane(static_cast<FXWindow*>(prt));
    }

//~ FXMenuButton.h
    ObjectPtr fx_menu_button_new(ObjectPtr prt, const char* title, ObjectPtr pop) {
        auto wgt = new FXMenuButton(static_cast<FXComposite*>(prt), title);
        wgt -> setMenu(static_cast<FXPopup*>(pop));
        return wgt;
    }
    void fx_menu_button_style(ObjectPtr wgt, FXuint style) {
        static_cast<FXMenuButton*>(wgt) -> setButtonStyle(style);
    }
    void fx_menu_button_popup_style(ObjectPtr wgt, FXuint style) {
        static_cast<FXMenuButton*>(wgt) -> setPopupStyle(style);
    }
    void fx_menu_button_attachment(ObjectPtr wgt, FXuint attachment) {
        static_cast<FXMenuButton*>(wgt) -> setAttachment(attachment);
    }

//~ FXMenuTitle
    ObjectPtr fx_menu_title_new(ObjectPtr prt, const char* text, ObjectPtr pop) {
        auto wgt = new FXMenuTitle(static_cast<FXComposite*>(prt), text);
        wgt -> setMenu(static_cast<FXPopup*>(pop));
        return wgt;
    }

//~ FXMenuCaption
    ObjectPtr fx_menu_caption_new(ObjectPtr prt, const char* text) {
        return new FXMenuCaption(static_cast<FXComposite*>(prt), text);
    }

//~ FXMenuCascade
    ObjectPtr fx_menu_cascade_new(ObjectPtr prt, const char* text) {
        return new FXMenuCascade(static_cast<FXComposite*>(prt), text);
    }

//~ FXMenuRadio
    ObjectPtr fx_menu_radio_new(ObjectPtr prt, const char* text) {
        return new FXMenuRadio(static_cast<FXComposite*>(prt), text);
    }

//~ FXMenuCheck
    ObjectPtr fx_menu_check_new(ObjectPtr prt, const char* text) {
        return new FXMenuCheck(static_cast<FXComposite*>(prt), text);
    }

//~ FXMenuSeparator
    ObjectPtr fx_menu_separator_new(ObjectPtr prt) {
        return new FXMenuSeparator(static_cast<FXComposite*>(prt));
    }

//~ FXMenuCommand
    ObjectPtr fx_menu_command_new(ObjectPtr prt, const char* text) {
        return new FXMenuCommand(static_cast<FXComposite*>(prt), text);
    }
    void fx_menu_command_set_accel_text(ObjectPtr wgt, const char* text) {
        static_cast<FXMenuCommand*>(wgt)->setAccelText(text);
    }
    const char* fx_menu_command_get_accel_text(ObjectPtr wgt) {
        static thread_local FXString buffer;
        buffer = static_cast<FXMenuCommand*>(wgt)->getAccelText();
        return buffer.text();
    }
}
