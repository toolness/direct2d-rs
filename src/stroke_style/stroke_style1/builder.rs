use crate::enums::{CapStyle, DashStyle, LineJoin, StrokeTransformType};
use crate::error::D2DResult;
use crate::factory::Factory1;
use crate::stroke_style::StrokeStyle1;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d2d1_1::D2D1_STROKE_STYLE_PROPERTIES1;

pub struct StrokeStyleBuilder1<'a> {
    factory: &'a Factory1,
    start_cap: CapStyle,
    end_cap: CapStyle,
    dash_cap: CapStyle,
    line_join: LineJoin,
    miter_limit: f32,
    dash_style: DashStyle,
    dash_offset: f32,
    transform_type: StrokeTransformType,
    dashes: Option<&'a [f32]>,
}

impl<'a> StrokeStyleBuilder1<'a> {
    pub fn new(factory: &'a Factory1) -> Self {
        // default values taken from D2D1::StrokeStyleProperties in d2d1helper.h
        StrokeStyleBuilder1 {
            factory,
            start_cap: CapStyle::Flat,
            end_cap: CapStyle::Flat,
            dash_cap: CapStyle::Flat,
            line_join: LineJoin::Miter,
            miter_limit: 10.0,
            dash_style: DashStyle::Solid,
            dash_offset: 0.0,
            transform_type: StrokeTransformType::Normal,
            dashes: None,
        }
    }

    pub fn build(self) -> D2DResult<StrokeStyle1> {
        unsafe {
            let properties = self.to_d2d1();
            let (dashes, dash_count) = self
                .dashes
                .map(|d| (d.as_ptr(), d.len() as u32))
                .unwrap_or((ptr::null(), 0));

            let mut ptr = ptr::null_mut();
            let hr = (*self.factory.get_raw()).CreateStrokeStyle(
                &properties,
                dashes,
                dash_count,
                &mut ptr,
            );

            if SUCCEEDED(hr) {
                Ok(StrokeStyle1::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn with_start_cap(mut self, start_cap: CapStyle) -> Self {
        self.start_cap = start_cap;
        self
    }

    pub fn with_end_cap(mut self, end_cap: CapStyle) -> Self {
        self.end_cap = end_cap;
        self
    }

    pub fn with_dash_cap(mut self, dash_cap: CapStyle) -> Self {
        self.dash_cap = dash_cap;
        self
    }

    pub fn with_line_join(mut self, line_join: LineJoin) -> Self {
        self.line_join = line_join;
        self
    }

    pub fn with_miter_limit(mut self, miter_limit: f32) -> Self {
        self.miter_limit = miter_limit;
        self
    }

    pub fn with_dash_style(mut self, dash_style: DashStyle) -> Self {
        self.dash_style = dash_style;
        self
    }

    pub fn with_dash_offset(mut self, dash_offset: f32) -> Self {
        self.dash_offset = dash_offset;
        self
    }

    pub fn with_dashes(mut self, dashes: &'a [f32]) -> Self {
        self.dash_style = DashStyle::Custom;
        self.dashes = Some(dashes);
        self
    }

    fn to_d2d1(&self) -> D2D1_STROKE_STYLE_PROPERTIES1 {
        D2D1_STROKE_STYLE_PROPERTIES1 {
            startCap: self.start_cap as u32,
            endCap: self.end_cap as u32,
            dashCap: self.dash_cap as u32,
            lineJoin: self.line_join as u32,
            miterLimit: self.miter_limit,
            dashStyle: self.dash_style as u32,
            dashOffset: self.dash_offset,
            transformType: self.transform_type as u32,
        }
    }
}
