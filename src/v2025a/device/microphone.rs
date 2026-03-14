use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    self, wb_microphone_disable, wb_microphone_enable, wb_microphone_get_sample_data,
    wb_microphone_get_sample_size, wb_microphone_get_sampling_period,
};

#[derive(Debug, Clone, Copy)]
pub struct Microphone(bindings::WbDeviceTag);

impl Microphone {
    pub fn new(tag: bindings::WbDeviceTag) -> Self {
        Self(tag)
    }

    pub fn enable(&self, sampling_period: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_microphone_enable(self.0, sampling_period))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_microphone_disable(self.0))?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_microphone_get_sampling_period(self.0))?;
        Ok(period)
    }

    pub fn get_sample_data(&self) -> Result<Vec<u8>, SimulatorError> {
        let data = ffi_try!(wb_microphone_get_sample_data(self.0))?;
        let size = ffi_try!(wb_microphone_get_sample_size(self.0))?;
        let slice = unsafe { std::slice::from_raw_parts(data as *const u8, size as usize) };
        Ok(slice.to_vec())
    }

    pub fn get_sample_size(&self) -> Result<i32, SimulatorError> {
        let size = ffi_try!(wb_microphone_get_sample_size(self.0))?;
        Ok(size)
    }
}
