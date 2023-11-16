#![allow(
    unused_parens,
    non_snake_case,
    dead_code,
    unused_variables,
    unreachable_code,
    unreachable_patterns,
    unused_imports
)]
mod circle;
mod colours;
mod constants;
mod gameObj;
mod point;
mod renderer;
mod square;
mod utils;
mod world;

use circle::Circle;
use colours::{getColourVal, ColourType};
use constants::{HEIGHT, WIDTH};
use point::Point;
use renderer::Renderer;
use square::Square;
use utils::getTime;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use world::World;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    let renderer = Renderer::new(pixels, HEIGHT, WIDTH);
    let square = Square::new(0, 0, 200, 200, getColourVal(ColourType::BLUE));
    let circle = Circle::new(100, 100, 10, getColourVal(ColourType::BLUE));
    let mut world = World::new(renderer, vec![Box::new(square), Box::new(circle)]);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.renderer.clearBuf(getColourVal(ColourType::BLACK));
            let start1 = getTime();
            for obj in &world.objects {
                obj.draw(&mut world.renderer)
            }
            world.objects[0].moveObj(2, 2);
            let mut c = world.objects[1].getColour() + 1;
            world.objects[1].setColour(c);
            world.renderer.drawEllipsis(
                &Point { x: 200, y: 400 },
                &Point { x: 400, y: 400 },
                500,
                getColourVal(ColourType::GREEN),
            );
            let end1 = getTime();
            world.renderer.pixelsObj.render().unwrap();
            /*         println!(
                "clear:{};;;draw:{};;;;render:{}",
                end1 - start1,
                end2 - start1,
                end3 - start3
            ); */
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                world
                    .renderer
                    .pixelsObj
                    .resize_surface(size.width, size.height)
                    .unwrap();
                return;
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}
