# glfw-ext

[![Latest Version](https://img.shields.io/crates/v/glfw-ext.svg)](https://crates.io/crates/glfw-ext)
[![Docs](https://docs.rs/glfw-ext/badge.svg)](https://docs.rs/glfw-ext)
[![License](https://img.shields.io/github/license/vallentin/glfw-ext.svg)](https://github.com/vallentin/glfw-ext)

Utilities and extension methods for [`glfw`].

Center window on the dominant monitor.

```rust
use glfw_ext::WindowExt;

// Center the window on the dominant monitor, i.e. if
// the window is 20% on monitor A and 80% on monitor B,
// then the window is centered onto monitor B
wnd.try_center();
```

Center window on primary monitor:

```rust
use glfw_ext::WindowExt;

// Center the window on the primary monitor
glfw.with_primary_monitor(|_glfw, monitor| {
    if let Some(monitor) = monitor {
        wnd.try_center_on_monitor(monitor);
    }
});
```

See [examples/center_window.rs] for a complete example.

[`glfw`]: https://crates.io/crates/glfw

[examples/center_window.rs]: https://github.com/vallentin/glfw-ext/blob/master/examples/center_window.rs
