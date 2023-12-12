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
mod matrix;
mod physicsEngine;
mod point;
mod renderer;
mod square;
mod triangle;
mod utils;
mod vector2;
mod world;

use std::env::args;
use std::f64::consts::PI;
use std::fs;

use circle::Circle;
use colours::{getColourVal, Colour, ColourType};
use constants::{HEIGHT, WIDTH};
use gameObj::BehaviourMap;
use line::Line;
use point::Point;
use renderer::Renderer;
use serde_json::{self, Value};
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
use crate::matrix::Matrix;
use crate::triangle::Triangle;
use crate::utils::rotatePoint;

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

    let mut world = World::new(renderer);
    let args: Vec<String> = args().collect();
    let file = match args.get(1) {
        Some(a) => a,
        None => panic!("Baka. Name a file"),
    };

    let content = fs::read_to_string(file.to_owned() + ".json")
        .expect("Baka. Name a correct file. Or get Permission to do your stuff");
    let test: Vec<Value> = serde_json::from_str(&content).unwrap();
    world.gObjMM().parseConfig(test);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let start1 = getTime();
            world.drawAll();
            world.renderer.pixelsObj.render().unwrap();
            let end1 = getTime();
        }
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if let Some(size) = input.window_resized() {
                world
                    .renderer
                    .pixelsObj
                    .resize_surface(size.width, size.height)
                    .unwrap();
                return;
            }
            window.request_redraw();
        }
    });
}
