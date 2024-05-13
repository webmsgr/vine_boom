#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{io::{Read, Seek}, sync::Arc};

use rand::Rng;
use rodio::{Decoder, OutputStream, Sink, Source};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(5.0);
    let src: Audio = 
        Audio::new(std::io::Cursor::new(SOUND));
    let mut rng = rand::thread_rng();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(rng.gen_range(5..=3600)));
        sink.append(src.clone());
        sink.sleep_until_end();
    }
}

struct Audio {
    data: Arc<Vec<f32>>,
    ptr: usize,
    sample_rate: u32,
    channels: u16,
}

impl Audio {
    fn new(vorb: impl Read + Seek + Send + Sync + 'static) -> Audio {
       let dec = Decoder::new_vorbis(vorb).unwrap();
       let sr = dec.sample_rate();
       let c = dec.channels();
        let data: Vec<f32> = dec.convert_samples().collect();
            Self {
                data: Arc::new(data),
                ptr: 0,
                sample_rate: sr,
                channels: c,
            }
    }
}

impl Iterator for Audio {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.get(self.ptr).map(|v| {
            self.ptr += 1;
            *v
        })
    }

}

impl Clone for Audio {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            ptr: 0,
            sample_rate: self.sample_rate,
            channels: self.channels,
        }
    }

}

impl Source for Audio {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}


const SOUND: &[u8] = include_bytes!("../sound.ogg");


