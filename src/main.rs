extern crate glutin;

use glutin::GlContext;
use std::thread;

fn main() {
    let window = glutin::WindowBuilder::new()
        .with_title("repro")
        .with_multitouch();

    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(false);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    thread::spawn(move || {
        events_loop.run_forever(|event| {
            println!("event");
            glutin::ControlFlow::Continue
        });
        // This will never finish so we'll get a panic at shutdown about stdout being gone, but that's fine
    });

    let handle = thread::spawn(move || {
        println!("Trying to make current...");
        unsafe {
            let _ = gl_window.make_current().unwrap();
        };
        println!("Trying to make current...OK");

        println!("Trying to swap...");
        let _ = gl_window.swap_buffers().unwrap();
        println!("Trying to swap...OK");
    });
    handle.join().unwrap();
}
