use std::{cell::RefCell, rc::Rc};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event as WinitEvent, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder as WinitWindowBuilder},
};
pub struct Window {
    window: Option<WinitWindow>,
    window_builder: WinitWindowBuilder,
    name: String,
    resizable: bool,
    callback: Rc<RefCell<dyn FnMut()>>,
}

impl Window {
    fn new(name: String, size: (u32, u32), resizable: bool) -> Self {
        let window_builder =
            WinitWindowBuilder::new()
                .with_title(&name)
                .with_inner_size(LogicalSize {
                    height: size.0 as f64,
                    width: size.1 as f64,
                });
        let callback = Rc::new(RefCell::new(move || ()));
        Self {
            window: None,
            window_builder,
            name,
            resizable,
            callback,
        }
    }
    pub fn run_loop(&mut self) {
        let event_loop = EventLoop::new();
        self.window = Some(self.window_builder.clone().build(&event_loop).unwrap());
        let callback = Rc::clone(&self.callback);
        event_loop.run(move |event, _, control_flow| match event {
            WinitEvent::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    } => match (virtual_keycode, state) {
                        (Some(VirtualKeyCode::A), ElementState::Pressed) => {
                            (&mut *callback.borrow_mut())();
                        }
                        _ => {}
                    },
                },
                _ => {}
            },
            _ => (),
        })
    }
    pub fn set_callback(&mut self, lambda: Box<dyn FnMut()>) {
        self.callback = Rc::new(RefCell::new(lambda));
    }
}

#[derive(Clone)]
pub struct WindowBuilder<'b> {
    name: &'b str,
    size: (u32, u32),
    resizable: bool,
}

impl<'b> WindowBuilder<'b> {
    pub fn new() -> Self {
        Self {
            name: "Nazarust",
            size: (0, 0),
            resizable: false,
        }
    }
    pub fn with_resizable(mut self) -> WindowBuilder<'b> {
        self.resizable = true;
        self
    }
    pub fn build_with(&self, name: &'b str, size: (u32, u32)) -> Window {
        Window::new(name.to_string(), size, self.resizable)
    }
}

use crate::events::{KeyEvent, MouseEvent, State, WindowEvent as NazarustWindowEvent};

trait NazarustEvent {}
impl NazarustEvent for KeyEvent {}
impl NazarustEvent for MouseEvent {}

// I'll probably remove the Unkown variant, it's here because i need a default value,
// or i get errors because of non-exhaustive patterns , and i can't debug in peace

enum NazarustEvents {
    KeyEvent(KeyEvent),
    MouseEvent(MouseEvent),
    WindowEvent(NazarustWindowEvent),
    Unknown,
}
