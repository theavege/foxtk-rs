#!/usr/bin/env python3
"""
check_consistency.py -- consolidated correctness checks for foxtk.h / foxtk.cpp.

Consolidates the ad hoc checks used throughout this project's development
into one repeatable script:

  1. compile        - actually compiles foxtk.cpp (ground truth; every other
                       check here is a heuristic, this is the real answer)
  2. new-signatures  - every FXWhatever_new()'s header declaration matches
                       its .cpp definition exactly (catches the FXList_new/
                       FXListBox_new/FXSpinner_new class of bug)
  3. macro-impls     - every function a header macro (EXT_TEXT, EXT_HELP,
                       EXT_RANGE, etc.) expands to declaring is actually
                       defined in the .cpp
  4. duplicates      - no function is defined twice in foxtk.cpp (catches
                       the FX7Segment_new / FXScrollWindow_new class of bug,
                       where something already existed and got re-added)
  5. opaque-types    - every FOXTK_OPAQUE()'d / typedef'd type in the header
                       resolves to a real, visible FOX class in the actual
                       compiled translation unit -- not a disconnected fake
                       (catches the FXGradientBar missing-#include bug)
  6. braces          - both files have balanced braces
  7. widget-coverage - which real FOX classes have zero presence anywhere
                       in foxtk.h (report only, not a pass/fail check --
                       some gaps are deliberate: image codecs, math types,
                       internal plumbing that don't belong in this API)

Usage:
    python3 check_consistency.py [--header foxtk.h] [--impl foxtk.cpp]
                                  [--fox-include DIR] [--compiler g++]

Requires `fox-config` on PATH (or --fox-include) to know where the real
FOX headers live for checks 1 and 5. Exits nonzero if any of checks 1-6
fail; check 7 is informational only.
"""
import argparse
import re
import subprocess
import sys
from collections import Counter


def read(path):
    with open(path, encoding="utf-8", errors="replace") as f:
        return f.read()


def strip_macro_defs(text):
    """Remove #define lines (and their line-continuations) so regexes that
    look for *usages* of a macro (EXT_TEXT(FXLabel)) don't also match the
    macro's own *definition* (#define EXT_TEXT(widget) ...), where the
    placeholder parameter name would otherwise look like a real widget."""
    return re.sub(r'^#define.*(?:\\\n.*)*$', '', text, flags=re.M)


# ---------------------------------------------------------------------------
# 1. Compile check -- ground truth
# ---------------------------------------------------------------------------
def check_compile(impl_path, fox_include, compiler):
    print(f"[1/7] compile ({compiler}) ... ", end="", flush=True)
    cmd = [compiler, "-c", "-std=c++17", "-Wall", "-Wextra",
           f"-I{fox_include}", impl_path, "-o", "/tmp/_consistency_check.o"]
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print("FAIL")
        print(result.stderr)
        return False
    warning_count = result.stderr.count("warning:")
    print(f"ok ({warning_count} warnings)")
    if warning_count:
        print(result.stderr)
    return True


# ---------------------------------------------------------------------------
# 2. `_new` signature consistency between header and impl
# ---------------------------------------------------------------------------
def extract_new_sigs(text):
    pattern = re.compile(r'([A-Za-z_][\w\*\s]*?)\b(\w+_new)\s*\(([^)]*)\)\s*[;{]')
    sigs = {}
    for m in pattern.finditer(text):
        name = m.group(2)
        params = m.group(3)
        parts = [p.strip() for p in params.split(',') if p.strip()]
        types = tuple(re.sub(r'\s*\w+$', '', p).strip() for p in parts)
        sigs.setdefault(name, []).append(types)
    return sigs


def check_new_signatures(header_text, impl_text):
    print("[2/7] _new signature consistency ... ", end="", flush=True)
    h = extract_new_sigs(strip_macro_defs(header_text))
    c = extract_new_sigs(strip_macro_defs(impl_text))
    problems = []
    for name in sorted(set(h) & set(c)):
        for ht in h[name]:
            for ct in c[name]:
                if ht != ct:
                    problems.append(f"  MISMATCH {name}: header={ht} impl={ct}")
    missing = sorted(n for n in h if n not in c)
    for n in missing:
        problems.append(f"  MISSING FROM IMPL: {n}")
    extra = sorted(n for n in c if n not in h)
    for n in extra:
        problems.append(f"  MISSING FROM HEADER (impl-only, unreachable): {n}")
    if problems:
        print("FAIL")
        print("\n".join(problems))
        return False
    print(f"ok ({len(h)} constructors checked)")
    return True


