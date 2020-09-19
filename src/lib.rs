/*!
[![GitHub CI Status](https://github.com/LPGhatguy/crevice/workflows/CI/badge.svg)](https://github.com/LPGhatguy/crevice/actions)
[![crevice on crates.io](https://img.shields.io/crates/v/crevice.svg)](https://crates.io/crates/crevice)
[![crevice docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/crevice)

Crevice creates GLSL-compatible versions of types through the power of derive
macros. Generated structs implement a number of traits from other crates:

- [`bytemuck::Zeroable`][Zeroable] and [`bytemuck::Pod`][Pod] to ease packing
  data into buffers for uploading.
- [`type_layout::TypeLayout`][TypeLayout] to debug the layout generated by the
  crate.

Crevice is similar to [`glsl-layout`][glsl-layout], but supports `mint` types
and explicitly initializes padding to remove one source of undefined behavior.

## Examples

This example uses `cgmath`, but any math crate that works with the `mint` crate
also works.

```rust
# use crevice::bytemuck;
use crevice::std140::AsStd140;
use cgmath::prelude::*;
use cgmath::{Matrix3, Vector3};

#[derive(AsStd140)]
struct MainUniform {
    orientation: mint::ColumnMatrix3<f32>,
    position: mint::Vector3<f32>,
    scale: f32,
}

let value = MainUniform {
    orientation: Matrix3::identity().into(),
    position: Vector3::new(1.0, 2.0, 3.0).into(),
    scale: 4.0,
};

let value_std140 = value.as_std140();

# fn upload_data_to_gpu(_value: &[u8]) {}
upload_data_to_gpu(bytemuck::bytes_of(&value_std140));
```

## Minimum Supported Rust Version (MSRV)

Crevice supports Rust 1.46.0 and newer due to use of new `const fn` features.

[glsl-layout]: https://github.com/rustgd/glsl-layout
[Zeroable]: https://docs.rs/bytemuck/latest/bytemuck/trait.Zeroable.html
[Pod]: https://docs.rs/bytemuck/latest/bytemuck/trait.Pod.html
[TypeLayout]: https://docs.rs/type-layout/latest/type_layout/trait.TypeLayout.html
*/

pub use bytemuck;
pub use type_layout;

pub mod std140;

#[doc(hidden)]
pub mod internal;

mod mint;
