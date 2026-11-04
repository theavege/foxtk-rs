# FoxTK-rs Project Specification

## 1. Purpose and scope

FoxTK-rs provides Rust bindings to the FOX 1.6 GUI toolkit. It lets Rust applications construct FOX widgets, configure them through Rust methods and extension traits, and run the native FOX application loop. The project contains two crates in one Cargo workspace:

- `foxtk-sys`: generated low-level bindings and the native C++ compatibility bridge.
- `foxtk`: the higher-level Rust API, widget wrappers, helper traits, and component abstraction.

This document describes the architecture implemented by the repository. Recommendations and planned directions are called out explicitly; they are not promises that the behavior already exists.

## 2. Architectural overview

```text
Rust application / examples
          |
          v
foxtk: App, widget newtypes, enums, prelude extension traits,
       callback helpers, Component event/model/view abstraction
          |
          v
foxtk-sys: bindgen-generated declarations in OUT_DIR/bindings.rs
          |
          v
foxtk-sys/src/foxtk.h + foxtk.cpp (C ABI bridge)
          |
          v
FOX 1.6.59 C++ toolkit + platform libraries
```

The Rust API is organized around FOX's native object graph: an application owns top-level windows and their resources; composite windows parent child widgets; FOX dispatches events through its event loop. The wrapper layer generally stores native pointers and delegates operations to the bridge.

## 3. Workspace and crate responsibilities

### 3.1 `foxtk-sys`

This crate is the unsafe, platform-aware boundary. Its build script:

1. Selects a build path from the target OS and `nobuild` feature.
2. On the default supported build path, obtains the pinned FOX 1.6.59 source archive and verifies it against the expected SHA-256 before extraction.
3. Builds the bridge and FOX sources with CMake/Ninja (Linux, macOS, and Windows default path).
4. Uses `bindgen` over `src/foxtk.h` to write Rust declarations into Cargo's `OUT_DIR`.
5. Emits native link directives required by the target.

On Linux, `nobuild` compiles the bridge against the installed FOX development package discovered through `pkg-config`; FOX itself is linked as the installed library. `nobuild` is rejected on Windows and macOS. Unsupported target operating systems fail from the build script with a clear error.

The bridge is deliberately a C ABI around C++ FOX. It centralizes constructors, accessors, FOX-specific overloads, callback adapters, and operations that bindgen cannot expose conveniently. Rust code should not call FOX C++ APIs directly.

### 3.2 `foxtk`

This crate presents the user-facing Rust API and depends on the exactly matching `foxtk-sys` version. Its main source areas are:

- `app.rs`: `App`, FOX application operations, and a borrowed `Timeout` handle.
- `widgets/`: concrete widget wrappers grouped by purpose (windows, buttons, text, menus, dialogs, lists, images, values, tabs, docking, MDI, and OpenGL).
- `prelude.rs`: extension traits for common FOX capabilities, including object/window/frame/composite, text, selector, ranger, and update behavior; it also defines the `Component` abstraction and event sender/outcome types.
- `enums.rs`: Rust representations for FOX option/style/value constants.
- `macros.rs`: internal macros for wrapper declarations and repeated trait implementations.
- `support.rs`: string and argv ownership helpers and panic-containing FFI callback adapters.

The public API re-exports `App` and widget types at crate root and makes common traits available through `foxtk::prelude`.

## 4. Object model and ownership

FOX's parent/child ownership remains authoritative. Widget wrapper values are handles around native pointers, not independent Rust-owned widget allocations. In the general widget API, Rust lifetimes do not encode the native parent relationship. Applications must therefore keep `App` and parent windows alive while using children. A stale wrapper may otherwise refer to an object already destroyed by FOX.

`ObjectExt::from_raw` is unsafe because the caller must guarantee that the pointer has the expected native type and remains live for each use. `as_raw` exposes the underlying pointer for bridge calls and advanced integration.

Some APIs add local lifetime protection where practical: `Timeout<'app>` borrows its `App` and unregisters its FOX timeout on drop; tree item handles borrow their tree list so mutable clearing cannot overlap the borrow. These are specific guarantees and should not be assumed for every wrapper.

Creation generally takes a parent reference and immediately constructs a lightweight wrapper around the bridge's returned pointer. Wrapper `Drop` behavior is not a substitute for FOX parent ownership. Any future ownership redesign must avoid double-freeing native objects or changing FOX ownership semantics.

## 5. Event and callback model

FOX owns the GUI event loop. `App::run` enters it, and native event callbacks are adapted to Rust closures through C callback functions and opaque context pointers. Callback adapters catch Rust panics before crossing the C ABI and return a fallback event result. This prevents unwinding through C++, but it also means a panic is contained rather than propagated through the FOX loop.

`App::add_timeout` registers a repeating FOX timeout and returns a handle; dropping the handle removes the timeout and its callback context. The timeout callback's boolean is the FOX event result and does not determine whether it repeats. FOX 1.6 timeout units are milliseconds.

### 5.1 Components

`Component` is a model/update/view style abstraction:

- `State` is the model and starts with `Default`.
- `view` constructs widgets and receives a typed `Sender<Event>`.
- `handle` processes an event and may mutate the model, returning `Unchanged`, `Changed`, or `Stop`.
- `update` refreshes the view from the model when at least one drained event reports `Changed`.

`mount` owns the receiver, component instance, model, dispatch closure, FOX chore, periodic timeout, and scheduler state. Its returned `ComponentMount` must remain alive while mounted; dropping it deactivates dispatch and removes the chore and timeout. Nested component mounts must likewise be retained.

