use winit::event_loop::{ControlFlow, ActiveEventLoop, EventLoop};
use winit::event::WindowEvent;
use winit::application::ApplicationHandler;
use winit::window::{Window, WindowId};

extern crate gl;

#[derive(Default)]
struct App {
	window: Option<Window>,
	}

impl ApplicationHandler for App {
	fn resumed (&mut self, event_loop: &ActiveEventLoop) {
		self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
		}
	
	fn window_event (&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
		match event {
			WindowEvent::CloseRequested => {
				println!("Killing the process");
				event_loop.exit();
				},
			WindowEvent::RedrawRequested => {
				self.window.as_ref().unwrap().request_redraw();
				}
			_ => ()
			}
		}
	}

fn main() {
	let event_loop = EventLoop::new().unwrap();
	//let window = Window::new(&event_loop).unwrap();
	
	println!("Oblivium Engine. 00000.preproto.1.ty.001p");

	event_loop.set_control_flow(ControlFlow::Poll);
	event_loop.set_control_flow(ControlFlow::Wait);

	let mut app = App::default();
	event_loop.run_app(&mut app);

	//println!("Oblivium Engine. 00000.preproto.1.ty.001p");
}

