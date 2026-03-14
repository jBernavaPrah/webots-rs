use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    WbDeviceTag, wb_inertial_unit_disable, wb_inertial_unit_enable, wb_inertial_unit_get_noise,
    wb_inertial_unit_get_quaternion, wb_inertial_unit_get_roll_pitch_yaw,
    wb_inertial_unit_get_sampling_period,
};

#[derive(Debug, Clone, Copy)]
pub struct InertialUnit {
    tag: WbDeviceTag,
}

impl InertialUnit {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_inertial_unit_enable(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_inertial_unit_disable(self.tag))?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_inertial_unit_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn get_noise(&self) -> Result<f64, SimulatorError> {
        let noise = ffi_try!(wb_inertial_unit_get_noise(self.tag))?;
        Ok(noise)
    }

    pub fn get_roll_pitch_yaw(&self) -> Result<[f64; 3], SimulatorError> {
        let rpy_ptr = ffi_try!(wb_inertial_unit_get_roll_pitch_yaw(self.tag))?;
        let rpy_slice = unsafe { std::slice::from_raw_parts(rpy_ptr, 3) };
        Ok([rpy_slice[0], rpy_slice[1], rpy_slice[2]])
    }

    pub fn get_quaternion(&self) -> Result<[f64; 4], SimulatorError> {
        let quat_ptr = ffi_try!(wb_inertial_unit_get_quaternion(self.tag))?;
        let quat_slice = unsafe { std::slice::from_raw_parts(quat_ptr, 4) };
        Ok([quat_slice[0], quat_slice[1], quat_slice[2], quat_slice[3]])
    }
}
