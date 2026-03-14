use crate::v2025a::bindings::{
    wb_position_sensor_disable, wb_position_sensor_enable, wb_position_sensor_get_brake,
    wb_position_sensor_get_motor, wb_position_sensor_get_sampling_period,
    wb_position_sensor_get_type, wb_position_sensor_get_value, WbDeviceTag,
};
use crate::v2025a::bindings::{WbJointType_WB_LINEAR, WbJointType_WB_ROTATIONAL};
use crate::v2025a::device::motor::MotorType;
use crate::v2025a::SimulatorError;

#[derive(Debug, Clone, Copy)]
pub struct PositionSensor {
    tag: WbDeviceTag,
}

impl PositionSensor {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_position_sensor_enable(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_position_sensor_disable(self.tag))?;
        Ok(())
    }

    pub fn value(&self) -> Result<f64, SimulatorError> {
        let val = ffi_try!(wb_position_sensor_get_value(self.tag))?;
        Ok(val)
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_position_sensor_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn get_type(&self) -> Result<MotorType, SimulatorError> {
        let motor_type = ffi_try!(wb_position_sensor_get_type(self.tag))?;
        #[allow(non_upper_case_globals)]
        let motor_type = match motor_type {
            WbJointType_WB_ROTATIONAL => MotorType::Rotational,
            WbJointType_WB_LINEAR => MotorType::Linear,
            _ => unreachable!(),
        };
        Ok(motor_type)
    }

    pub fn get_motor(&self) -> Result<WbDeviceTag, SimulatorError> {
        let motor_tag = ffi_try!(wb_position_sensor_get_motor(self.tag))?;
        Ok(motor_tag)
    }

    pub fn get_brake(&self) -> Result<WbDeviceTag, SimulatorError> {
        let brake_tag = ffi_try!(wb_position_sensor_get_brake(self.tag))?;
        Ok(brake_tag)
    }
}