# ---------------------------------------------------------------------------
# 3. Macro-expanded declarations all have implementations
# ---------------------------------------------------------------------------
MACRO_EXPANSIONS = {
    'EXT_TEXT': ['{w}_get_text', '{w}_set_text', '{w}_set_text_color', '{w}_set_font'],
    'EXT_HELP': ['{w}_get_help_text', '{w}_get_tip_text', '{w}_set_help_text', '{w}_set_tip_text'],
    'EXT_CHECK': ['{w}_get_check', '{w}_set_check'],
    'EXT_EDITABLE': ['{w}_is_editable', '{w}_set_editable'],
    'EXT_STATE': ['{w}_get_state', '{w}_set_state'],
    'EXT_JUSTIFY': ['{w}_get_justify', '{w}_set_justify'],
    'EXT_STYLE': ['{w}_get_style', '{w}_set_style'],
    'EXT_RANGE': ['{w}_get_value', '{w}_get_range', '{w}_set_value', '{w}_set_range'],
    'EXT_SELECTABLE': ['{w}_get_item_text', '{w}_get_num_items', '{w}_get_current_item',
                        '{w}_append_item', '{w}_clear_items', '{w}_set_current_item',
                        '{w}_set_num_visible'],
    'EXT_DRAWING': ['{w}_set_foreground', '{w}_set_line_width', '{w}_draw_line',
                     '{w}_draw_point', '{w}_draw_rect', '{w}_fill_rect'],
}


def check_macro_impls(header_text, impl_text):
    print("[3/7] macro-declared functions have implementations ... ", end="", flush=True)
    header_text = strip_macro_defs(header_text)
    expanded = set()
    for macro, templates in MACRO_EXPANSIONS.items():
        for m in re.finditer(macro + r'\(\s*(\w+)', header_text):
            widget = m.group(1)
            for t in templates:
                expanded.add(t.format(w=widget))
    expanded -= KNOWN_UNIMPLEMENTED_MACRO_FUNCS
    missing = sorted(
        n for n in expanded
        if not re.search(r'\b' + re.escape(n) + r'\s*\(', impl_text)
    )
    if missing:
        print("FAIL")
        for n in missing:
            print(f"  MISSING: {n}")
        return False
    print(f"ok ({len(expanded)} macro-expanded functions checked)")
    return True


# ---------------------------------------------------------------------------
# 4. No duplicate function definitions
# ---------------------------------------------------------------------------
def check_duplicates(impl_text):
    print("[4/7] no duplicate function definitions ... ", end="", flush=True)
    names = re.findall(r'^\s*[\w:\*&<>, ]+?\b(FX\w+)\s*\([^;{]*\)\s*\{', impl_text, re.M)
    dupes = {n: cnt for n, cnt in Counter(names).items() if cnt > 1 and n != 'FXDECLARE'}
    if dupes:
        print("FAIL")
        for n, cnt in dupes.items():
            print(f"  DUPLICATE: {n} defined {cnt} times")
        return False
    print("ok")
    return True


# ---------------------------------------------------------------------------
# 5. Every opaque type resolves to a real, visible FOX class
# ---------------------------------------------------------------------------
# Types with no real FOX counterpart -- this wrapper's own synthetic handles.
KNOWN_SYNTHETIC_TYPES = {"FXTimeout"}

# Declared via a header macro but deliberately never implemented, with a
# NOTE comment in the .cpp explaining why (checked against real FOX API,
# not just an oversight). Currently: FXTable has no table-wide justify in
# the real FOX API -- justification is per-cell on FXTableItem, not the
# table itself -- so implementing get/set_justify would silently do the
# wrong thing rather than just fail to link.
KNOWN_UNIMPLEMENTED_MACRO_FUNCS = {"FXTable_get_justify", "FXTable_set_justify"}


