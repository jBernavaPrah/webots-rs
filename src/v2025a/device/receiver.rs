use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    WbDeviceTag, wb_receiver_disable, wb_receiver_enable, wb_receiver_get_channel,
    wb_receiver_get_data, wb_receiver_get_data_size, wb_receiver_get_emitter_direction,
    wb_receiver_get_queue_length, wb_receiver_get_sampling_period, wb_receiver_get_signal_strength,
    wb_receiver_next_packet, wb_receiver_set_channel,
};

#[derive(Debug, Clone)]
pub struct ReceiverMessage {
    pub data: Vec<u8>,
    pub signal_strength: f64,
    pub direction: [f64; 3],
}

#[derive(Debug, Clone, Copy)]
pub struct Receiver {
    tag: WbDeviceTag,
}

impl Receiver {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_receiver_enable(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_receiver_disable(self.tag))?;
        Ok(())
    }

    pub fn set_channel(&self, channel: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_receiver_set_channel(self.tag, channel))?;
        Ok(())
    }

    pub fn get_channel(&self) -> Result<i32, SimulatorError> {
        let channel = ffi_try!(wb_receiver_get_channel(self.tag))?;
        Ok(channel)
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_receiver_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn receive(&self) -> Result<Vec<ReceiverMessage>, SimulatorError> {
        let mut messages = Vec::new();
        let queue_length = ffi_try!(wb_receiver_get_queue_length(self.tag))? as usize;
        for _ in 0..queue_length {
            let size = ffi_try!(wb_receiver_get_data_size(self.tag))? as usize;
            let ptr = ffi_try!(wb_receiver_get_data(self.tag))? as *const u8;
            let data = ffi_try!(std::slice::from_raw_parts(ptr, size))?;
            let strength = ffi_try!(wb_receiver_get_signal_strength(self.tag))?;
            let direction_ptr = ffi_try!(wb_receiver_get_emitter_direction(self.tag))?;
            let direction_slice = ffi_try!(std::slice::from_raw_parts(direction_ptr, 3))?;
            messages.push(ReceiverMessage {
                data: data.to_vec(),
                signal_strength: strength,
                direction: [direction_slice[0], direction_slice[1], direction_slice[2]],
            });
            ffi_try!(wb_receiver_next_packet(self.tag))?;
        }

        Ok(messages)
    }
}
