# glium-core

**glium-core** is a simple OpenGL wrapper in Rust that you can use if you want things to *just work*.

## Installation

```toml
[dependencies.glium_core]
git = "http://github.com/glium/glium-core"
```

## [Documentation](http://rust-ci.org/glium/glium-core/doc/glium_core/index.html)

Everything is explained in the documentation.

## Example

```rust
#![feature(phase)]

#[phase(plugin)]
extern crate glium_core_macros;

extern crate gl_init;
extern crate glium_core;

fn main() {
    use glium_core::DisplayBuild;

    // building the display, ie. the main object
    let display = gl_init::WindowBuilder::new()
        .build_glium_core()
        .unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[vertex_format]
        struct Vertex {
            iPosition: [f32, ..2],
            iColor: [f32, ..3],
        }

        glium_core::VertexBuffer::new(&display, 
            vec![
                Vertex { iPosition: [-0.5, -0.5], iColor: [0.0, 1.0, 0.0] },
                Vertex { iPosition: [ 0.0,  0.5], iColor: [0.0, 0.0, 1.0] },
                Vertex { iPosition: [ 0.5, -0.5], iColor: [1.0, 0.0, 0.0] },
            ]
        )
    };

    // building the index buffer
    let index_buffer = display.build_index_buffer(glium_core::TrianglesList,
        &[ 0u16, 1, 2 ]);

    // compiling shaders and linking them together
    let program = glium_core::Program::new(&display,
        // vertex shader
        "
            #version 110

            uniform mat4 uMatrix;

            attribute vec2 iPosition;
            attribute vec3 iColor;

            varying vec3 vColor;

            void main() {
                gl_Position = vec4(iPosition, 0.0, 1.0) * uMatrix;
                vColor = iColor;
            }
        ",

        // fragment shader
        "
            #version 110
            varying vec3 vColor;

            void main() {
                gl_FragColor = vec4(vColor, 1.0);
            }
        ",

        // geometry shader
        None)
        .unwrap();

    // creating an object that will allow us to set the uniforms of our shaders
    let mut program = program.build_uniforms();
    program.set_value("uMatrix", [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ]);
    
    // the main loop
    // each cycle will draw once
    'main: loop {
        use std::io::timer;
        use std::time::Duration;

        // drawing a frame
        let target = display.draw();
        target.draw(&vertex_buffer, &index_buffer, &program);
        target.finish();

        // sleeping for some time in order not to use up too much CPU
        timer::sleep(Duration::milliseconds(17));

        // polling and handling the events received by the window
        for event in display.poll_events().move_iter() {
            match event {
                gl_init::Closed => break 'main,
                _ => ()
            }
        }
    }
}
```

## Note

The `#[vertex_format]` syntax extension was shamefully copied from [gfx-rs](https://github.com/gfx-rs/gfx-rs).