def check_opaque_types(header_text, impl_path, fox_include, compiler):
    print("[5/7] opaque types resolve to real FOX classes ... ", end="", flush=True)
    stripped = strip_macro_defs(header_text)
    declared = set(re.findall(r'FOXTK_OPAQUE\((\w+)\)', stripped))
    declared |= set(re.findall(r'typedef struct (\w+) \1;', stripped))
    declared -= KNOWN_SYNTHETIC_TYPES

    cmd = [compiler, "-E", "-std=c++17", f"-I{fox_include}", impl_path]
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print("SKIPPED (preprocessing failed, see compile check)")
        return True
    visible = set(re.findall(r'(?<=^class )\s*(FX\w+)', result.stdout, re.M))

    missing = sorted(declared - visible)
    if missing:
        print("FAIL")
        for n in missing:
            print(f"  NOT VISIBLE: {n} (declared in header but no real class "
                  f"found in the compiled translation unit -- check #include order)")
        return False
    print(f"ok ({len(declared)} opaque types checked, "
          f"{len(KNOWN_SYNTHETIC_TYPES)} synthetic exceptions excluded)")
    return True


# ---------------------------------------------------------------------------
# 6. Brace balance
# ---------------------------------------------------------------------------
def check_braces(header_text, impl_text):
    print("[6/7] brace balance ... ", end="", flush=True)
    h_bal = header_text.count('{') - header_text.count('}')
    c_bal = impl_text.count('{') - impl_text.count('}')
    if h_bal != 0 or c_bal != 0:
        print("FAIL")
        print(f"  header balance: {h_bal}, impl balance: {c_bal}")
        return False
    print("ok")
    return True


# ---------------------------------------------------------------------------
# 7. Widget coverage report (informational, not pass/fail)
# ---------------------------------------------------------------------------
# Classes that are deliberately out of scope for this API -- image codecs,
# math/geometry types, streams, exceptions, and other internal plumbing that
# don't belong in a widget wrapper. Extend this list rather than treating a
# hit here as a real gap.
OUT_OF_SCOPE_PATTERNS = [
    r'^FX(88591|CP|KOI8R|UTF)', r'Codec$', r'^FXMat', r'^FXVec', r'^FXQuat',
    r'^FXRange', r'^FXExtent', r'^FXSpher', r'^FXStream$',
    r'^FX(File|GZFile|BZFile|Memory)Stream',
    r'^FX(Array|Dict|Hash|ObjectList|StringDict|FileDict|IconDict)$',
    r'^FX(Thread|Socket|Pipe|MemMap|IO|Path|Stat|Dir|System|Registry|Settings|Element)$',
    r'^FXException$', r'^FX(Point|Rectangle|Size|Region)$', r'^FXDLL$',
    r'^FXGUISignal$', r'^FXDelegator$', r'^FXDebugTarget$', r'^FXAccelTable$',
    r'^FXComposeContext$', r'^FXRex$', r'^FXURL$', r'^FXDate$',
    r'^FXTranslator$', r'^FXTextCodec$', r'^FX(Undo)?List$',
    r'^FXDataTarget$', r'^FXExpression$',
    r'(GIF|ICO|IFF|JPG|PCX|PNG|PPM|RAS|RGB|TGA|TIF|XBM|XPM|BMP)(Icon|Image|Cursor)$',
    r'^FXGL(Cone|Context|Cube|Cylinder|Object|Shape|Sphere|TriangleMesh|Group|Line|Point)$',
    r'^FXDocument$', r'^FXFile$', r'^FXFont$', r'^FXIconSource$',
    r'^FXRootWindow$', r'^FXVisual$', r'^FXShell$',
    # Added after the first full audit pass:
    r'Exception$',            # FOX's C++ exception hierarchy -- C has no
                               # exceptions, nothing sensible to wrap
    r'^FXString$',            # the wrapper converts to/from this
                               # everywhere internally; exposing it
                               # directly would defeat the point of the
                               # const-char*-based C API
    r'^FXMetaClass$',         # FOX's own RTTI/reflection system, used
                               # internally by FXDECLARE -- not meant for
                               # application-level construction
    r'^FX(Mutex(Lock)?|Condition|Semaphore)$',  # threading primitives --
                               # Rust has its own; wrapping FOX's is a
                               # separate concern from widget wrapping
    r'^FXDC$',                # abstract base, no public constructor --
                               # see the NOTE already in foxtk.h
    r'^FX(Color|ComboTable|Dir|File|Folding|Header|Icon|List|Tree)Item$',
                               # per-row/per-cell helper objects for the
                               # list/tree/table widgets. Deliberately not
                               # exposed as separate opaque handles --
                               # each of those widgets' EXT_SELECTABLE
                               # accessors already covers basic usage by
                               # index/text without needing a raw Item
                               # pointer. A real gap only if per-item
                               # icons/custom user data become necessary.
    r'^FXCommand(Group)?$',   # FOX's undo/redo command base classes,
                               # meant to be subclassed by application
                               # code -- same shape as the exception
                               # classes, not directly instantiable
    r'^FXScrollArea$',        # confirmed: protected constructor in the
                               # real FOX header, only reachable via
                               # concrete subclasses (FXScrollWindow,
                               # FXIconList, both already wrapped)
    r'^FXDockHandler$',       # confirmed: FXDECLARE_ABSTRACT + protected
                               # constructor, internal to the docking
                               # machinery (FXDockSite/FXDockBar/
                               # FXDockTitle), not meant for direct use
    r'^FXPicker$',            # a bare FXButton subclass -- inherits
                               # everything from FXButton (already
                               # wrapped) and adds no new public members
                               # of its own; wrapping it would just
                               # duplicate FXButton's accessor set for
                               # zero new functionality
    r'^FXCURCursor$',         # loads a .cur file's raw pixel data
                               # directly -- same category as the
                               # already-excluded format-specific Icon/
                               # Image loaders. FXCursor (built-in stock
                               # cursors) is wrapped and covers the
                               # common case.
]


