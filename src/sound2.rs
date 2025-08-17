use rodio::{stream::OutputStream, OutputStreamBuilder, Sink};

pub trait Sfx<TSource> {
    fn open(self) -> TSource;
}

pub trait Music<TSource>: Sfx<TSource> {
    fn id(&self) -> u32;
}

pub struct Sound2 {
    music: Option<(u32, Sink)>,
    sfx: Option<Sink>,
    stream: OutputStream,
}

impl Sound2 {
    pub fn new() -> Self {
        Sound2 {
            music: None,
            sfx: None,
            stream: OutputStreamBuilder::open_default_stream().unwrap(),
        }
    }

    pub fn stop_music(&mut self) {
        if let Some((_, sink)) = self.music.take() {
            sink.stop();
        }
    }

    pub fn stop_sfx(&mut self) {
        if let Some(sink) = self.sfx.take() {
            sink.stop();
        }
    }

    fn is_playing_music(&self, id: u32) -> bool {
        if let Some((playing, _)) = self.music.as_ref() {
            *playing == id
        } else {
            false
        }
    }

    pub fn start_music<T, TSource>(&mut self, music: T)
    where
        T: Music<TSource>,
        TSource: rodio::Source + Send + 'static,
        f32: cpal::FromSample<TSource::Item>,
    {
        let id = music.id();

        if self.is_playing_music(id) {
            return; // Allready playing this music
        }

        self.stop_music();

        let sink = Sink::connect_new(self.stream.mixer());
        sink.append(music.open());
        self.music = Some((id, sink));
    }

    pub fn play_sfx<T, TSource>(&mut self, sound: T)
    where
        T: Sfx<TSource>,
        TSource: rodio::Source + Send + 'static,
        f32: cpal::FromSample<TSource::Item>,
    {
        if let Some(sink) = self.sfx.take() {
            sink.stop();
        }

        let sink = Sink::connect_new(self.stream.mixer());
        sink.append(sound.open());
        self.sfx = Some(sink);
    }
}
