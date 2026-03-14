use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    WbDeviceTag, wb_vacuum_gripper_disable_presence, wb_vacuum_gripper_enable_presence,
    wb_vacuum_gripper_get_presence, wb_vacuum_gripper_get_presence_sampling_period,
    wb_vacuum_gripper_is_on, wb_vacuum_gripper_turn_off, wb_vacuum_gripper_turn_on,
};

#[derive(Debug, Clone, Copy)]
pub struct VacuumGripper {
    tag: WbDeviceTag,
}

impl VacuumGripper {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable_presence(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_vacuum_gripper_enable_presence(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable_presence(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_vacuum_gripper_disable_presence(self.tag))?;
        Ok(())
    }

    pub fn get_presence_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_vacuum_gripper_get_presence_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn status(&self) -> Result<(bool, bool), SimulatorError> {
        let presence = ffi_try!(wb_vacuum_gripper_get_presence(self.tag))? != 0;
        let is_on = ffi_try!(wb_vacuum_gripper_is_on(self.tag))? != 0;
        Ok((presence, is_on))
    }

    pub fn turn_on(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_vacuum_gripper_turn_on(self.tag))?;
        Ok(())
    }

    pub fn turn_off(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_vacuum_gripper_turn_off(self.tag))?;
        Ok(())
    }
}
