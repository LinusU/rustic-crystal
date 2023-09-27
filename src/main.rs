use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use rustic_crystal::cpu::Cpu;
use rustic_crystal::{KeypadEvent, Sound};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
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
    let scale = 4;

    let (sender1, receiver1) = mpsc::channel();
    let (sender2, receiver2) = mpsc::sync_channel(1);

    let mut cpu = Cpu::new_cgb(None, sender2, receiver1)?;

    let cpal_audio_stream = match CpalPlayer::get() {
        Some((v, s)) => {
            cpu.mmu.sound = Some(Sound::new_cgb(
                Box::new(v) as Box<dyn rustic_crystal::AudioPlayer>
            ));
            s
        }
        None => {
            return Err("Could not open audio device");
        }
    };

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

    let cputhread = thread::spawn(move || run_cpu(cpu));

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
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } => stop = true,
                    KeyboardInput {
                        state: Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Key1),
                        ..
                    } => set_window_size(display.gl_window().window(), 1),
                    KeyboardInput {
                        state: Pressed,
                        virtual_keycode: Some(VirtualKeyCode::R),
                        ..
                    } => set_window_size(display.gl_window().window(), scale),
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
                match receiver2.recv() {
                    Ok(data) => recalculate_screen(&display, &mut texture, &data, &renderoptions),
                    Err(..) => stop = true, // Remote end has hung-up
                }
            }
            _ => (),
        }
        if stop {
            *controlflow = glium::glutin::event_loop::ControlFlow::Exit;
        }
    });

    drop(cpal_audio_stream);
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

fn run_cpu(mut cpu: Cpu) {
    let periodic = timer_periodic(16);

    let waitticks = (4194304f64 / 1000.0 * 16.0).round() as u32;
    let mut ticks = 0;

    loop {
        while ticks < waitticks {
            ticks += cpu.do_cycle();
        }

        ticks -= waitticks;

        let _ = periodic.recv();
    }
}

fn timer_periodic(ms: u64) -> Receiver<()> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(ms));
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

struct CpalPlayer {
    buffer: Arc<Mutex<Vec<(f32, f32)>>>,
    sample_rate: u32,
}

impl CpalPlayer {
    fn get() -> Option<(CpalPlayer, cpal::Stream)> {
        let device = match cpal::default_host().default_output_device() {
            Some(e) => e,
            None => return None,
        };

        // We want a config with:
        // chanels = 2
        // SampleFormat F32
        // Rate at around 44100

        let wanted_samplerate = cpal::SampleRate(44100);
        let supported_configs = match device.supported_output_configs() {
            Ok(e) => e,
            Err(_) => return None,
        };
        let mut supported_config = None;
        for f in supported_configs {
            if f.channels() == 2 && f.sample_format() == cpal::SampleFormat::F32 {
                if f.min_sample_rate() <= wanted_samplerate
                    && wanted_samplerate <= f.max_sample_rate()
                {
                    supported_config = Some(f.with_sample_rate(wanted_samplerate));
                } else {
                    supported_config = Some(f.with_max_sample_rate());
                }
                break;
            }
        }
        supported_config.as_ref()?;

        let selected_config = supported_config.unwrap();

        let sample_format = selected_config.sample_format();
        let config: cpal::StreamConfig = selected_config.into();

        let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

        let shared_buffer = Arc::new(Mutex::new(Vec::new()));
        let stream_buffer = shared_buffer.clone();

        let player = CpalPlayer {
            buffer: shared_buffer,
            sample_rate: config.sample_rate.0,
        };

        let stream = match sample_format {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config,
                move |data: &mut [f32], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::U16 => device.build_output_stream(
                &config,
                move |data: &mut [u16], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::I16 => device.build_output_stream(
                &config,
                move |data: &mut [i16], _callback_info: &cpal::OutputCallbackInfo| {
                    cpal_thread(data, &stream_buffer)
                },
                err_fn,
                None,
            ),
            _ => panic!("Unsupported sample format: {:?}", sample_format),
        }
        .unwrap();

        stream.play().unwrap();

        Some((player, stream))
    }
}

fn cpal_thread<T: cpal::Sample + cpal::FromSample<f32>>(
    outbuffer: &mut [T],
    audio_buffer: &Arc<Mutex<Vec<(f32, f32)>>>,
) {
    let mut inbuffer = audio_buffer.lock().unwrap();
    let outlen = ::std::cmp::min(outbuffer.len() / 2, inbuffer.len());
    for (i, (in_l, in_r)) in inbuffer.drain(..outlen).enumerate() {
        outbuffer[i * 2] = cpal::Sample::from_sample(in_l);
        outbuffer[i * 2 + 1] = cpal::Sample::from_sample(in_r);
    }
}

impl rustic_crystal::AudioPlayer for CpalPlayer {
    fn play(&mut self, buf_left: &[f32], buf_right: &[f32]) {
        debug_assert!(buf_left.len() == buf_right.len());

        let mut buffer = self.buffer.lock().unwrap();

        for (l, r) in buf_left.iter().zip(buf_right) {
            if buffer.len() > self.sample_rate as usize {
                // Do not fill the buffer with more than 1 second of data
                // This speeds up the resync after the turning on and off the speed limiter
                return;
            }
            buffer.push((*l, *r));
        }
    }

    fn samples_rate(&self) -> u32 {
        self.sample_rate
    }

    fn underflowed(&self) -> bool {
        (*self.buffer.lock().unwrap()).is_empty()
    }
}
