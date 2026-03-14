use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{WbDeviceTag, wb_led_get, wb_led_set};

#[derive(Debug, Clone, Copy)]
pub struct Led {
    tag: WbDeviceTag,
}

impl Led {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn value(&self) -> Result<i32, SimulatorError> {
        let val = ffi_try!(wb_led_get(self.tag))?;
        Ok(val)
    }

    pub fn set(&self, value: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_led_set(self.tag, value))?;
        Ok(())
    }
}
