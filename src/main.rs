use rustic_crystal::cpu::Cpu;
use rustic_crystal::KeypadEvent;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{atomic::AtomicU64, Arc};
use std::thread;
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

#[cfg(target_os = "windows")]
fn create_window_attributes() -> winit::window::WindowAttributes {
    use winit::platform::windows::WindowAttributesExtWindows;
    winit::window::Window::default_attributes()
        .with_drag_and_drop(false)
        .with_title("Rustic Crystal")
}

#[cfg(not(target_os = "windows"))]
fn create_window_attributes() -> winit::window::WindowAttributes {
    winit::window::Window::default_attributes().with_title("Rustic Crystal")
}

fn main() -> Result<(), &'static str> {
    env_logger::init();

    let scale = 4;

    let render_delay = Arc::new(AtomicU64::new(16));

    let (sender1, receiver1) = mpsc::channel();
    let (sender2, receiver2) = mpsc::sync_channel(1);

    let mut event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window_attributes = create_window_attributes();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_attributes)
        .build(&event_loop);
    set_window_size(&window, scale);

    let mut texture = glium::texture::texture2d::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        rustic_crystal::SCREEN_W as u32,
        rustic_crystal::SCREEN_H as u32,
    )
    .unwrap();

    let cputhread = thread::spawn(move || run_game(sender2, receiver1));
    // let periodic = timer_periodic(render_delay.clone());

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    loop {
        let timeout = Some(std::time::Duration::ZERO);
        let status = event_loop.pump_events(timeout, |ev, elwt| {
            use winit::event::ElementState::{Pressed, Released};
            use winit::event::{Event, WindowEvent};
            use winit::keyboard::Key;

            if let Event::WindowEvent { event, .. } = ev {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput {
                        event: keyevent, ..
                    } => {
                        eprintln!("{:?} {:?}", keyevent.state, keyevent.logical_key);
                        match (keyevent.state, keyevent.logical_key.as_ref()) {
                            (Pressed, Key::Character("1")) => {
                                // 59.7 fps
                                render_delay.store(16_743, std::sync::atomic::Ordering::Relaxed)
                            }
                            (Pressed, Key::Character("2")) => {
                                // 100 fps
                                render_delay.store(10_000, std::sync::atomic::Ordering::Relaxed)
                            }
                            (Pressed, Key::Character("3")) => {
                                // 120 fps
                                render_delay.store(8_333, std::sync::atomic::Ordering::Relaxed)
                            }
                            (Pressed, Key::Character("4")) => {
                                // 200 fps
                                render_delay.store(5_000, std::sync::atomic::Ordering::Relaxed)
                            }
                            (Pressed, Key::Character("5")) => {
                                // 240 fps
                                render_delay.store(4_166, std::sync::atomic::Ordering::Relaxed)
                            }
                            (Pressed, Key::Character("6")) => {
                                // 400 fps
                                render_delay.store(2_500, std::sync::atomic::Ordering::Relaxed)
                            }
                            (Pressed, Key::Character("7")) => {
                                // 1000 fps
                                render_delay.store(1_000, std::sync::atomic::Ordering::Relaxed)
                            }
                            (Pressed, winitkey) => {
                                if let Some(key) = winit_to_keypad(winitkey) {
                                    let _ = sender1.send(KeypadEvent::Down(key));
                                }
                            }
                            (Released, winitkey) => {
                                if let Some(key) = winit_to_keypad(winitkey) {
                                    let _ = sender1.send(KeypadEvent::Up(key));
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        });

        if let PumpStatus::Exit(_) = status {
            break;
        }

        // periodic.recv().unwrap();
        // while periodic.try_recv().is_ok() {}

        match receiver2.recv() {
            Ok(data) => recalculate_screen(&display, &mut texture, &data),
            Err(..) => break, // Remote end has hung-up
        }
    }

    let _ = cputhread.join();

    Ok(())
}

fn winit_to_keypad(key: winit::keyboard::Key<&str>) -> Option<rustic_crystal::KeypadKey> {
    use winit::keyboard::{Key, NamedKey};
    match key {
        Key::Character("z" | "Z") => Some(rustic_crystal::KeypadKey::A),
        Key::Character("x" | "X") => Some(rustic_crystal::KeypadKey::B),
        Key::Named(NamedKey::ArrowUp) => Some(rustic_crystal::KeypadKey::Up),
        Key::Named(NamedKey::ArrowDown) => Some(rustic_crystal::KeypadKey::Down),
        Key::Named(NamedKey::ArrowLeft) => Some(rustic_crystal::KeypadKey::Left),
        Key::Named(NamedKey::ArrowRight) => Some(rustic_crystal::KeypadKey::Right),
        Key::Named(NamedKey::Space) => Some(rustic_crystal::KeypadKey::Select),
        Key::Named(NamedKey::Enter) => Some(rustic_crystal::KeypadKey::Start),
        _ => None,
    }
}

fn recalculate_screen<
    T: glium::glutin::surface::SurfaceTypeTrait + glium::glutin::surface::ResizeableSurface + 'static,
>(
    display: &glium::Display<T>,
    texture: &mut glium::texture::texture2d::Texture2d,
    datavec: &[u8],
) {
    use glium::Surface;

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
        glium::uniforms::MagnifySamplerFilter::Nearest,
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
        // eprintln!("Waiting for {micros} micros");
        std::thread::sleep(std::time::Duration::from_micros(micros));
        if tx.send(()).is_err() {
            break;
        }
    });
    rx
}

fn set_window_size(window: &winit::window::Window, scale: u32) {
    let _ = window.request_inner_size(winit::dpi::LogicalSize::<u32>::from((
        rustic_crystal::SCREEN_W as u32 * scale,
        rustic_crystal::SCREEN_H as u32 * scale,
    )));
}
