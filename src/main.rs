use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use rustic_crystal::cpu::Cpu;
use rustic_crystal::KeypadEvent;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{atomic::AtomicU64, Arc};
use std::thread;

#[derive(Default)]
struct RenderOptions {
    pub linear_interpolation: bool,
}

#[cfg(target_os = "windows")]
fn create_window_builder() -> glium::glutin::window::WindowBuilder {
    use glium::glutin::platform::windows::WindowBuilderExtWindows;
    glium::glutin::window::WindowBuilder::new()
        .with_drag_and_drop(false)
        .with_inner_size(glium::glutin::dpi::LogicalSize::<u32>::from((
            rustic_crystal::SCREEN_W as u32,
            rustic_crystal::SCREEN_H as u32,
        )))
        .with_title("Rustic Crystal")
}

#[cfg(not(target_os = "windows"))]
fn create_window_builder() -> glium::glutin::window::WindowBuilder {
    glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::<u32>::from((
            rustic_crystal::SCREEN_W as u32,
            rustic_crystal::SCREEN_H as u32,
        )))
        .with_title("Rustic Crystal")
}

fn main() -> Result<(), &'static str> {
    env_logger::init();

    let scale = 4;

    let render_delay = Arc::new(AtomicU64::new(16_743));

    let (sender1, receiver1) = mpsc::channel();
    let (sender2, receiver2) = mpsc::sync_channel(1);

    let mut eventloop = glium::glutin::event_loop::EventLoop::new();
    let window_builder = create_window_builder();
    let context_builder = glium::glutin::ContextBuilder::new();
    let display =
        glium::backend::glutin::Display::new(window_builder, context_builder, &eventloop).unwrap();
    set_window_size(display.gl_window().window(), scale);

    let mut texture = glium::texture::texture2d::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        rustic_crystal::SCREEN_W as u32,
        rustic_crystal::SCREEN_H as u32,
    )
    .unwrap();

    let mut renderoptions = <RenderOptions as Default>::default();

    let cputhread = thread::spawn(move || run_game(sender2, receiver1));
    let periodic = timer_periodic(render_delay.clone());

    eventloop.run_return(move |ev, _evtarget, controlflow| {
        use glium::glutin::event::ElementState::{Pressed, Released};
        use glium::glutin::event::VirtualKeyCode;
        use glium::glutin::event::{Event, KeyboardInput, WindowEvent};

        let mut stop = false;
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => stop = true,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        state: Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Key1),
                        ..
                    } => render_delay.store(16_743, std::sync::atomic::Ordering::Relaxed), // 59.7 fps
                    KeyboardInput {
                        state: Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Key2),
                        ..
                    } => render_delay.store(10_000, std::sync::atomic::Ordering::Relaxed), // 100 fps
                    KeyboardInput {
                        state: Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Key3),
                        ..
                    } => render_delay.store(8_333, std::sync::atomic::Ordering::Relaxed), // 120 fps
                    KeyboardInput {
                        state: Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Key4),
                        ..
                    } => render_delay.store(5_000, std::sync::atomic::Ordering::Relaxed), // 200 fps
                    KeyboardInput {
                        state: Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Key5),
                        ..
                    } => render_delay.store(4_166, std::sync::atomic::Ordering::Relaxed), // 240 fps
                    KeyboardInput {
                        state: Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Key6),
                        ..
                    } => render_delay.store(2_500, std::sync::atomic::Ordering::Relaxed), // 400 fps
                    KeyboardInput {
                        state: Pressed,
                        virtual_keycode: Some(VirtualKeyCode::T),
                        ..
                    } => {
                        renderoptions.linear_interpolation = !renderoptions.linear_interpolation;
                    }
                    KeyboardInput {
                        state: Pressed,
                        virtual_keycode: Some(glutinkey),
                        ..
                    } => {
                        if let Some(key) = glutin_to_keypad(glutinkey) {
                            let _ = sender1.send(KeypadEvent::Down(key));
                        }
                    }
                    KeyboardInput {
                        state: Released,
                        virtual_keycode: Some(glutinkey),
                        ..
                    } => {
                        if let Some(key) = glutin_to_keypad(glutinkey) {
                            let _ = sender1.send(KeypadEvent::Up(key));
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                periodic.recv().unwrap();

                match receiver2.recv() {
                    Ok(data) => {
                        recalculate_screen(&display, &mut texture, &data, &renderoptions);
                    }
                    Err(..) => stop = true, // Remote end has hung-up
                }
            }
            _ => (),
        }
        if stop {
            *controlflow = glium::glutin::event_loop::ControlFlow::Exit;
        }
    });

    let _ = cputhread.join();

    Ok(())
}

