//! Defines traits and types for working with data adhering to GLSL's `std140`
//! layout specification.

use std::io::{self, Write};
use std::mem::size_of;

use bytemuck::{bytes_of, Pod, Zeroable};

use crate::internal::align_offset;

pub use crevice_derive::AsStd140;

/// Trait implemented for all `std140` primitives. Generally should not be
/// implemented outside this crate.
pub unsafe trait Std140: Copy + Zeroable + Pod {
    const ALIGNMENT: usize;
}

/**
Trait implemented for all types that can be turned into `std140` values.

This trait can often be `#[derive]`'d instead of manually implementing it. Any
struct which contains only fields that also implement `AsStd140` can derive
`AsStd140`.

Types from the mint crate implement `AsStd140`, making them convenient for use
in uniform types. Most Rust geometry crates, like cgmath, nalgebra, and
ultraviolet support mint.

## Example

```
use cgmath::prelude::*;
use cgmath::{Matrix4, Deg, perspective};
use crevice::std140::AsStd140;

#[derive(AsStd140)]
struct CameraUniform {
    view: mint::ColumnMatrix4<f32>,
    projection: mint::ColumnMatrix4<f32>,
}

let camera = CameraUniform {
    view: Matrix4::identity().into(),
    projection: perspective(Deg(60.0), 16.0/9.0, 0.01, 100.0).into(),
};

// Crevice's Std140 types implement bytemuck's Zeroable and Pod traits, making
// it easy to turn them into bytes for uploading.

# fn write_to_gpu_buffer(bytes: &[u8]) {}
let camera_std140 = camera.as_std140();
write_to_gpu_buffer(bytemuck::bytes_of(&camera_std140));
```
*/
pub trait AsStd140 {
    type Std140Type: Std140;

    fn as_std140(&self) -> Self::Std140Type;
}

impl<T> AsStd140 for T
where
    T: Std140,
{
    type Std140Type = Self;

    fn as_std140(&self) -> Self {
        *self
    }
}

/**
Type that enables writing correctly aligned `std140` values to a buffer.

`Writer` is useful when many values need to be laid out in a row that cannot be
represented by a struct alone, like dynamically sized arrays or dynamically
laid-out values.

## Example
In this example, we'll write a length-prefixed list of lights to a buffer.
`std140::Writer` helps align correctly, even across multiple structs, which can
be tricky and error-prone otherwise.

```
use crevice::std140::{self, AsStd140};

#[derive(AsStd140)]
struct PointLight {
    position: mint::Vector3<f32>,
    color: mint::Vector3<f32>,
    brightness: f32,
}

let lights = vec![
    PointLight {
        position: [0.0, 1.0, 0.0].into(),
        color: [1.0, 0.0, 0.0].into(),
        brightness: 0.6,
    },
    PointLight {
        position: [0.0, 4.0, 3.0].into(),
        color: [1.0, 1.0, 1.0].into(),
        brightness: 1.0,
    },
];

# fn map_gpu_buffer_for_write() -> &'static mut [u8] {
#     Box::leak(vec![0; 1024].into_boxed_slice())
# }
let target_buffer = map_gpu_buffer_for_write();
let mut writer = std140::Writer::new(target_buffer);

let light_count = lights.len() as u32;
writer.write(&light_count)?;

// Crevice will automatically insert the required padding to align the
// PointLight structure correctly. In this case, there will be 12 bytes of
// padding between the length field and the light list.

for light in &lights {
    writer.write(light)?;

    // Crevice will also pad between each array element.
}

# fn unmap_gpu_buffer() {}
unmap_gpu_buffer();

# Ok::<(), std::io::Error>(())
```
*/
pub struct Writer<W> {
    writer: W,
    offset: usize,
}

impl<W: Write> Writer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer, offset: 0 }
    }

    pub fn write<T>(&mut self, value: &T) -> io::Result<()>
    where
        T: AsStd140,
    {
        let size = size_of::<<T as AsStd140>::Std140Type>();
        let alignment = <T as AsStd140>::Std140Type::ALIGNMENT;
        let padding = align_offset(self.offset, alignment);

        for _ in 0..padding {
            self.writer.write_all(&[0])?;
        }
        self.offset += padding;

        let value = value.as_std140();
        self.writer.write_all(bytes_of(&value))?;
        self.offset += size;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.offset
    }
}

unsafe impl Std140 for f32 {
    const ALIGNMENT: usize = 4;
}

unsafe impl Std140 for f64 {
    const ALIGNMENT: usize = 8;
}

unsafe impl Std140 for i32 {
    const ALIGNMENT: usize = 4;
}

unsafe impl Std140 for u32 {
    const ALIGNMENT: usize = 4;
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

unsafe impl Zeroable for Vec2 {}
unsafe impl Pod for Vec2 {}

unsafe impl Std140 for Vec2 {
    const ALIGNMENT: usize = 8;
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

unsafe impl Zeroable for Vec3 {}
unsafe impl Pod for Vec3 {}

unsafe impl Std140 for Vec3 {
    const ALIGNMENT: usize = 16;
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

unsafe impl Zeroable for Vec4 {}
unsafe impl Pod for Vec4 {}

unsafe impl Std140 for Vec4 {
    const ALIGNMENT: usize = 16;
}

#[derive(Debug, Clone, Copy)]
pub struct Mat2 {
    pub x: Vec2,
    pub _pad_y: [f32; 2],
    pub y: Vec2,
}

unsafe impl Zeroable for Mat2 {}
unsafe impl Pod for Mat2 {}

unsafe impl Std140 for Mat2 {
    const ALIGNMENT: usize = 16;
}

#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub x: Vec3,
    pub _pad_y: f32,
    pub y: Vec3,
    pub _pad_z: f32,
    pub z: Vec3,
}

unsafe impl Zeroable for Mat3 {}
unsafe impl Pod for Mat3 {}

unsafe impl Std140 for Mat3 {
    const ALIGNMENT: usize = 16;
}

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
    pub w: Vec4,
}

unsafe impl Zeroable for Mat4 {}
unsafe impl Pod for Mat4 {}

unsafe impl Std140 for Mat4 {
    const ALIGNMENT: usize = 16;
}
