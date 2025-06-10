// src/image/image.rs
use std::any::Any;
use bytemuck::Pod;
use num_traits::NumCast;

#[derive(Debug, Clone)]
pub struct Image<T> {
    pub voxels: Vec<T>,
    pub width:  u32,
    pub height: u32,
    pub depth:  u32,
    pub spacing: (f64, f64, f64),
    pub origin:  (f64, f64, f64),
    pub direction: (u8, u8, u8, u8, u8, u8, u8, u8, u8),
}

impl<T> Image<T> {
    pub fn num_voxels(&self) -> usize {
        (self.width as usize) * (self.height as usize) * (self.depth as usize)
    }
}

/// We don't know T at compiletime so this is a placeholder.
pub trait AnyImage: Any {
    fn as_any(&self) -> &dyn Any;
    fn width(&self)  -> u32;
    fn height(&self) -> u32;
    fn depth(&self)  -> u32;
    fn spacing(&self) -> (f64, f64, f64);
    fn origin(&self)  -> (f64, f64, f64);
    fn direction(&self)  -> (u8, u8, u8, u8, u8, u8, u8, u8, u8);

    /// Iterate (lazily) through all values as f64.
    fn iter_f64(&self) -> Box<dyn Iterator<Item = f64> + '_>;
}

/// Covers u8, i8, u16, i16, u32, i32, f32, f64, i64, u64, etc. (but not `bool`)
impl<T> AnyImage for Image<T>
where
    T: Pod + NumCast + Copy + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn depth(&self) -> u32 {
        self.depth
    }
    fn spacing(&self) -> (f64, f64, f64) {
        self.spacing
    }
    fn origin(&self) -> (f64, f64, f64) {
        self.origin
    }
    fn direction(&self) -> (u8, u8, u8, u8, u8, u8, u8, u8, u8) {
        self.direction
    }

    fn iter_f64(&self) -> Box<dyn Iterator<Item = f64> + '_> {
        Box::new(self.voxels.iter().map(|&x| NumCast::from(x).unwrap()))
    }
}