fn glutin_to_keypad(
    key: glium::glutin::event::VirtualKeyCode,
) -> Option<rustic_crystal::KeypadKey> {
    use glium::glutin::event::VirtualKeyCode;
    match key {
        VirtualKeyCode::Z => Some(rustic_crystal::KeypadKey::A),
        VirtualKeyCode::X => Some(rustic_crystal::KeypadKey::B),
        VirtualKeyCode::Up => Some(rustic_crystal::KeypadKey::Up),
        VirtualKeyCode::Down => Some(rustic_crystal::KeypadKey::Down),
        VirtualKeyCode::Left => Some(rustic_crystal::KeypadKey::Left),
        VirtualKeyCode::Right => Some(rustic_crystal::KeypadKey::Right),
        VirtualKeyCode::Space => Some(rustic_crystal::KeypadKey::Select),
        VirtualKeyCode::Return => Some(rustic_crystal::KeypadKey::Start),
        _ => None,
    }
}

fn recalculate_screen(
    display: &glium::Display,
    texture: &mut glium::texture::texture2d::Texture2d,
    datavec: &[u8],
    renderoptions: &RenderOptions,
) {
    use glium::Surface;

    let interpolation_type = if renderoptions.linear_interpolation {
        glium::uniforms::MagnifySamplerFilter::Linear
    } else {
        glium::uniforms::MagnifySamplerFilter::Nearest
    };

    let rawimage2d = glium::texture::RawImage2d {
        data: std::borrow::Cow::Borrowed(datavec),
        width: rustic_crystal::SCREEN_W as u32,
        height: rustic_crystal::SCREEN_H as u32,
        format: glium::texture::ClientFormat::U8U8U8,
    };
    texture.write(
        glium::Rect {
            left: 0,
            bottom: 0,
            width: rustic_crystal::SCREEN_W as u32,
            height: rustic_crystal::SCREEN_H as u32,
        },
        rawimage2d,
    );

    // We use a custom BlitTarget to transform OpenGL coordinates to row-column coordinates
    let target = display.draw();
    let (target_w, target_h) = target.get_dimensions();
    texture.as_surface().blit_whole_color_to(
        &target,
        &glium::BlitTarget {
            left: 0,
            bottom: target_h,
            width: target_w as i32,
            height: -(target_h as i32),
        },
        interpolation_type,
    );
    target.finish().unwrap();
}

fn run_game(update_screen: SyncSender<Vec<u8>>, keypad_events: Receiver<KeypadEvent>) {
    Cpu::new_cgb(None, update_screen, keypad_events)
        .unwrap()
        .call(0x0100)
}

fn timer_periodic(delay: Arc<AtomicU64>) -> Receiver<()> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || loop {
        let micros = delay.load(std::sync::atomic::Ordering::Relaxed);
        std::thread::sleep(std::time::Duration::from_micros(micros));
        if tx.send(()).is_err() {
            break;
        }
    });
    rx
}

fn set_window_size(window: &glium::glutin::window::Window, scale: u32) {
    use glium::glutin::dpi::{LogicalSize, PhysicalSize};

    let dpi = window.scale_factor();

    let physical_size = PhysicalSize::<u32>::from((
        rustic_crystal::SCREEN_W as u32 * scale,
        rustic_crystal::SCREEN_H as u32 * scale,
    ));
    let logical_size = LogicalSize::<u32>::from_physical(physical_size, dpi);

    window.set_inner_size(logical_size);
}
