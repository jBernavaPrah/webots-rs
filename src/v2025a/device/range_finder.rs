use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    self, wb_range_finder_disable, wb_range_finder_enable, wb_range_finder_get_fov,
    wb_range_finder_get_height, wb_range_finder_get_max_range, wb_range_finder_get_min_range,
    wb_range_finder_get_range_image, wb_range_finder_get_sampling_period,
    wb_range_finder_get_width, wb_range_finder_save_image,
};
use std::ffi::CString;

pub struct RangeFinder(bindings::WbDeviceTag);

impl RangeFinder {
    pub fn new(tag: bindings::WbDeviceTag) -> Self {
        Self(tag)
    }

    pub fn enable(&self, sampling_period: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_range_finder_enable(self.0, sampling_period))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_range_finder_disable(self.0))?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_range_finder_get_sampling_period(self.0))?;
        Ok(period)
    }

    pub fn get_range_image(&self) -> Result<Vec<f32>, SimulatorError> {
        let image = ffi_try!(wb_range_finder_get_range_image(self.0))?;
        let width = ffi_try!(wb_range_finder_get_width(self.0))?;
        let height = ffi_try!(wb_range_finder_get_height(self.0))?;
        let slice = unsafe { std::slice::from_raw_parts(image, (width * height) as usize) };
        Ok(slice.to_vec())
    }

    pub fn get_width(&self) -> Result<i32, SimulatorError> {
        let width = ffi_try!(wb_range_finder_get_width(self.0))?;
        Ok(width)
    }

    pub fn get_height(&self) -> Result<i32, SimulatorError> {
        let height = ffi_try!(wb_range_finder_get_height(self.0))?;
        Ok(height)
    }

    pub fn get_fov(&self) -> Result<f64, SimulatorError> {
        let fov = ffi_try!(wb_range_finder_get_fov(self.0))?;
        Ok(fov)
    }

    pub fn get_min_range(&self) -> Result<f64, SimulatorError> {
        let min_range = ffi_try!(wb_range_finder_get_min_range(self.0))?;
        Ok(min_range)
    }

    pub fn get_max_range(&self) -> Result<f64, SimulatorError> {
        let max_range = ffi_try!(wb_range_finder_get_max_range(self.0))?;
        Ok(max_range)
    }

    pub fn save_image(&self, filename: &str, quality: i32) -> Result<i32, SimulatorError> {
        let filename_cstring = CString::new(filename)?;
        let result = ffi_try!(wb_range_finder_save_image(
            self.0,
            filename_cstring.as_ptr(),
            quality
        ))?;
        Ok(result)
    }
}
