#ifdef __cplusplus
extern "C" {
#endif

// Opaque handles
typedef void* FXWidgetPtr;
typedef void* FXAppPtr;
typedef long (*CWidgetCb)(FXWidgetPtr widget, void* context);
typedef long (*CTimerCb)(FXAppPtr application, void* context);
typedef void* CTargetPtr;
typedef void* CTimeoutPtr;
typedef void* FXParentPtr;

// APPLICATION
FXAppPtr fox_app_new(const char* name, const char* vendor);
void fox_app_init(FXAppPtr app, int argc, char** argv);
int fox_app_run(FXAppPtr app);
CTimeoutPtr fox_app_add_timeout(FXAppPtr app, CTimerCb cb, unsigned int ns, void* ctx);

// WINDOW
FXParentPtr fox_main_window_new(FXAppPtr app, const char* title, int width, int height);
void fox_main_window_show(FXAppPtr window);

// FRAME
FXParentPtr fox_vertical_frame_new(FXParentPtr win);

// BUTTON
FXWidgetPtr fox_button_new(FXParentPtr frm, const char* title);
void fox_button_set_target(FXWidgetPtr btn, CWidgetCb callback, void* context);
unsigned int fox_button_get_state(FXWidgetPtr btn);
const char* fox_button_get_text(FXWidgetPtr btn);

// TextField
FXWidgetPtr fox_textfield_new(FXParentPtr frm, int ncols);
void fox_textfield_set_text(FXWidgetPtr wgt, const char* text);

#ifdef __cplusplus
}
#endif
