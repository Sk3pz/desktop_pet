use std::fs::File;
use std::path::Path;
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, GenericImageView, RgbaImage};

pub struct Gif {
    pub(crate) name: String,
    frames: Vec<RgbaImage>,
    current: usize,
}

impl Gif {
    pub fn new<S: AsRef<Path>>(path: S, name: String) -> Self {
        let file = File::open(path).unwrap();
        let decoder = GifDecoder::new(file).unwrap();
        let frames = decoder.into_frames().map(|frame| {
            let frame = frame.unwrap().into_buffer();
            RgbaImage::from_vec(frame.width(), frame.height(), frame.into_vec()).unwrap()
        }).collect();

        Self {
            name,
            frames,
            current: 0,
        }
    }

    pub fn current(&self) -> &RgbaImage {
        &self.frames[self.current]
    }

    pub fn next(&mut self) {
        self.current = (self.current + 1) % self.frames.len();
    }
}