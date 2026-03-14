use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    WbDeviceTag, wb_touch_sensor_disable, wb_touch_sensor_enable, wb_touch_sensor_get_lookup_table,
    wb_touch_sensor_get_lookup_table_size, wb_touch_sensor_get_sampling_period,
    wb_touch_sensor_get_type, wb_touch_sensor_get_value, wb_touch_sensor_get_values,
};
use crate::v2025a::bindings::{
    WbTouchSensorType_WB_TOUCH_SENSOR_BUMPER, WbTouchSensorType_WB_TOUCH_SENSOR_FORCE,
    WbTouchSensorType_WB_TOUCH_SENSOR_FORCE3D,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchSensorType {
    Bumper,
    Force,
    Force3D,
}

#[derive(Debug, Clone, Copy)]
pub struct TouchSensor {
    tag: WbDeviceTag,
}

impl TouchSensor {
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

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_touch_sensor_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn get_lookup_table(&self) -> Result<Vec<f64>, SimulatorError> {
        let size = ffi_try!(wb_touch_sensor_get_lookup_table_size(self.tag))? as usize;
        let ptr = ffi_try!(wb_touch_sensor_get_lookup_table(self.tag))?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, size) };
        Ok(slice.to_vec())
    }

    pub fn get_value(&self) -> Result<f64, SimulatorError> {
        let val = ffi_try!(wb_touch_sensor_get_value(self.tag))?;
        Ok(val)
    }

    pub fn get_values(&self) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(wb_touch_sensor_get_values(self.tag))?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, 3) };
        Ok([slice[0], slice[1], slice[2]])
    }

    pub fn get_type(&self) -> Result<TouchSensorType, SimulatorError> {
        let sensor_type = ffi_try!(wb_touch_sensor_get_type(self.tag))?;
        #[allow(non_upper_case_globals)]
        let sensor_type = match sensor_type {
            WbTouchSensorType_WB_TOUCH_SENSOR_BUMPER => TouchSensorType::Bumper,
            WbTouchSensorType_WB_TOUCH_SENSOR_FORCE => TouchSensorType::Force,
            WbTouchSensorType_WB_TOUCH_SENSOR_FORCE3D => TouchSensorType::Force3D,
            _ => unreachable!(),
        };
        Ok(sensor_type)
    }
}
