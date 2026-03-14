use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    wb_robot_battery_sensor_disable, wb_robot_battery_sensor_enable,
    wb_robot_battery_sensor_get_sampling_period, wb_robot_battery_sensor_get_value,
};

#[derive(Debug, Clone, Copy)]
pub struct BatterySensor;

impl BatterySensor {
    pub fn new() -> Self {
        Self
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_robot_battery_sensor_enable(step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_robot_battery_sensor_disable())?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_robot_battery_sensor_get_sampling_period())?;
        Ok(period)
    }

    pub fn get_value(&self) -> Result<f64, SimulatorError> {
        let value = ffi_try!(wb_robot_battery_sensor_get_value())?;
        Ok(value)
    }
}

impl Default for BatterySensor {
    fn default() -> Self {
        Self::new()
    }
}
