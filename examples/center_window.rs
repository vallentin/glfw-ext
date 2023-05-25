use std::sync::mpsc::Receiver;

use glfw::{Context, Glfw, Key, OpenGlProfileHint, Window, WindowEvent, WindowHint, WindowMode};
use glfw_ext::WindowExt;

fn main() {
    let (mut glfw, mut wnd, events) = glfw_init();

    // Center the window on the dominant monitor, i.e. if
    // the window is 20% on monitor A and 80% on monitor B,
    // then the window is centered onto monitor B
    wnd.try_center();

    // Center the window on the primary monitor
    glfw.with_primary_monitor(|_glfw, monitor| {
        if let Some(monitor) = monitor {
            wnd.try_center_on_monitor(monitor);
        }
    });

    'main: loop {
        glfw.poll_events();

        for (_timestamp, evt) in glfw::flush_messages(&events) {
            match evt {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                    break 'main;
                }
                _ => {}
            }
        }

        wnd.swap_buffers();
    }

    drop(glfw);
}

fn glfw_init() -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
    let mut glfw = glfw::init(Some(glfw::Callback {
        f: |err, desc, _| panic!("glfw error [{}]: {}", err, desc),
        data: (),
    }))
    .expect("unable to initialize glfw");

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::Visible(false));

    let (mut wnd, events) = glfw
        .create_window(856, 482, env!("CARGO_PKG_NAME"), WindowMode::Windowed)
        .unwrap();

    wnd.set_key_polling(true);
    wnd.set_close_polling(true);

    wnd.make_current();

    wnd.show();

    (glfw, wnd, events)
}
