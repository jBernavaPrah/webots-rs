use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    WbDeviceTag, wb_gyro_disable, wb_gyro_enable, wb_gyro_get_lookup_table,
    wb_gyro_get_lookup_table_size, wb_gyro_get_sampling_period, wb_gyro_get_values,
};

#[derive(Debug, Clone, Copy)]
pub struct Gyro {
    tag: WbDeviceTag,
}

impl Gyro {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_gyro_enable(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_gyro_disable(self.tag))?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_gyro_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn get_lookup_table(&self) -> Result<Vec<f64>, SimulatorError> {
        let size = ffi_try!(wb_gyro_get_lookup_table_size(self.tag))? as usize;
        let ptr = ffi_try!(wb_gyro_get_lookup_table(self.tag))?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, size) };
        Ok(slice.to_vec())
    }

    pub fn values(&self) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(wb_gyro_get_values(self.tag))?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, 3) };
        Ok([slice[0], slice[1], slice[2]])
    }
}
