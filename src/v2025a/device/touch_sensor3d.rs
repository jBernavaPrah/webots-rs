use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    WbDeviceTag, wb_touch_sensor_disable, wb_touch_sensor_enable, wb_touch_sensor_get_values,
};

#[derive(Debug, Clone, Copy)]
pub struct TouchSensor3D {
    tag: WbDeviceTag,
}

impl TouchSensor3D {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_touch_sensor_enable(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_touch_sensor_disable(self.tag))?;
        Ok(())
    }

    pub fn values(&self) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(wb_touch_sensor_get_values(self.tag))?;
        let slice = ffi_try!(std::slice::from_raw_parts(ptr, 3))?;
        Ok([slice[0], slice[1], slice[2]])
    }
}