def widget_coverage_report(header_text, fox_include_dir):
    print("[7/7] widget coverage (informational) ...")
    import glob
    import os
    all_classes = set()
    for path in glob.glob(os.path.join(fox_include_dir, "FX*.h")):
        text = read(path)
        all_classes.update(re.findall(r'^class\s+FXAPI\s+(\w+)', text, re.M))

    out_of_scope = re.compile("|".join(OUT_OF_SCOPE_PATTERNS))

    touched = set()
    for cls in all_classes:
        if re.search(r'\b' + re.escape(cls) + r'(_\w+\s*\(|\)?\s*;)', header_text):
            touched.add(cls)

    untouched = sorted(
        c for c in (all_classes - touched) if not out_of_scope.search(c)
    )
    print(f"  {len(touched)} classes touched, {len(all_classes)} total, "
          f"{len(untouched)} untouched and in-scope")
    if untouched:
        for c in untouched:
            print(f"    - {c}")
    return untouched


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                      formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--header", default="foxtk.h")
    parser.add_argument("--impl", default="foxtk.cpp")
    parser.add_argument("--fox-include", default=None,
                         help="Path to FOX's include/ dir. Auto-detected via "
                              "`fox-config --cflags` if not given.")
    parser.add_argument("--compiler", default="g++")
    args = parser.parse_args()

    fox_include = args.fox_include
    if fox_include is None:
        try:
            out = subprocess.run(["fox-config", "--cflags"],
                                  capture_output=True, text=True, check=True).stdout
            m = re.search(r'-I(\S+)', out)
            fox_include = m.group(1) if m else None
        except (FileNotFoundError, subprocess.CalledProcessError):
            pass
    if fox_include is None:
        print("error: could not determine FOX include dir "
              "(pass --fox-include or install fox-config)")
        sys.exit(2)

    header_text = read(args.header)
    impl_text = read(args.impl)

    results = [
        check_compile(args.impl, fox_include, args.compiler),
        check_new_signatures(header_text, impl_text),
        check_macro_impls(header_text, impl_text),
        check_duplicates(impl_text),
        check_opaque_types(header_text, args.impl, fox_include, args.compiler),
        check_braces(header_text, impl_text),
    ]
    widget_coverage_report(header_text, fox_include)

    print()
    if all(results):
        print("ALL CHECKS PASSED")
        sys.exit(0)
    else:
        print(f"{results.count(False)} CHECK(S) FAILED")
        sys.exit(1)


if __name__ == "__main__":
    main()