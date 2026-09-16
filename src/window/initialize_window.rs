use std::rc::Rc;

use tabled::settings::object::ColumnsIter;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use fontdue::{Font, FontSettings};

#[derive(Default)]
struct App {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
    context: Option<softbuffer::Context<Rc<Window>>>,
    font: Option<Font>,
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
                    // let rect_x = 50u32;
                    // let rect_y = 50u32;
                    // let rect_w = 200u32;
                    // let rect_h = 100u32;

                    // for y in rect_y..(rect_y + rect_h) {
                    //     for x in rect_x..(rect_x + rect_w) {
                    //         let idx = (y * size.width + x) as usize;
                    //         buffer[idx] =  rgb(255, 100, 0);
                    //     }
                    // }

                    if let Some(font) = &self.font {
                        draw_text(&mut buffer, font, "kestrel@Desktop", 10, 20, 16.0, rgb(23, 147, 209), size.width);
                    }
                    buffer.present().unwrap();
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

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn draw_text(
    buffer: &mut [u32],
    font: &fontdue::Font,
    text: &str,
    x: u32,
    y: u32,
    size: f32,
    color: u32,
    screen_width: u32,
) {
    let mut cursor_x = x;

    for ch in text.chars() {
        let (metrics, bitmap) = font.rasterize(ch, size);

        for row in 0..metrics.height {
            for col in 0..metrics.width {
                let coverage = bitmap[row * metrics.width + col];
                if coverage > 0 {
                    let pixel_y = y as i32 + row as i32 - metrics.ymin as i32 - metrics.height as i32;             
                    if pixel_y < 0 { continue;}
                    let idx = pixel_y as u32 * screen_width + (cursor_x + col as u32);
                    let alpha = coverage as u32;
                    let r = (color >> 16) & 0xff;
                    let g = (color >> 8) & 0xff;
                    let b = color & 0xff;
                    buffer[idx as usize] = ((r * alpha / 255) << 16) | ((g * alpha / 255) << 8) | ((b * alpha / 255));
                }
            }
        }
        cursor_x += metrics.advance_width as u32
    }
}