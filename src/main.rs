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
mod collisionBox;
mod colours;
mod constants;
mod ellipsis;
mod gameObj;
mod gameObjectManager;
mod line;
mod physicsEngine;
mod point;
mod renderer;
mod square;
mod triangle;
mod utils;
mod vector2;
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
use vector2::Vector2;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use world::World;

use crate::collisionBox::CollisionBox;
use crate::ellipsis::Ellipsis;
use crate::gameObj::{GameObj, IGameObj};
use crate::triangle::Triangle;

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
    baseColour.setHSVA([255, 255, 255, 255]);
    let renderer = Renderer::new(pixels, HEIGHT, WIDTH);

    let mut world = World::new(renderer);

    /*     world.objectManager.createSquare(
           Point::newI(300, 300),
           Point::newI(100, 100),
           baseColour.clone(),
       );
    */

    world.objectManager.createLine(
        Point::newI(100, 500),
        Point::newI(700, 500),
        Colour::createColour(colours::ColourType2::RED),
    );
    world.objectManager.createLine(
        Point::newI(100, 100),
        Point::newI(100, 500),
        Colour::createColour(colours::ColourType2::RED),
    );

    world.objectManager.createLine(
        Point::newI(100, 100),
        Point::newI(700, 100),
        Colour::createColour(colours::ColourType2::RED),
    );
    world.objectManager.createLine(
        Point::newI(700, 100),
        Point::newI(700, 500),
        Colour::createColour(colours::ColourType2::RED),
    );

    /*     world.gObjMM().createCircle(
        Point::newI(300, 300),
        10.0,
        Colour::createColour(colours::ColourType2::BLUE),
    ); */
    /*     world
    .objectManager
    .getGameObj(5)
    .setVelocity(Vector2::newI(2, 2)); */

    for x in 0..1 {
        world
            .gObjMM()
            .createRandCircle((200, 500), (200, 400), (10, 20), (-5, 5), (-5, 5));
    }
    fn test() {
        //left
        let baseI = Vector2::newI(3, 3);
        //r
        let velJ = Vector2::newI(-1, -1);
        //right
        let baseJ = Vector2::newI(-1, 1);
        //s
        let velI = Vector2::newI(1, -1);

        let origBaseI = baseI;
        let origBaseJ = baseJ;
        let origVelJ = velJ;
        let origVelI = velI;

        let baseI = baseI.subtract(&baseJ);
        let velJ = velJ.reverse();

        println!("baseI{:?}", baseI);
        println!("vel{:?}", velI);

        let bx = baseI.x;
        let by = baseI.y;
        let rx = velJ.x;
        let ry = velJ.y;
        let sx = velI.x;
        let sy = velI.y;

        println!("by{}", by);
        println!("ry{}", ry);

        let bx = bx / rx;
        let sx = sx / rx;
        let rx = rx / rx;
        let by = by / ry;
        let sy = sy / ry;
        let ry = ry / ry;

        println!("sy{}", sy);
        println!("ry{}", ry);
        println!("by{}", by);

        let s = -sy + sx;

        println!("sx{}", sx);

        println!("s{}", s);
        let b = bx - by;
        let b = b / s;
        let s = s / s;

        let sGes = b;
        let xGes = bx - s * sx;

        println!("sGes{}", sGes);
        println!("xGes{}", xGes);
        println!("welli{:?}", velI);
        println!("xCoord{}", origBaseI.x + xGes * origVelJ.x);
        println!("yCoord{}", origBaseI.y + xGes * origVelJ.y);
        println!("xCoord{}", origBaseJ.x + sGes * origVelI.x);
        println!("yCoord{}", origBaseJ.y + sGes * origVelI.y);
    }

    test();
    println!("angle{}", Vector2::newI(1, 0).angleTo(&Vector2::newI(1, 1)));
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let start1 = getTime();
            /*
            for x in &mut world.objectManager.gameObj {
                x.getColour().increaseHSVA(1);
            } */

            world.drawAll();
            world.renderer.pixelsObj.render().unwrap();
            let end1 = getTime();
            //println!("Time:{}", end1 - start1);
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
