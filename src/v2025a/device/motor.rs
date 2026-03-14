use crate::v2025a::bindings::{
    wb_motor_disable_force_feedback, wb_motor_disable_torque_feedback,
    wb_motor_enable_force_feedback, wb_motor_enable_torque_feedback, wb_motor_get_acceleration,
    wb_motor_get_available_force, wb_motor_get_available_torque, wb_motor_get_brake,
    wb_motor_get_force_feedback, wb_motor_get_force_feedback_sampling_period,
    wb_motor_get_max_force, wb_motor_get_max_position, wb_motor_get_max_torque,
    wb_motor_get_max_velocity, wb_motor_get_min_position, wb_motor_get_multiplier,
    wb_motor_get_position_sensor, wb_motor_get_target_position, wb_motor_get_torque_feedback,
    wb_motor_get_torque_feedback_sampling_period, wb_motor_get_type, wb_motor_get_velocity,
    wb_motor_set_acceleration, wb_motor_set_available_force, wb_motor_set_available_torque,
    wb_motor_set_control_pid, wb_motor_set_force, wb_motor_set_position, wb_motor_set_torque,
    wb_motor_set_velocity, WbDeviceTag,
};
use crate::v2025a::bindings::{WbJointType_WB_LINEAR, WbJointType_WB_ROTATIONAL};
use crate::v2025a::SimulatorError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorType {
    Rotational,
    Linear,
}

#[derive(Debug, Clone, Copy)]
pub struct Motor {
    tag: WbDeviceTag,
}

impl Motor {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable_velocity_control(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_set_position(self.tag, f64::INFINITY))?;
        ffi_try!(wb_motor_set_velocity(self.tag, 0.0))?;
        Ok(())
    }

    pub fn enable_torque_feedback(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_enable_torque_feedback(self.tag, step_ms))?;
        Ok(())
    }

    pub fn get_torque_feedback(&self) -> Result<f64, SimulatorError> {
        let torque = ffi_try!(wb_motor_get_torque_feedback(self.tag))?;
        Ok(torque)
    }

    pub fn set_velocity(&self, velocity: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_set_velocity(self.tag, velocity))?;
        Ok(())
    }

    pub fn set_position(&self, position: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_set_position(self.tag, position))?;
        Ok(())
    }

    pub fn set_acceleration(&self, acceleration: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_set_acceleration(self.tag, acceleration))?;
        Ok(())
    }

    pub fn set_force(&self, force: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_set_force(self.tag, force))?;
        Ok(())
    }

    pub fn set_torque(&self, torque: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_set_torque(self.tag, torque))?;
        Ok(())
    }

    pub fn set_available_force(&self, force: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_set_available_force(self.tag, force))?;
        Ok(())
    }

    pub fn set_available_torque(&self, torque: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_set_available_torque(self.tag, torque))?;
        Ok(())
    }

    pub fn set_control_pid(&self, p: f64, i: f64, d: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_set_control_pid(self.tag, p, i, d))?;
        Ok(())
    }

    pub fn enable_force_feedback(&self, sampling_period: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_enable_force_feedback(self.tag, sampling_period))?;
        Ok(())
    }

    pub fn disable_force_feedback(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_disable_force_feedback(self.tag))?;
        Ok(())
    }

    pub fn get_force_feedback_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_motor_get_force_feedback_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn get_force_feedback(&self) -> Result<f64, SimulatorError> {
        let force = ffi_try!(wb_motor_get_force_feedback(self.tag))?;
        Ok(force)
    }

    pub fn disable_torque_feedback(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_motor_disable_torque_feedback(self.tag))?;
        Ok(())
    }

    pub fn get_torque_feedback_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_motor_get_torque_feedback_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn get_type(&self) -> Result<MotorType, SimulatorError> {
        let motor_type = ffi_try!(wb_motor_get_type(self.tag))?;
        #[allow(non_upper_case_globals)]
        let motor_type = match motor_type {
            WbJointType_WB_ROTATIONAL => MotorType::Rotational,
            WbJointType_WB_LINEAR => MotorType::Linear,
            _ => unreachable!(),
        };
        Ok(motor_type)
    }

    pub fn get_target_position(&self) -> Result<f64, SimulatorError> {
        let position = ffi_try!(wb_motor_get_target_position(self.tag))?;
        Ok(position)
    }

    pub fn get_min_position(&self) -> Result<f64, SimulatorError> {
        let position = ffi_try!(wb_motor_get_min_position(self.tag))?;
        Ok(position)
    }

    pub fn get_max_position(&self) -> Result<f64, SimulatorError> {
        let position = ffi_try!(wb_motor_get_max_position(self.tag))?;
        Ok(position)
    }

    pub fn get_velocity(&self) -> Result<f64, SimulatorError> {
        let velocity = ffi_try!(wb_motor_get_velocity(self.tag))?;
        Ok(velocity)
    }

    pub fn get_max_velocity(&self) -> Result<f64, SimulatorError> {
        let velocity = ffi_try!(wb_motor_get_max_velocity(self.tag))?;
        Ok(velocity)
    }

    pub fn get_acceleration(&self) -> Result<f64, SimulatorError> {
        let acceleration = ffi_try!(wb_motor_get_acceleration(self.tag))?;
        Ok(acceleration)
    }

    pub fn get_available_force(&self) -> Result<f64, SimulatorError> {
        let force = ffi_try!(wb_motor_get_available_force(self.tag))?;
        Ok(force)
    }

    pub fn get_max_force(&self) -> Result<f64, SimulatorError> {
        let force = ffi_try!(wb_motor_get_max_force(self.tag))?;
        Ok(force)
    }

    pub fn get_available_torque(&self) -> Result<f64, SimulatorError> {
        let torque = ffi_try!(wb_motor_get_available_torque(self.tag))?;
        Ok(torque)
    }

    pub fn get_max_torque(&self) -> Result<f64, SimulatorError> {
        let torque = ffi_try!(wb_motor_get_max_torque(self.tag))?;
        Ok(torque)
    }

    pub fn get_multiplier(&self) -> Result<f64, SimulatorError> {
        let multiplier = ffi_try!(wb_motor_get_multiplier(self.tag))?;
        Ok(multiplier)
    }

    pub fn get_brake(&self) -> Result<WbDeviceTag, SimulatorError> {
        let brake_tag = ffi_try!(wb_motor_get_brake(self.tag))?;
        Ok(brake_tag)
    }

    pub fn get_position_sensor(&self) -> Result<WbDeviceTag, SimulatorError> {
        let sensor_tag = ffi_try!(wb_motor_get_position_sensor(self.tag))?;
        Ok(sensor_tag)
    }
}
