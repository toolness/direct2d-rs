use crate::factory::Factory;
use math2d::*;

use std::{mem, ptr};

use winapi::um::d2d1::ID2D1Brush;
use winapi::um::d2d1_1::ID2D1Factory1;
use wio::com::ComPtr;

#[doc(inline)]
pub use crate::brush::gradient::linear::LinearGradientBrush;
#[doc(inline)]
pub use crate::brush::gradient::radial::RadialGradientBrush;
#[doc(inline)]
pub use crate::brush::gradient::{GradientStop, GradientStopCollection};
#[doc(inline)]
pub use crate::brush::solid_color::SolidColorBrush;

pub mod bitmap;
pub mod gradient;
pub mod solid_color;

pub trait Brush {
    unsafe fn get_ptr(&self) -> *mut ID2D1Brush;

    #[inline]
    fn get_factory(&self) -> Factory {
        unsafe {
            let mut ptr = ptr::null_mut();
            (*self.get_ptr()).GetFactory(&mut ptr);

            let ptr: ComPtr<ID2D1Factory1> = ComPtr::from_raw(ptr).cast().unwrap();
            Factory::from_raw(ptr.into_raw())
        }
    }

    #[inline]
    fn to_generic(&self) -> GenericBrush {
        let ptr = unsafe { ComPtr::from_raw(self.get_ptr()) };
        mem::forget(ptr.clone());
        GenericBrush { ptr }
    }

    #[inline]
    fn set_opacity(&mut self, opacity: f32) {
        unsafe {
            (*self.get_ptr()).SetOpacity(opacity);
        }
    }

    #[inline]
    fn set_transform(&mut self, transform: &Matrix3x2f) {
        unsafe {
            (*self.get_ptr()).SetTransform((&transform) as *const _ as *const _);
        }
    }

    #[inline]
    fn get_opacity(&self) -> f32 {
        unsafe { (*self.get_ptr()).GetOpacity() }
    }

    #[inline]
    fn get_transform(&self) -> Matrix3x2f {
        unsafe {
            let mut mat: Matrix3x2f = mem::uninitialized();
            (*self.get_ptr()).GetTransform((&mut mat) as *mut _ as *mut _);
            mat
        }
    }
}

#[derive(Clone)]
pub struct GenericBrush {
    ptr: ComPtr<ID2D1Brush>,
}

brush_type!(GenericBrush: ID2D1Brush);
