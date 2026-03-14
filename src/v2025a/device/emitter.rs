use crate::v2025a::bindings::{
    wb_emitter_get_buffer_size, wb_emitter_get_channel, wb_emitter_get_range, wb_emitter_send,
    wb_emitter_set_channel, wb_emitter_set_range, WbDeviceTag,
};
use crate::v2025a::SimulatorError;

#[derive(Debug, Clone, Copy)]
pub struct Emitter {
    tag: WbDeviceTag,
}

impl Emitter {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn set_channel(&self, channel: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_emitter_set_channel(self.tag, channel))?;
        Ok(())
    }

    pub fn get_channel(&self) -> Result<i32, SimulatorError> {
        let channel = ffi_try!(wb_emitter_get_channel(self.tag))?;
        Ok(channel)
    }

    pub fn get_range(&self) -> Result<f64, SimulatorError> {
        let range = ffi_try!(wb_emitter_get_range(self.tag))?;
        Ok(range)
    }

    pub fn set_range(&self, range: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_emitter_set_range(self.tag, range))?;
        Ok(())
    }

    pub fn get_buffer_size(&self) -> Result<i32, SimulatorError> {
        let size = ffi_try!(wb_emitter_get_buffer_size(self.tag))?;
        Ok(size)
    }

    pub fn send(&self, data: &[u8]) -> Result<(), SimulatorError> {
        ffi_try!(wb_emitter_send(
            self.tag,
            data.as_ptr() as *const std::ffi::c_void,
            data.len() as i32,
        ))?;
        Ok(())
    }
}
