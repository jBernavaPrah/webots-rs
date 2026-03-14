use crate::v2025a::bindings::{
    wb_light_sensor_disable, wb_light_sensor_enable, wb_light_sensor_get_lookup_table,
    wb_light_sensor_get_lookup_table_size, wb_light_sensor_get_sampling_period,
    wb_light_sensor_get_value, WbDeviceTag,
};
use crate::v2025a::SimulatorError;

#[derive(Debug, Clone, Copy)]
pub struct LightSensor {
    tag: WbDeviceTag,
}

impl LightSensor {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_light_sensor_enable(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_light_sensor_disable(self.tag))?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_light_sensor_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn get_lookup_table(&self) -> Result<Vec<f64>, SimulatorError> {
        let size = ffi_try!(wb_light_sensor_get_lookup_table_size(self.tag))? as usize;
        let ptr = ffi_try!(wb_light_sensor_get_lookup_table(self.tag))?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, size) };
        Ok(slice.to_vec())
    }

    pub fn value(&self) -> Result<f64, SimulatorError> {
        let val = ffi_try!(wb_light_sensor_get_value(self.tag))?;
        Ok(val)
    }
}
