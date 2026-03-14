use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    WbDeviceTag, wb_connector_disable_presence, wb_connector_enable_presence,
    wb_connector_get_presence, wb_connector_get_presence_sampling_period, wb_connector_is_locked,
    wb_connector_lock, wb_connector_unlock,
};

#[derive(Debug, Clone, Copy)]
pub struct Connector {
    tag: WbDeviceTag,
}

impl Connector {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable_presence(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_connector_enable_presence(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable_presence(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_connector_disable_presence(self.tag))?;
        Ok(())
    }

    pub fn get_presence_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_connector_get_presence_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn presence(&self) -> Result<(i32, bool), SimulatorError> {
        let presence = ffi_try!(wb_connector_get_presence(self.tag))?;
        let locked = ffi_try!(wb_connector_is_locked(self.tag))? != 0;
        Ok((presence, locked))
    }

    pub fn lock(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_connector_lock(self.tag))?;
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_connector_unlock(self.tag))?;
        Ok(())
    }
}
