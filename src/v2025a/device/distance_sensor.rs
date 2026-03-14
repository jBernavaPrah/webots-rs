use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    WbDeviceTag, wb_distance_sensor_disable, wb_distance_sensor_enable,
    wb_distance_sensor_get_aperture, wb_distance_sensor_get_lookup_table,
    wb_distance_sensor_get_lookup_table_size, wb_distance_sensor_get_max_value,
    wb_distance_sensor_get_min_value, wb_distance_sensor_get_sampling_period,
    wb_distance_sensor_get_type, wb_distance_sensor_get_value,
};
use crate::v2025a::bindings::{
    WbDistanceSensorType_WB_DISTANCE_SENSOR_GENERIC,
    WbDistanceSensorType_WB_DISTANCE_SENSOR_INFRA_RED,
    WbDistanceSensorType_WB_DISTANCE_SENSOR_LASER, WbDistanceSensorType_WB_DISTANCE_SENSOR_SONAR,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceSensorType {
    Generic,
    InfraRed,
    Sonar,
    Laser,
}

#[derive(Debug, Clone, Copy)]
pub struct DistanceSensor {
    tag: WbDeviceTag,
}

impl DistanceSensor {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_distance_sensor_enable(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_distance_sensor_disable(self.tag))?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_distance_sensor_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn value(&self) -> Result<f64, SimulatorError> {
        let val = ffi_try!(wb_distance_sensor_get_value(self.tag))?;
        Ok(val)
    }

    pub fn get_max_value(&self) -> Result<f64, SimulatorError> {
        let val = ffi_try!(wb_distance_sensor_get_max_value(self.tag))?;
        Ok(val)
    }

    pub fn get_min_value(&self) -> Result<f64, SimulatorError> {
        let val = ffi_try!(wb_distance_sensor_get_min_value(self.tag))?;
        Ok(val)
    }

    pub fn get_aperture(&self) -> Result<f64, SimulatorError> {
        let val = ffi_try!(wb_distance_sensor_get_aperture(self.tag))?;
        Ok(val)
    }

    pub fn get_lookup_table(&self) -> Result<Vec<f64>, SimulatorError> {
        let size = ffi_try!(wb_distance_sensor_get_lookup_table_size(self.tag))? as usize;
        let ptr = ffi_try!(wb_distance_sensor_get_lookup_table(self.tag))?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, size) };
        Ok(slice.to_vec())
    }

    pub fn get_type(&self) -> Result<DistanceSensorType, SimulatorError> {
        let sensor_type = ffi_try!(wb_distance_sensor_get_type(self.tag))?;
        #[allow(non_upper_case_globals)]
        let sensor_type = match sensor_type {
            WbDistanceSensorType_WB_DISTANCE_SENSOR_GENERIC => DistanceSensorType::Generic,
            WbDistanceSensorType_WB_DISTANCE_SENSOR_INFRA_RED => DistanceSensorType::InfraRed,
            WbDistanceSensorType_WB_DISTANCE_SENSOR_SONAR => DistanceSensorType::Sonar,
            WbDistanceSensorType_WB_DISTANCE_SENSOR_LASER => DistanceSensorType::Laser,
            _ => unreachable!(),
        };
        Ok(sensor_type)
    }
}
