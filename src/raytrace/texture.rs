use super::vec::Vec3;
use std::rc::Rc;
use noise::{NoiseFn, Fbm};

pub trait Texture {
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
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        CheckerTexture{odd, even}
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
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
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.fbm.get(p.elements())).sin())
    }
}
