#!/usr/bin/env python3
"""
Script to improve foxtk.cpp by adding:
1. Error handling macros
2. Validation to make_widget template
3. Section organization
"""

import re

def improve_foxtk_cpp(content):
    """Apply improvements to foxtk.cpp content"""
    
    # Add includes and macros at the top
    new_header = '''#include <fx.h>
#include <cstdio>

// ============================================================================
// ERROR HANDLING MACROS
// ============================================================================

/// Validates that a pointer is not null, logs error and returns nullptr if null
#define VALIDATE_POINTER(ptr, name) \\
  if (!ptr) { \\
    fprintf(stderr, "%s: %s is null at %s:%d\\n", __func__, name, __FILE__, __LINE__); \\
    return nullptr; \\
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

'''
    
    # Find the position after #include <fx.h>
    include_pos = content.find('#include <fx.h>')
    if include_pos == -1:
        return content
    
    # Find the end of the line
    line_end = content.find('\n', include_pos)
    if line_end == -1:
        return content
    
    # Insert the new header after the include
    new_content = content[:line_end + 1] + new_header + content[line_end + 1:]
    
    # Add section comments - simpler approach, just add before each section
    new_content = new_content.replace(
        '//~ FXObject.h\n', 
        '// ============================================================================\n// BASE WIDGETS\n// ============================================================================\n//~ FXObject.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXApp.h\n',
        '// ============================================================================\n// APPLICATION\n// ============================================================================\n//~ FXApp.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXWindow.h\n',
        '// ============================================================================\n// WINDOW MANAGEMENT\n// ============================================================================\n//~ FXWindow.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXDrawable.h\n',
        '// ============================================================================\n// DRAWING\n// ============================================================================\n//~ FXDrawable.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXCanvas.h\n',
        '// ============================================================================\n// DRAWING WIDGETS\n// ============================================================================\n//~ FXCanvas.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXGLCanvas.h\n',
        '// ============================================================================\n// OPENGL WIDGETS\n// ============================================================================\n//~ FXGLCanvas.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXPacker.h\n',
        '// ============================================================================\n// LAYOUT WIDGETS\n// ============================================================================\n//~ FXPacker.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXButton.h\n',
        '// ============================================================================\n// INPUT WIDGETS - BUTTONS\n// ============================================================================\n//~ FXButton.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXTopWindow.h\n',
        '// ============================================================================\n// TOP-LEVEL WINDOWS\n// ============================================================================\n//~ FXTopWindow.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXMenuBar.h\n',
        '// ============================================================================\n// MENU WIDGETS\n// ============================================================================\n//~ FXMenuBar.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXStatusBar.h\n',
        '// ============================================================================\n// STATUS WIDGETS\n// ============================================================================\n//~ FXStatusBar.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXDockBar.h\n',
        '// ============================================================================\n// DOCKING WIDGETS\n// ============================================================================\n//~ FXDockBar.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXToolBar.h\n',
        '// ============================================================================\n// TOOLBAR WIDGETS\n// ============================================================================\n//~ FXToolBar.h\n'
    )
    
    new_content = new_content.replace(
        '//~ FXMDIButton.h\n',
        '// ============================================================================\n// MDI WIDGETS\n// ============================================================================\n//~ FXMDIButton.h\n'
    )
    
    return new_content

if __name__ == '__main__':
    with open('foxtk-sys/src/foxtk.cpp', 'r') as f:
        content = f.read()
    
    improved = improve_foxtk_cpp(content)
    
    with open('foxtk-sys/src/foxtk.cpp', 'w') as f:
        f.write(improved)
    
    print("foxtk.cpp has been improved!")
    print(f"Original size: {len(content)} bytes")
    print(f"Improved size: {len(improved)} bytes")
