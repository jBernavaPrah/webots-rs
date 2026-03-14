use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    WbDeviceTag, wb_altimeter_disable, wb_altimeter_enable, wb_altimeter_get_sampling_period,
    wb_altimeter_get_value,
};

#[derive(Debug, Clone, Copy)]
pub struct Altimeter {
    tag: WbDeviceTag,
}

impl Altimeter {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_altimeter_enable(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_altimeter_disable(self.tag))?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_altimeter_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn value(&self) -> Result<f64, SimulatorError> {
        let val = ffi_try!(wb_altimeter_get_value(self.tag))?;
        Ok(val)
    }
}
