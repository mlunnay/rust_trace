use super::Vec3;
use std::sync::Arc;
use noise::{NoiseFn, Fbm};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct ConstantTexture {
    pub color: Vec3
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Self {
        ConstantTexture{color}
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        CheckerTexture{odd, even}
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        }
        else {
            self.even.value(u, v, p)
        }
    }
}

pub struct MarbleTexture {
    pub scale: f64,
    fbm: Fbm
}

impl MarbleTexture {
    pub fn new(scale: f64) -> Self {
        MarbleTexture{scale, fbm: Fbm::new()}
    }
}

impl Texture for MarbleTexture {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.z() + 10.0 * self.fbm.get(p.elements())).sin())
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        ImageTexture{data, width, height}
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Vec3 {
        let mut i = (u * self.width as f64) as usize;
        let mut j = ((1.0 - v) * self.height as f64) as usize;
        if i > self.width as usize - 1 {
            i = self.width as usize - 1;
        }
        if j > self.height as usize - 1 {
            j = self.height as usize - 1;
        }

        i *= 3;
        j *= 3;
        let r = self.data[i + self.width as usize * j] as f64 / 255.0;
        let g = self.data[i + self.width as usize * j + 1] as f64 / 255.0;
        let b = self.data[i + self.width as usize * j + 2] as f64 / 255.0;
        Vec3::new(r, g, b)
    }
}
