use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    WbDeviceTag, wb_brake_get_motor, wb_brake_get_position_sensor, wb_brake_get_type,
    wb_brake_set_damping_constant,
};
use crate::v2025a::bindings::{WbJointType_WB_LINEAR, WbJointType_WB_ROTATIONAL};
use crate::v2025a::device::motor::MotorType;

#[derive(Debug, Clone, Copy)]
pub struct Brake {
    tag: WbDeviceTag,
}

impl Brake {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn set_damping(&self, value: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_brake_set_damping_constant(self.tag, value))?;
        Ok(())
    }

    pub fn get_type(&self) -> Result<MotorType, SimulatorError> {
        let motor_type = ffi_try!(wb_brake_get_type(self.tag))?;
        #[allow(non_upper_case_globals)]
        let motor_type = match motor_type {
            WbJointType_WB_ROTATIONAL => MotorType::Rotational,
            WbJointType_WB_LINEAR => MotorType::Linear,
            _ => unreachable!(),
        };
        Ok(motor_type)
    }

    pub fn get_motor(&self) -> Result<WbDeviceTag, SimulatorError> {
        let motor_tag = ffi_try!(wb_brake_get_motor(self.tag))?;
        Ok(motor_tag)
    }

    pub fn get_position_sensor(&self) -> Result<WbDeviceTag, SimulatorError> {
        let sensor_tag = ffi_try!(wb_brake_get_position_sensor(self.tag))?;
        Ok(sensor_tag)
    }
}
