use pixels::{Error, Pixels, SurfaceTexture};
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{automata::Automata, sim_config::SimConfig};

pub fn run_sim<T: 'static>(
    mut sim: Box<dyn Automata<T> + 'static>,
    config: Option<SimConfig>,
) -> Result<(), Error> {
    let config = config.unwrap_or_default();

    let grid_width = sim.grid_width();
    let grid_height = sim.grid_height();

    let viewport_width = grid_width * sim.render_pixel_scale();
    let viewport_height = grid_height * sim.render_pixel_scale();

    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(viewport_width, viewport_height);

        WindowBuilder::new()
            .with_title("Cellular Automata")
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(viewport_width, viewport_height, surface_texture)?
    };

    env_logger::init();

    let mut sim_running = false;
    let mut sim_rendering = true;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = if sim_running {
            ControlFlow::Poll
        } else {
            ControlFlow::Wait
        };

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                sim_running = !sim_running;
            }

            if input.key_pressed(VirtualKeyCode::R) {
                sim_rendering = !sim_rendering;
            }
        }

        match event {
            Event::RedrawRequested(_) => {
                if !sim_rendering {
                    return;
                }

                let frame = pixels.frame_mut();

                let start_time = Instant::now();

                let context = sim.before_render();

                let elapsed = start_time.elapsed();
                if config.debug {
                    println!("Simulation before_render time: {:?}", elapsed);
                }

                let start_time = Instant::now();

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    sim.render(&context, i, pixel);
                }

                let elapsed = start_time.elapsed();
                if config.debug {
                    println!("Simulation render time: {:?}", elapsed);
                }

                pixels.render().unwrap();
            }
            Event::MainEventsCleared => {
                if !sim_running {
                    return;
                }

                let start_time = Instant::now();

                sim.update();

                let elapsed = start_time.elapsed();
                if config.debug {
                    println!("Simulation update time: {:?}", elapsed);
                }

                window.request_redraw()
            }
            Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
