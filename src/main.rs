#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod pet_handler;
mod config;
mod gfx;
mod gif;

use wgpu::SurfaceError;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;
use winit::window::{WindowBuilder, WindowLevel};
use crate::pet_handler::Pos;

#[cfg(target_os = "windows")]
use winit::platform::windows::WindowBuilderExtWindows;

pub const DOG_SIZE: (i32, i32) = (128, 128);
pub const BORDER_SIZE: (i32, i32) = (16, 16);

// data path
#[cfg(debug_assertions)]
const DATA_PATH: &str = r"./data";

// os release config path
#[cfg(not(debug_assertions))]
#[cfg(target_os = "windows")]
const DATA_PATH: &str = r"C:\Users\%USERNAME%\AppData\Roaming\desktop-pet";
#[cfg(not(debug_assertions))]
#[cfg(target_os = "linux")]
const DATA_PATH: &str = r"~/.config/desktop-pet";
#[cfg(not(debug_assertions))]
#[cfg(target_os = "macos")]
const DATA_PATH: &str = r"~/Library/Application Support/desktop-pet";

fn main() {
    // load the config
    let config = config::Config::load();

    let event_loop_result = EventLoop::new();

    if let Err(e) = event_loop_result {
        eprintln!("Failed to create event loop: {}", e);
        return;
    }

    let event_loop = event_loop_result.unwrap();


    let window = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true)
        .with_resizable(false)
        .with_active(true)
        //.with_undecorated_shadow(false)
        .with_window_level(WindowLevel::AlwaysOnTop)
        .with_title("Desktop Pet")
        .with_inner_size(PhysicalSize::new(128, 128))
        .build(&event_loop)
        .unwrap_or_else(|e| {
            eprintln!("Failed to create window: {}", e);
            std::process::exit(1);
        });

    // create the pet state
    let mut pet_state = pet_handler::PetState::new(config.pet.clone(), Pos::new(0, 0));

    // Initialize wgpu
    let mut wgpu_state = gfx::GfxState::new(&window, pet_state.get_gif()).unwrap_or_else(|e| {
        eprintln!("Failed to initialize wgpu: {}", e);
        std::process::exit(1);
    });

    // get monitor size
    let monitors = window.available_monitors().collect::<Vec<MonitorHandle>>();

    let mut size_x = 1920;
    let mut size_y = 1080;

    if !monitors.is_empty() {
        size_x = 0;
        size_y = 0;
        for monitor in monitors {
            size_x += monitor.size().width;
            size_y += monitor.size().height;
        }
    }

    // todo: rework this to support multiple monitors
    //   the current problem is some monitors can be lower or higher

    size_x = 1920;
    size_y = 1080;

    let result = event_loop.run(|event, elwt| { match event {
        Event::WindowEvent { event, window_id } => {
            if window_id == window.id() {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        // get the available monitors

                        // redraw the window
                        wgpu_state.update(size_x, size_y, &mut pet_state);

                        match wgpu_state.render() {
                            Ok(_) => {}
                            Err(SurfaceError::Lost) => wgpu_state.resize(wgpu_state.window().outer_size()),
                            Err(SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }

                        window.request_redraw();
                    }
                    _ => (),
                }
            }
        }
        _ => {}
    } });

    if let Err(e) = result {
        eprintln!("Failed to run event loop: {}", e);
    }
}
