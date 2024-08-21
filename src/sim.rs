use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::automata::Automata;

pub fn run_sim(mut sim: Box<dyn Automata + 'static>) -> Result<(), Error> {
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
        }

        match event {
            Event::RedrawRequested(_) => {
                let frame = pixels.frame_mut();
                sim.before_render();
                sim.render(frame);
                pixels.render().unwrap();
            }
            Event::MainEventsCleared => {
                if !sim_running {
                    return;
                }

                sim.update();
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
