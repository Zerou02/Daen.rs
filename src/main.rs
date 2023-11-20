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
mod ellipsis;
mod gameObj;
mod line;
mod point;
mod renderer;
mod square;
mod utils;
mod world;

use std::f64::consts::PI;

use circle::Circle;
use colours::{getColourVal, Colour, ColourType};
use constants::{HEIGHT, WIDTH};
use line::Line;
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

use crate::ellipsis::Ellipsis;
use crate::gameObj::GameObj;

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

    let mut c = Colour::new();
    let baseColour = c.createRandHSVA();
    println!("SET");
    baseColour.setHSVA([170, 255, 255, 255]);
    let renderer = Renderer::new(pixels, HEIGHT, WIDTH);
    let square = Square::new(0.0, 0.0, 200.0, 200.0, baseColour.clone());
    let circle = Circle::new(100.0, 100.0, 10.0, baseColour.clone());
    let ellipsis = Ellipsis::new(
        Point { x: 205.0, y: 300.0 },
        Point { x: 400.0, y: 300.0 },
        500.0,
        baseColour.clone(),
    );
    let line = Line::new(
        Point { x: 200.0, y: 300.0 },
        Point { x: 400.0, y: 301.0 },
        baseColour.clone(),
    );
    let line2 = Line::new(
        Point { x: 501.0, y: 100.0 },
        Point { x: 500.0, y: 200.0 },
        baseColour.clone(),
    );
    let mut world = World::new(
        renderer,
        vec![
            //     Box::new(square),
            //      Box::new(circle),
            Box::new(line),
            Box::new(ellipsis),
            //       Box::new(line2),
        ],
    );

    let mut rot = 0.0;
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let start1 = getTime();
            world.renderer.clearBuf(getColourVal(ColourType::BLACK));
            world.objects[0].getColour().increaseHSVA(1);
            world.objects[0].rotate(0.01);
            world.objects[1].getColour().increaseHSVA(1);
            world.objects[1].rotate(0.1);
            //println!("{:?}", world.objects[0].getColour());
            //world.objects[0].moveObj(2, 2);
            // let c = world.objects[1].getColour() + 1;
            //world.objects[1].setColour(c);
            /*            world.renderer.drawEllipsis(
                &Point { x: 200.0, y: 400.0 },
                &Point { x: 400.0, y: 400.0 },
                500,
                getColourVal(ColourType::GREEN),
            ); */
            world.drawAll();
            rot += 0.1;
            /*    world.renderer.drawEllipsis2(
                &Point { x: 300.0, y: 300.0 },
                300,
                10,
                Colour::new().createRandHSVA().rgba,
                rot,
            ); */
            world.renderer.pixelsObj.render().unwrap();
            /*         println!(
                "clear:{};;;draw:{};;;;render:{}",
                end1 - start1,
                end2 - start1,
                end3 - start3
            ); */
            let end1 = getTime();
            // println!("Time:{}", end1 - start1);
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
