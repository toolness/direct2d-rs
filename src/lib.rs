#![cfg(windows)]

#[macro_use]
extern crate derive_com_wrapper;

#[macro_use]
extern crate auto_enum;

extern crate com_wrapper;
extern crate checked_enum;
extern crate directwrite;
extern crate dxgi;
extern crate either;
extern crate math2d;
extern crate winapi;
extern crate wio;

#[doc(inline)]
pub use crate::device::Device;
#[doc(inline)]
pub use crate::device_context::DeviceContext;
#[doc(inline)]
pub use crate::error::Error;
#[doc(inline)]
pub use crate::factory::Factory;
#[doc(inline)]
pub use crate::render_target::RenderTarget;

#[macro_use]
mod macros;
mod helpers;

pub mod brush;
pub mod device;
pub mod device_context;
pub mod enums;
pub mod error;
pub mod factory;
pub mod geometry;
pub mod image;
pub mod layer;
pub mod properties;
pub mod render_target;
pub mod stroke_style;