Events sent on the UI thread schedule a FOX chore. A worker thread only sends through the channel; it does not call FOX. A repeating 50 ms timeout polls for worker-thread messages because FOX scheduling functions are restricted to the UI thread in this implementation. Therefore worker messages can have up to approximately one poll interval of additional dispatch latency. `Stop` deactivates the sender/scheduler; the mount handle remains responsible for removing registered FOX sources.

## 6. Build and platform contract

The supported build targets declared by `foxtk-sys` are Linux, macOS, and Windows. The README currently describes Linux and Windows as supported and macOS as experimental pending CI validation. The default path builds a pinned FOX source release locally. Build prerequisites and platform caveats are documented in the root README and `.github/workflows` scripts.

Linux-only `nobuild` mode requires installed FOX 1.6 headers, library, and `pkg-config` metadata. The default build requires a C++ toolchain, CMake, Ninja, network access to retrieve the FOX archive on a clean build, and libclang for bindgen. GUI runtime tests/examples require a display server.

`foxtk` and `foxtk-sys` are versioned together; the high-level crate uses an exact dependency version for the sys crate. Cargo workspace profile settings optimize release builds for size, enable LTO, strip symbols, use one codegen unit, and abort on panic.

## 7. API and compatibility policy

The project is currently pre-1.0. The root README states that minor releases may make breaking API changes until 1.0. Both crates should continue to ship at the same version. Native toolkit compatibility is currently pinned to FOX 1.6.59; timeout units and native ABI assumptions must be treated as FOX 1.6 behavior unless the project deliberately changes that baseline.

The supported API consists of the Rust wrappers and traits actually exported by `foxtk`, plus the low-level generated API in `foxtk-sys` for users who depend on it directly. Generated bindings are build artifacts, not checked-in source-of-truth declarations; `foxtk.h` is the bindgen input boundary.

## 8. Architectural invariants

Changes to this project should preserve these invariants unless an explicit migration changes them:

1. C++ exceptions and Rust panics must not unwind across the C ABI boundary.
2. Callback context allocations must be reclaimed exactly once, including registration failure and handle-drop paths.
3. FOX widget destruction follows FOX parent/application ownership; wrapper handles must not independently delete parent-owned objects.
4. Worker threads do not invoke FOX GUI APIs. UI operations and component dispatch that touches widgets run on the UI thread.
5. Every native API exposed by the high-level crate must have a corresponding declared bridge operation with compatible pointer, string, integer, and ownership semantics.
6. The downloaded FOX archive must be checksum-verified before extraction/build.
7. `nobuild` is Linux-only until another platform path is intentionally implemented and validated.

## 9. Current design limits and risks

- Most widget wrappers do not encode parent/application lifetimes, so correct destruction order is partly a caller obligation.
- `from_raw` and low-level FFI calls can violate pointer validity and aliasing assumptions if misused.
- The component worker-message poll introduces bounded latency and continues to be managed by the mount handle.
- Build-from-source requires a native toolchain and external FOX source retrieval on a clean build; offline behavior is not guaranteed unless the source is already staged in Cargo's build output.
- Platform support statements are asymmetric: the build script has a macOS path, while project documentation treats macOS as experimental.
- The public wrapper surface is broad and partly generated by macros, so API reviews must inspect macro expansions and bridge declarations together.

## 10. Recommended architectural direction

These are design goals, not claims about current implementation:

1. **Make native ownership visible.** Prefer borrowed child handles or lifetime-bound constructors where the Rust API can represent FOX's parent relationship without making ordinary GUI construction impractical. Document and test unavoidable exceptions.
2. **Keep the bridge narrow and auditable.** Treat `foxtk.h` plus `foxtk.cpp` as the explicit native API. Add a Rust wrapper only when constructor, return-value, string, callback, and ownership behavior are understood.
3. **Separate build policy from binding generation.** Keep FOX source acquisition, checksum verification, native compilation, bindgen configuration, and link metadata independently understandable and testable.
4. **Clarify UI-thread access.** State which public operations require the UI thread and keep cross-thread communication message-based. A future wake mechanism should schedule work on the UI thread without making worker threads call FOX.
5. **Promote validated platform claims.** Treat CI coverage, toolchain setup, native dependency availability, and GUI runtime constraints as the evidence for a platform being supported.
6. **Keep compatibility explicit.** Pin and document the FOX baseline and Rust MSRV; update generated bindings and bridge assumptions together when either changes.

## 11. Repository map

```text
Cargo.toml                 workspace and release profile
README.md                  overview, platform requirements, examples, ownership notes
foxtk-sys/Cargo.toml        low-level crate and native build dependencies
foxtk-sys/build.rs         build selection, native compilation, bindgen invocation
foxtk-sys/build_support.rs archive validation/extraction helpers
foxtk-sys/src/foxtk.h      C ABI declarations consumed by bindgen
foxtk-sys/src/foxtk.cpp    C++ adapters that invoke FOX
foxtk-sys/src/CMakeLists.txt native bridge and FOX build definition
foxtk/src/lib.rs           public module exports and shared trait implementations
foxtk/src/app.rs           application and timeout API
foxtk/src/prelude.rs       shared traits and component API
foxtk/src/support.rs       strings, argv, and callback support
foxtk/src/widgets/         safe-ish typed wrapper surface by widget family
docs/                      focused widget and toolkit notes
foxtk/examples/             runnable examples and component showcase
.github/workflows/          platform setup/build automation
```

## 12. Change workflow

A change that crosses the FFI boundary should be reviewed across the C declaration, C++ implementation, generated binding usage, Rust wrapper, and relevant example or test. Build changes should be checked against the documented Linux/Windows contract and macOS status. Ownership, callback, thread-affinity, and failure behavior should be documented alongside any new API that changes those contracts.
