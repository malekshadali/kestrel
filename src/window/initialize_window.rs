use std::rc::Rc;

use fontdue::{Font, FontSettings};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use super::renderer::{ rgb, draw_text };
use crate::shell::commands::handle_command;

#[derive(Default)]
struct App {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
    context: Option<softbuffer::Context<Rc<Window>>>,
    font: Option<Font>,
    input_line: String,
    history: Vec<String>,
}

impl ApplicationHandler for App {
    // Called when the event loop starts (or resumes on mobile/Android)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes().with_title("Kestrel");
        let window = Rc::new(event_loop.create_window(attrs).unwrap());

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        let font_bytes = include_bytes!("../../assets/JetBrainsMono-Bold.ttf");
        let font = Font::from_bytes(font_bytes as &[u8], FontSettings::default()).unwrap();

        self.font = Some(font);
        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);
    }

    //Called for every window-specific event
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(surface)) = (&self.window, &mut self.surface) {
                    let size = window.inner_size();
                    surface
                        .resize(
                            std::num::NonZeroU32::new(size.width).unwrap(),
                            std::num::NonZeroU32::new(size.height).unwrap(),
                        )
                        .unwrap();

                    let mut buffer = surface.buffer_mut().unwrap();
                    for pixel in buffer.iter_mut() {
                        *pixel = 0x00_10_14_1a;
                    }

                    if let Some(font) = &self.font {
                        let line_height = 20u32;

                        //draw history
                        for (i, line) in self.history.iter().enumerate() {
                            draw_text(
                                &mut buffer,
                                font,
                                line,
                                10,
                                20 + i as u32 * line_height,
                                16.0,
                                rgb(200, 200, 200),
                                size.width,
                            );
                        }

                        //draw current input with prompt
                        let prompt = format!("> {}", self.input_line);
                        draw_text(
                            &mut buffer,
                            font,
                            &prompt,
                            10,
                            20 + self.history.len() as u32 * line_height,
                            16.0,
                            rgb(23, 147, 209),
                            size.width,
                        );
                    }
                    buffer.present().unwrap();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    if let winit::keyboard::Key::Character(ch) = &event.logical_key {
                        self.input_line.push_str(ch.as_str());
                        self.window.as_ref().unwrap().request_redraw();
                    }
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Backspace) =
                        &event.logical_key
                    {
                        self.input_line.pop();
                        self.window.as_ref().unwrap().request_redraw();
                    }
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Enter) = &event.logical_key {
                        let cmd = self.input_line.clone();
                        handle_command(&cmd, &mut self.history);
                        self.input_line.clear();
                    }
                    if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Space) = &event.logical_key {
                        self.input_line.push(' ');
                        self.window.as_ref().unwrap().request_redraw();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}