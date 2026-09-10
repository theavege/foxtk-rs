#include "foxtk.h"

// Here we begin
int
main(int argc, char* argv[])
{

  // Make an application
  FXApp* app = FXApp_new("Scribble", "FoxTest", argc, argv);

  // Add a main window
  FXMainWindow* win = FXMainWindow_new(app, "Scribble", 640, 400);
  FXMainWindow_show(win);

  // Run the application
  return FXApp_run(app);
}
