extern crate glutin;
extern crate glium;

use glium::{DisplayBuild, Surface};

fn main() {
    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new().build_glium().unwrap();

    let mut fullscreen = false;

    println!("Press Enter to switch fullscreen mode");

    'main: loop {
        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 0.0, 1.0);
        target.finish();

        // polling and handling the events received by the window
        for event in display.poll_events().into_iter() {
            match event {
                glutin::Event::Closed => break 'main,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _,
                                             Some(glutin::VirtualKeyCode::Return)) =>
                {
                    if fullscreen {
                        glutin::WindowBuilder::new().rebuild_glium(&display).unwrap();
                        fullscreen = false;

                    } else {
                        glutin::WindowBuilder::new().with_fullscreen(glutin::get_primary_monitor())
                                                    .rebuild_glium(&display).unwrap();
                        fullscreen = true;
                    }
                },
                _ => ()
            }
        }
    }
}
