//! Utilities and extension methods for [`glfw`].
//!
//! Center window on the dominant monitor.
//!
//! ```no_run
//! # let mut wnd: glfw::Window = todo!();
//! use glfw_ext::WindowExt;
//!
//! // Center the window on the dominant monitor, i.e. if
//! // the window is 20% on monitor A and 80% on monitor B,
//! // then the window is centered onto monitor B
//! wnd.try_center();
//! ```
//!
//! Center window on primary monitor:
//!
//! ```no_run
//! # let glfw: glfw::Glfw = todo!();
//! # let mut wnd: glfw::Window = todo!();
//! use glfw_ext::WindowExt;
//!
//! // Center the window on the primary monitor
//! glfw.with_primary_monitor(|_glfw, monitor| {
//!     if let Some(monitor) = monitor {
//!         wnd.try_center_on_monitor(monitor);
//!     }
//! });
//! ```
//!
//! See [examples/center_window.rs] for a complete example.
//!
//! [`glfw`]: https://crates.io/crates/glfw
//!
//! [examples/center_window.rs]: https://github.com/vallentin/glfw-ext/blob/master/examples/center_window.rs

#![forbid(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use glfw::{Monitor, Window};

/// Extension methods for [`glfw::Window`].
///
/// [`glfw::Window`]: https://docs.rs/glfw/0.51.0/glfw/struct.Window.html
pub trait WindowExt {
    /// Center the window on the dominant monitor,
    ///
    /// If the window is 20% on monitor A and 80% on
    /// monitor B, then the window is centered onto monitor B
    ///
    /// Returns `false` if the window could not be
    /// centered.
    fn try_center(&mut self) -> bool;

    /// Center the window on the primary monitor.
    ///
    /// Returns `false` if the window could not be
    /// centered.
    fn try_center_on_monitor(&mut self, monitor: &Monitor) -> bool;
}

impl WindowExt for Window {
    fn try_center(&mut self) -> bool {
        // `Glfw` is zero-sized and only costs an atomic increment to clone
        self.glfw.clone().with_connected_monitors(|_, monitors| {
            if let Some(monitor) = find_dominant_monitor(self, monitors) {
                self.try_center_on_monitor(monitor)
            } else {
                false
            }
        })
    }

    fn try_center_on_monitor(&mut self, monitor: &Monitor) -> bool {
        let Some(mode) = monitor.get_video_mode() else {
            return false;
        };

        let (mon_x, mon_y) = monitor.get_pos();
        let (wnd_w, wnd_h) = self.get_size();

        let x = mon_x + ((mode.width as i32) - wnd_w) / 2;
        let y = mon_y + ((mode.height as i32) - wnd_h) / 2;
        self.set_pos(x, y);

        true
    }
}

fn find_dominant_monitor<'a>(wnd: &Window, monitors: &'a [Monitor]) -> Option<&'a Monitor> {
    let (wnd_min_x, wnd_min_y) = wnd.get_pos();
    let (wnd_max_x, wnd_max_y) = {
        let (w, h) = wnd.get_size();
        (wnd_min_x + w, wnd_min_y + h)
    };

    monitors
        .iter()
        .filter_map(|monitor| Some((monitor, monitor.get_video_mode()?)))
        .max_by_key(|(monitor, mode)| {
            let (mon_min_x, mon_min_y) = monitor.get_pos();
            let (mon_max_x, mon_max_y) = (
                mon_min_x + (mode.width as i32),
                mon_min_y + (mode.height as i32),
            );

            let (area_min_x, area_min_y) = (wnd_min_x.max(mon_min_x), wnd_min_y.max(mon_min_y));
            let (area_max_x, area_max_y) = (wnd_max_x.min(mon_max_x), wnd_max_y.min(mon_max_y));

            (area_max_x - area_min_x) * (area_max_y - area_min_y)
        })
        .map(|(monitor, _mode)| monitor)
}
