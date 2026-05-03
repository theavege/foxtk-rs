# Placing Widgets Automatically

Making an attractive layout for a Dialog or Window  is an important consideration in design of a user interface.  Setting windows at specific x,y coordinates, and specifying explicit dimensions allow the GUI designer full control over  the placement of each Control.  However, this is very tedious and time-consuming.  Also, what if the labels on buttons change, or if the user wants to use a bigger font?

For these reasons, the preferred method for placing GUI Controls on windows in FOX is through the use of so-called Layout Managers.  A Layout manager is a widget whose primary purpose is to arrange GUI Controls contained inside of it in a certain way. This even includes other Layout Managers! In fact, Layout Managers may be nested arbitrarily!

Different layout managers arrange their children in different arrangements, for example, from left-to-right, top-to-bottom, in a grid, or even all on top of one another. Most layout managers also allow for explicit placement of their children, using hard-coded coordinates.

The benefits of this approach vis-a-vis a precise and explicit placement is that:

1. It takes the tedium out of placing GUI Controls; the application programmer does not concern him or herself with specific coordinates.
2. GUI Controls are automatically arranged correctly, even if button labels are changed, or users choose bigger fonts.
3. Layouts may be recalculated intelligently when a user resizes the window.
4. It makes it easy to accomodate and place Controls which are created automatically under program control, for example in GUI's created from database tables.

In FOX, you determine the arrangement of a GUI Control by selecting the appropriate Layout Managers, and a combination of Packing Styles passed to the Layout Manager, as well as a combination of Layout Hints passed to the GUI Control being arranged. Thus, virtually every conceivable arrangement can be achieved simply by nesting the appropriate layout managers in a certain way.

# Basic Layout Patterns

FOX supports a number of general-purpose layout managers. The desired arrangement of GUI controls determines which layout manager is the most appropriate for the job; the following table lists the most commonly used layout managers and their layout arrangement:

## [FXWindow](http://fox-toolkit.org/ref/classFX_1_1FXWindow.html#details)

The FXWindow class manages a window on the screen. FXWindow is the base class of all FOX GUI widgets such as buttons and sliders, in other words, all widgets are ultimately derived from FXWindow. All windows are organized into a so called widget tree; at the root of this widget tree is the root window which is a special window which represents the entire screen. Top level or shell windows are children of the root window; they're special because top level windows, such as the main window and dialog box, are positioned and resized directly by the user. Layout manager windows are composite windows which may have zero or more children, possibly including other layout managers. Layout managers position their child-windows according to certain layout patterns and layout flags. Child windows or simple windows are windows which do not themselves have any children. Child windows are usually simple controls such as buttons and sliders.

## [FXPacker](http://fox-toolkit.org/ref/classFX_1_1FXPacker.html#details)

The Packer layout widget places its GUI elements in its interior rectangle, placing each child against one of the four sides. As each child is placed, the size of the rectangle is reduced by the space taken up by the child.
If a child is placed against the left or right, the interior rectangle's width is reduced; if the child is placed against the top or bottom, the height is reduced.
Children may be of any type, including other layout managers.

## [FXTopWindow](http://fox-toolkit.org/ref/classFX_1_1FXTopWindow.html#details)

The TopWindow operates like an FXPacker window. For simple dialogs and toplevel windows, no additional layout managers may be needed in many cases, as the TopWindow's layout characteristics may be sufficient.

## [FXGroupBox](http://fox-toolkit.org/ref/classFX_1_1FXGroupBox.html#details)

The GroupBox is a layout manager that provides identical layout facilities as the Packer.  In addition, the GroupBox draws an attractive border around its contents, and provides an optional caption.

## [FXHorizontalFrame](http://fox-toolkit.org/ref/classFX_1_1FXHorizontalFrame.html#details)

The HorizontalFrame layout manager packs its children horizontally from left to right (or right to left).

## [FXVerticalFrame](http://fox-toolkit.org/ref/classFX_1_1FXVerticalFrame.html#details)

The VerticalFrame layout manager packs its children vertically, from top to bottom or vice versa.  It behaves similar to the HorizontalFrame layout manager.

## [FXSwitcher](http://fox-toolkit.org/ref/classFX_1_1FXSwitcher.html#details)

The Switcher layout manager places its children exactly on top of each other; it ignores most of the layout hints provided by each child.  You typically use a layout manager like the switcher to save screen real-estate, by placing for example several control panels on top of each other, and bringing the right one on top depending on the context.

## [FXSplitter](http://fox-toolkit.org/ref/classFX_1_1FXSplitter.html#details)

The Splitter layout manager divides some area of the screen horizontally or vertically.  The divider bars can be repositioned by the user, so that depending on what the user is doing, he or she may give one or the other partition more screen space.

## [FX4Splitter](http://fox-toolkit.org/ref/classFX_1_1FX4Splitter.html#details)

The Four-way splitter divides its contents into four subframes, like a four-paned window. The user can interactively adjust the dividers to change the division. Unlike the simple splitter, the subdivision of the four-way splitter is fractional, i.e. the subframes are resized proportionally if the entire four-way splitter is resized.

## [FXSpring](http://fox-toolkit.org/ref/classFX_1_1FXSpring.html#details)

The spring is typically used inside a FXHorizontalFrame or FXVerticalFrame. As its name implies, it stretches and compresses like a spring of a certain length. Different springs of different lengths are typically placed side-by-side in a FXHorizontalFrame, allowing for a fixed-proportion arrangement, e.g. a 60:40 split.
