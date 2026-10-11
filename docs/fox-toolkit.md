# FOX Toolkit Tutorial

A practical introduction to the [FOX Toolkit](http://www.fox-toolkit.org), a C++ GUI library, built from the official documentation.

**Sources:** the *Introduction*, *Targets and Messages*, *Data Targets*, and *Timers, Chores, Signals, and Inputs* pages on fox-toolkit.org. The prose and examples are my own, written to illustrate what those pages describe. I have **not compiled** the examples, so check them against your FOX version's headers. Pages I did not read are listed at the end.

---

## 1. What FOX is

FOX is a C++ toolkit for building graphical user interfaces. Its stated design goal is ease of programming: most widgets are created with a single line of C++, most constructor parameters have sensible defaults, and layout managers handle placement, so you rarely compute coordinates by hand.

The FOX library is licensed under the LGPL. If you ship software built on it, the project asks that you credit it somewhere visible (for example, the About box): *This software uses the FOX Toolkit (http://www.fox-toolkit.org).*

---

## 2. The one idea you must understand: targets and messages

Most toolkits connect GUI events to your code with callbacks or signal/slot connections. FOX uses a **target/message system** instead.

Every widget is constructed with a **target** (an object) and a **message selector**. When the user does something, the widget sends that message to its target.

A message selector has two parts:

| Part | Answers | Examples |
|---|---|---|
| **type** | *What happened?* | `SEL_COMMAND`, `SEL_UPDATE`, `SEL_PAINT`, `SEL_TIMEOUT` |
| **id** | *Who is it about / which source?* | your own `ID_INCREMENT`, or built-in ones like `FXApp::ID_QUIT` |

A few consequences follow from this design:

- **Anything derived from `FXObject` can receive messages.** That includes every widget and the `FXApp` object itself.
- **IDs only need to be unique per target class** (including its base classes), not across the whole program. Components written by different people don't have to coordinate numbering.
- **Targets and selectors can be changed at run time** (`setTarget()` / `setSelector()`), which suits GUI builders and dynamic UIs.
- **Widgets can receive messages too.** A menu item can target a toolbar directly and toggle it, with no glue code.

### Handler signature

Every message handler has the same shape:

```cpp
long onSomething(FXObject* sender, FXSelector sel, void* ptr);
```

- `sender`: the object that sent the message (always an `FXObject`).
- `sel`: the combined type + id. Use `FXSELTYPE(sel)` and `FXSELID(sel)` to take it apart.
- `ptr`: extra data. For input events this is usually an `FXEvent*`.

**Return value:** return `1` if you handled the message, `0` if you didn't. For messages that come from user input, returning `1` tells FOX that something may have changed, so it refreshes the GUI during the next idle period.

---

## 3. Anatomy of a FOX class

Any class that receives messages follows the same five-step pattern.

1. **Derive** (directly or indirectly) from `FXObject`.
2. Put **`FXDECLARE(YourClass)`** at the top of the class body.
3. Provide a **protected default constructor** (needed by the macro's serialization support).
4. Define your message IDs in an **`enum`** that starts from the base class's `ID_LAST`.
5. In the `.cpp` file, build a **message map** with `FXDEFMAP` / `FXMAPFUNC` and finish with **`FXIMPLEMENT`**.

### Message numbering with enums

```cpp
class MyWindow : public FXMainWindow {
public:
  enum {
    ID_FIRST = FXMainWindow::ID_LAST,   // continue where the base class stops
    ID_SECOND,
    ID_LAST                             // so subclasses can continue after us
  };
};
```

The compiler does the counting. If the base class gains new IDs later, yours are renumbered automatically on recompile.

### The message map

```cpp
FXDEFMAP(MyWindow) MyWindowMap[] = {
  //            type          id                 handler
  FXMAPFUNC(SEL_COMMAND, MyWindow::ID_FIRST,  MyWindow::onCmdFirst),
  FXMAPFUNC(SEL_UPDATE,  MyWindow::ID_FIRST,  MyWindow::onUpdFirst),
};

FXIMPLEMENT(MyWindow, FXMainWindow, MyWindowMap, ARRAYNUMBER(MyWindowMap))
```

Notes on the macros:

- `FXMAPFUNC(type, id, handler)` maps one message.
- `FXMAPFUNCS(type, lo, hi, handler)` maps a **range** of IDs to one handler. This is handy for, say, ten digit buttons: read the digit with `FXSELID(sel)`.
- `FXMAPTYPE` / `FXMAPTYPES` match on the type only and ignore the id (rarely needed).
- `FXIMPLEMENT(Class, BaseClass, map, count)`. If the class handles no messages, pass `NULL, 0`.
- Lookup runs **from the derived class up to the base class**, so you can override the behavior of built-in widgets by handling the message in your subclass.
- The map is searched at run time, so put frequently seen messages (like `SEL_PAINT`) first.
- IDs must fall in `MINKEY..MAXKEY`. ID `0` is reserved for system-originated messages.

---

## 4. Your first application: a counter

This small program shows the full pattern: a main window, a message map, command and update handlers, and a data target. It's based on the same concepts as the documentation's Scribble example, with different code.

```cpp
#include "fx.h"

class CounterWindow : public FXMainWindow {
  FXDECLARE(CounterWindow)

private:
  FXint        count;          // plain application data
  FXDataTarget countTarget;    // binds `count` to widgets (declared after count)

protected:
  CounterWindow() : count(0), countTarget(count) {}   // required by FXDECLARE

public:
  enum {
    ID_INC = FXMainWindow::ID_LAST,
    ID_DEC,
    ID_RESET,
    ID_LAST
  };

  long onCmdInc  (FXObject*, FXSelector, void*);
  long onCmdDec  (FXObject*, FXSelector, void*);
  long onCmdReset(FXObject*, FXSelector, void*);
  long onUpdReset(FXObject*, FXSelector, void*);

  CounterWindow(FXApp* app);
  virtual void create();
};

FXDEFMAP(CounterWindow) CounterWindowMap[] = {
  FXMAPFUNC(SEL_COMMAND, CounterWindow::ID_INC,   CounterWindow::onCmdInc),
  FXMAPFUNC(SEL_COMMAND, CounterWindow::ID_DEC,   CounterWindow::onCmdDec),
  FXMAPFUNC(SEL_COMMAND, CounterWindow::ID_RESET, CounterWindow::onCmdReset),
  FXMAPFUNC(SEL_UPDATE,  CounterWindow::ID_RESET, CounterWindow::onUpdReset),
};

FXIMPLEMENT(CounterWindow, FXMainWindow, CounterWindowMap, ARRAYNUMBER(CounterWindowMap))

CounterWindow::CounterWindow(FXApp* app)
  : FXMainWindow(app, "Counter", NULL, NULL, DECOR_ALL, 0, 0, 300, 100),
    count(0), countTarget(count) {

  FXVerticalFrame* col = new FXVerticalFrame(this, LAYOUT_FILL_X | LAYOUT_FILL_Y);

  // The text field shows `count` through the data target.
  new FXTextField(col, 8, &countTarget, FXDataTarget::ID_VALUE,
                  TEXTFIELD_INTEGER | TEXTFIELD_READONLY | LAYOUT_FILL_X);

  FXHorizontalFrame* row = new FXHorizontalFrame(col, LAYOUT_FILL_X);
  new FXButton(row, "&-",     NULL, this,    ID_DEC);
  new FXButton(row, "&+",     NULL, this,    ID_INC);
  new FXButton(row, "&Reset", NULL, this,    ID_RESET);
  new FXButton(row, "&Quit",  NULL, getApp(), FXApp::ID_QUIT);   // built-in message
}

void CounterWindow::create() {
  FXMainWindow::create();        // create server-side windows
  show(PLACEMENT_SCREEN);        // then show
}

long CounterWindow::onCmdInc  (FXObject*, FXSelector, void*) { count++; return 1; }
long CounterWindow::onCmdDec  (FXObject*, FXSelector, void*) { count--; return 1; }
long CounterWindow::onCmdReset(FXObject*, FXSelector, void*) { count = 0; return 1; }

// GUI update: enable Reset only when there is something to reset.
long CounterWindow::onUpdReset(FXObject* sender, FXSelector, void*) {
  sender->handle(this,
                 FXSEL(SEL_COMMAND, count ? FXWindow::ID_ENABLE : FXWindow::ID_DISABLE),
                 NULL);
  return 1;
}

int main(int argc, char* argv[]) {
  FXApp application("Counter", "Example");   // application key, vendor key
  application.init(argc, argv);
  new CounterWindow(&application);
  application.create();
  return application.run();                  // does not return until quit
}
```

### What to notice

- **Widget creation is one line.** `new FXButton(parent, "text", icon, target, id, options)`: the parent comes first, and the target and message come right after the icon.
- **`&` in a caption** creates an Alt-hotkey and underlines the letter.
- **`FXApp::ID_QUIT` needs no handler from you.** The application object already understands it, so the Quit button targets `getApp()` directly.
- **Handlers only change data.** `onCmdInc` just does `count++`. It never touches the text field; the data target and the update phase handle that (see sections 5 and 7).
- **`onUpdReset` doesn't call `button->enable()`.** It sends a message back to whoever asked. More on that in section 5.

### The `main()` sequence

```text
FXApp app(appKey, vendorKey);   // keys select the registry/settings location
app.init(argc, argv);           // also strips FOX-specific command-line args
new MyWindow(&app);             // construct the widget tree (client-side only)
app.create();                   // create the real windows for the whole tree
return app.run();               // event loop
```

FOX builds widgets in **two stages**. Constructors only build C++ objects. `create()` makes the actual windows, which can only happen once the display connection exists. That is why a window's constructor builds the GUI while `create()` is a separate, virtual step that you call after the whole tree is built.

### Building

Link against the FOX library and its dependencies. Header and library names depend on version and distribution, so the easiest approach is to ask the install (not verified here):

```sh
g++ counter.cpp -o counter $(fox-config --cflags --libs)
# or, depending on your distro/version:
g++ counter.cpp -o counter $(pkg-config --cflags --libs fox17)
```

---

## 5. The GUI update mechanism

During idle time, FOX walks the widget tree and sends each widget's target a **`SEL_UPDATE`** message. This is the second half of the target/message system, and it is what makes FOX applications stay consistent without manual refresh code.

The update handler asks a question ("what should my state be?") and answers by **sending a message back to the sender**:

```cpp
long MyWindow::onUpdSomething(FXObject* sender, FXSelector, void*) {
  sender->handle(this,
                 FXSEL(SEL_COMMAND, condition ? FXWindow::ID_ENABLE : FXWindow::ID_DISABLE),
                 NULL);
  return 1;
}
```

Why send a message rather than call a method? The handler doesn't know what kind of widget the sender is. A menu item, toolbar button, and check button can all share one update handler, and each reacts in its own way to `ID_CHECK` / `ID_UNCHECK` / `ID_ENABLE` / `ID_DISABLE`. A widget that doesn't understand the reply simply ignores it. The only guarantee is that the sender is an `FXObject`.

**Why it scales:** with N commands and M widgets that reflect each command's state, a callback-per-pair design needs N×M update routines. FOX needs about N+M. Data changed by timers, other threads, or network input still shows up in the GUI without extra code.

Sending your own messages works the same way: any `FXObject` can be sent a message with `obj->handle(sender, FXSEL(type, id), ptr)`. `FXSEL` packs two 16-bit values into one 32-bit selector.

---

## 6. Data targets: binding variables to widgets

For forms and settings dialogs, writing a handler per field gets tedious. **`FXDataTarget`** is a ready-made target that connects one application variable to any number of widgets, in both directions:

- The user edits a widget → the variable changes.
- The variable changes → every connected widget refreshes (via `SEL_UPDATE`).

It works with `FXint`, real numbers, and `FXString` variables, among others. A target is tied to one variable, but several widgets can share it (for example, a text field and a slider for the same number).

### Value binding (`ID_VALUE`)

For widgets that hold a value, such as text fields, spinners, and sliders:

```cpp
struct Settings {
  FXString nickname;
  FXint    volume;
  FXint    theme;     // 0, 1, 2
};

class SettingsDialog : public FXDialogBox {
  FXDECLARE(SettingsDialog)
public:
  Settings     data;
private:
  FXDataTarget nickTarget;
  FXDataTarget volTarget;
  FXDataTarget themeTarget;
protected:
  SettingsDialog() {}
public:
  SettingsDialog(FXWindow* owner);
};

FXIMPLEMENT(SettingsDialog, FXDialogBox, NULL, 0)

SettingsDialog::SettingsDialog(FXWindow* owner)
  : FXDialogBox(owner, "Settings"),
    nickTarget(data.nickname),
    volTarget(data.volume),
    themeTarget(data.theme) {

  new FXTextField(this, 20, &nickTarget, FXDataTarget::ID_VALUE);
  new FXSlider   (this, &volTarget, FXDataTarget::ID_VALUE /* , options... */);

  // Option binding (see below): the value is encoded in the message id.
  new FXRadioButton(this, "Light", &themeTarget, FXDataTarget::ID_OPTION + 0);
  new FXRadioButton(this, "Dark",  &themeTarget, FXDataTarget::ID_OPTION + 1);
  new FXRadioButton(this, "Auto",  &themeTarget, FXDataTarget::ID_OPTION + 2);

  new FXButton(this, "Cancel", NULL, this, FXDialogBox::ID_CANCEL);
  new FXButton(this, "OK",     NULL, this, FXDialogBox::ID_ACCEPT);
}
```

### Option binding (`ID_OPTION + i`)

For radio buttons, the value is **added to the message id**. Sending `ID_OPTION + 2` sets the variable to `2`. During update, the target replies `ID_CHECK` to the widget whose `i` matches the current value and `ID_UNCHECK` to the others, so the right radio button is selected automatically.

### The edit-a-copy pattern

The docs' recommended dialog usage is:

```cpp
SettingsDialog dlg(mainWindow);
dlg.data = currentSettings;        // work on a copy
if (dlg.execute()) {               // true if the user accepted
  currentSettings = dlg.data;      // commit
}                                  // Cancel: copy is simply discarded
```

The dialog shows the old values initially (because of the update mechanism), and nothing needs undoing on Cancel.

### How it works internally

`SEL_UPDATE` arrives at the data target. It reads the variable and answers the sender with `ID_SETINTVALUE`, `ID_SETREALVALUE`, or `ID_SETSTRINGVALUE`. It never needs to know what kind of widget it is talking to, which is the same trick as in section 5.

---

## 7. Messages that don't come from widgets

Besides widget messages, `FXApp` can deliver four kinds of non-GUI messages to any target. Registration always names a **target object and a message id**, the same pair you'd put in a widget constructor.

| Kind | Message type | Register with | Fires | Auto-removed after firing? |
|---|---|---|---|---|
| Timer | `SEL_TIMEOUT` | `addTimeout()` / `addDeadline()` | once, after an interval | yes |
| Chore | `SEL_CHORE` | `addChore()` | once, when the app is about to block waiting for events | yes |
| Signal | `SEL_SIGNAL` | `addSignal()` | when a POSIX signal arrives | no |
| Input | `SEL_IO_READ` / `SEL_IO_WRITE` / `SEL_IO_EXCEPT` | `addInput()` | when an fd/handle is ready | no |

### Timers

Intervals in the current docs are given in **nanoseconds**. (I believe older FOX 1.6 releases used milliseconds, so check your version's headers.)

Timers fire **once**, so animations re-arm themselves:

```cpp
// in the message map
FXMAPFUNC(SEL_TIMEOUT, Ticker::ID_TICK, Ticker::onTick),

// start (1 second)
getApp()->addTimeout(this, ID_TICK, 1000000000);

long Ticker::onTick(FXObject*, FXSelector, void*) {
  // ... do the periodic work ...
  if (running) getApp()->addTimeout(this, ID_TICK, 1000000000);   // re-arm
  return 1;
}

Ticker::~Ticker() {
  getApp()->removeTimeout(this, ID_TICK);   // safe even if it already fired
}
```

Useful properties:

- A timer is identified by **(target, id)**, so adding the same pair again resets it. No bookkeeping is needed, and removing a timer that already fired is harmless, which keeps destructors simple.
- An optional `void*` user-data argument is passed through to the handler's `ptr`.
- `hasTimeout()` and `remainingTimeout()` let you inspect a timer. `remainingTimeout()` returns `0` if due but not yet handled, and a special "forever" value if none is set.
- Timers fire when the app returns to the event loop, so they can be a little late. Re-arming each time with a fresh interval lets that **jitter accumulate**. For long-running periodic work (a clock, say), use **`addDeadline()`** with an absolute time (nanoseconds since the Unix epoch) and add one period to the previous deadline each time. Errors then stay bounded. The caveat is that your handler must finish faster than the period, or the app will do nothing but handle timeouts.

### Chores (idle work)

Chores run when the event queue is empty. Use them for low-priority background work:

```cpp
getApp()->addChore(this, ID_IDLETASK);
getApp()->removeChore(this, ID_IDLETASK);   // safe even if already fired
```

Warnings from the docs:

- Re-registering a chore from inside its own handler means the app never gets to sleep, so it uses 100% CPU. Avoid it.
- GUI updates and chores are interleaved, so a slow chore slows GUI updating down too.
- You don't need chores to defer painting. All painting in FOX is already delayed.

### Signals

```cpp
getApp()->addSignal(SIGINT, this, ID_INTERRUPT /*, immediate, flags */);
getApp()->removeSignal(SIGINT);
```

Two delivery modes:

- **Synchronous (default):** held until the app is back in the event loop, so your data structures are in a consistent state. Use this for things like `SIGINT`.
- **Immediate (asynchronous):** delivered right away, possibly mid-operation. Use this only for last-ditch recovery, such as saving the user's data after `SIGSEGV`, and take extreme care in the handler.

### Inputs (sockets, pipes, and so on)

For network or pipe I/O, register the descriptor and let FOX wake you only when there is something to do, instead of polling in a timer:

```cpp
getApp()->addInput(this, ID_SOCKET, fd, INPUT_READ | INPUT_WRITE | INPUT_EXCEPT, userdata);
getApp()->removeInput(fd, INPUT_WRITE);
```

Input registrations stay active until you remove them. A typical server registers a listening socket, and in the `SEL_IO_READ` handler it calls `accept()` and registers the new connection. On UNIX this uses `select()` and on Windows `MsgWaitForMultipleObjects()`. The docs suggest dropping `INPUT_WRITE` when you have nothing to send, because a writable descriptor stays "ready" as long as its buffer has room, and re-adding it only when output is blocked.

---

## 8. Layout basics

Widgets are placed by **layout managers**, which are themselves widgets (containers). The ones you'll meet immediately:

- **`FXHorizontalFrame`** places children left to right.
- **`FXVerticalFrame`** places children top to bottom.
- **`FXMatrix`** places children in rows and columns.
- **`FXMainWindow`** is itself a layout manager.

The last two constructor arguments of a widget typically control *how it is placed in its parent* through **layout hints** passed in the options bitmask:

- `LAYOUT_FILL_X` / `LAYOUT_FILL_Y`: stretch to take the available space.
- `LAYOUT_TOP`, `LAYOUT_LEFT`, and similar: where to sit when not stretched.
- `LAYOUT_SIDE_TOP` and the other `LAYOUT_SIDE_*` hints: which edge to dock to in a packing layout.
- `FRAME_SUNKEN`, `FRAME_RAISED`, `FRAME_THICK`: border style, not placement, but passed in the same options word.

Nest frames to build complex layouts: a horizontal frame that holds two vertical frames is a two-column UI. Stretch (`LAYOUT_FILL_*`) on the widgets that should absorb extra space, and leave the rest at their natural size.

> I did not read the Layout Managers page itself, so this section is a summary of what appears in the examples I did read plus general FOX usage. Treat it as a starting point and read that page for padding, spacing, matrix options, and exact hint semantics.

---

## 9. Common messages cheat sheet

| Type | Meaning |
|---|---|
| `SEL_COMMAND` | A control was activated (button clicked, menu item chosen). Also used for replies like `ID_ENABLE` / `ID_CHECK`. |
| `SEL_UPDATE` | Idle-time "what should my state be?" query. |
| `SEL_PAINT` | A canvas needs repainting. `ptr` is an `FXEvent*` with the dirty rectangle. |
| `SEL_LEFTBUTTONPRESS` / `SEL_LEFTBUTTONRELEASE` | Mouse button events. |
| `SEL_MOTION` | Mouse moved. |
| `SEL_CHANGED` | A value is changing (for example, during a drag or while typing). |
| `SEL_CLOSE` | A top-level window is being closed. If the target handles this, the window is not closed. |
| `SEL_TIMEOUT` / `SEL_CHORE` / `SEL_SIGNAL` | See section 7. |
| `SEL_IO_READ` / `SEL_IO_WRITE` / `SEL_IO_EXCEPT` | See section 7. |

For the full list of which messages each widget sends, see the community-maintained "FOX Messages Sent" reference.

---

## 10. Advanced: delegation

If a target has no handler for a message, FOX calls its **`onDefault()`**. Overriding it lets you forward unknown messages to another object:

```cpp
long MyWidget::onDefault(FXObject* sender, FXSelector sel, void* ptr) {
  return delegate && delegate->handle(sender, sel, ptr);
}
```

To forward a message that your *base class* would otherwise handle, map it to `onDefault` explicitly:

```cpp
FXMAPFUNC(SEL_COMMAND, BaseWidget::ID_DOSOMETHING, MyWidget::onDefault),
```

If you delegate, start the delegate's own IDs where yours end, so numbering doesn't overlap unintentionally.

---

## 11. Pitfalls and rules of thumb

1. **Targets must outlive their sources.** A widget sending to a destroyed object is a bug. Usually the target is a dialog, the app, or another long-lived object. If a target can disappear first, call `setTarget(NULL)` on the sources. FOX also overwrites objects in their destructors, so such bugs tend to crash quickly instead of lurking.
2. **Always `FXDECLARE` + `FXIMPLEMENT`** on every `FXObject`-derived class, and provide the protected default constructor.
3. **Start enum IDs at the base class's `ID_LAST`**, and add an `ID_LAST` of your own if anyone might subclass you.
4. **Return `1` from handlers that handled the message** so the GUI refreshes. Return `0` if you didn't handle it.
5. **Don't mutate widgets from command handlers when an update handler or data target can do it.** Change the data and let the update phase reflect it.
6. **Remove timers and chores in destructors.** It's safe even if they already fired.
7. **Don't build a chore that re-registers itself** unless you want a busy loop.
8. **Don't forget `application.create()`** before `run()`, or no windows will exist.
9. **Hold off using immediate signals** for anything other than emergency cleanup.

---

## 12. If you are wrapping FOX from another language

These points follow directly from the design above and matter most for bindings:

- **The core mechanism is C++-specific.** The message map is a static table generated by macros (`FXDECLARE`, `FXDEFMAP`, `FXIMPLEMENT`), and handlers are member functions with the fixed signature `long (FXObject*, FXSelector, void*)`. A foreign-language layer must either generate C++ subclasses or route through a generic receiver object that implements `onDefault` / its own map.
- **Selectors are just packed integers** (`FXSEL(type, id)`, two 16-bit halves), and handlers return a `long`. That maps cleanly onto a C ABI.
- **Dynamic retargeting** (`setTarget` / `setSelector`) lets you point widgets at a single receiver object per window or per binding.
- **Lifetime rule:** the target must outlive its senders, so the owner of the target should also own, or explicitly detach, the widgets that point at it.
- **Two-phase creation** (construct the whole tree, then `create()`) should be exposed explicitly instead of being hidden.
- **Message IDs are per-class,** so a wrapper has to keep numbering consistent with the base class's `ID_LAST` when it defines new IDs.

---

## 13. Where to go next

The documentation index lists these sections, which this tutorial **did not cover**. Read them for the details:

- **Updating Control State** (`guiupdate.html`): the full treatment of the GUI Update phase.
- **The FXApp Class** and **The FXWindow Class**: the application and base-widget APIs.
- **Layout Managers** and **Keyboard Focus**.
- **Icons and Images**, **Fonts**, **Help and Tool Tips**.
- **The FOX Registry** (persistent settings), **File Manipulations**, **Regular Expressions** (`FXRex`).
- **Drag and Drop**, **Clipboard and Primary Selection**.
- **The OpenGL Viewer**, **Serialization of Objects**.
- **Writing Your Own Widgets**.
- **Windows Notes**, **FAQ**, and the **class indexes** for FOX 1.0 and 1.3.

Docs home: <http://fox-toolkit.org/doc.html>. The distribution's `tests` directory also contains example programs.
