use crate::v2025a::bindings::{
    self, wb_radio_disable, wb_radio_enable, wb_radio_get_address, wb_radio_get_bitrate,
    wb_radio_get_channel, wb_radio_get_frequency, wb_radio_get_rx_sensitivity,
    wb_radio_get_tx_power, wb_radio_message_delete, wb_radio_message_new, wb_radio_send,
    wb_radio_set_address, wb_radio_set_bitrate, wb_radio_set_callback, wb_radio_set_channel,
    wb_radio_set_frequency, wb_radio_set_rx_sensitivity, wb_radio_set_tx_power,
};
use crate::v2025a::SimulatorError;
use std::ffi::{CStr, CString};

pub struct Radio(bindings::WbDeviceTag);

impl Radio {
    pub fn new(tag: bindings::WbDeviceTag) -> Self {
        Self(tag)
    }

    pub fn enable(&self, sampling_period: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_radio_enable(self.0, sampling_period))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_radio_disable(self.0))?;
        Ok(())
    }

    pub fn set_address(&self, address: &str) -> Result<(), SimulatorError> {
        let address_cstring = CString::new(address)?;
        ffi_try!(wb_radio_set_address(self.0, address_cstring.as_ptr()))?;
        Ok(())
    }

    pub fn get_address(&self) -> Result<String, SimulatorError> {
        let address_ptr = ffi_try!(wb_radio_get_address(self.0))?;
        let address = unsafe { CStr::from_ptr(address_ptr).to_string_lossy().into_owned() };
        Ok(address)
    }

    pub fn set_frequency(&self, frequency: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_radio_set_frequency(self.0, frequency))?;
        Ok(())
    }

    pub fn get_frequency(&self) -> Result<f64, SimulatorError> {
        let frequency = ffi_try!(wb_radio_get_frequency(self.0))?;
        Ok(frequency)
    }

    pub fn set_channel(&self, channel: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_radio_set_channel(self.0, channel))?;
        Ok(())
    }

    pub fn get_channel(&self) -> Result<i32, SimulatorError> {
        let channel = ffi_try!(wb_radio_get_channel(self.0))?;
        Ok(channel)
    }

    pub fn set_bitrate(&self, bitrate: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_radio_set_bitrate(self.0, bitrate))?;
        Ok(())
    }

    pub fn get_bitrate(&self) -> Result<i32, SimulatorError> {
        let bitrate = ffi_try!(wb_radio_get_bitrate(self.0))?;
        Ok(bitrate)
    }

    pub fn set_rx_sensitivity(&self, sensitivity: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_radio_set_rx_sensitivity(self.0, sensitivity))?;
        Ok(())
    }

    pub fn get_rx_sensitivity(&self) -> Result<f64, SimulatorError> {
        let sensitivity = ffi_try!(wb_radio_get_rx_sensitivity(self.0))?;
        Ok(sensitivity)
    }

    pub fn set_tx_power(&self, power: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_radio_set_tx_power(self.0, power))?;
        Ok(())
    }

    pub fn get_tx_power(&self) -> Result<f64, SimulatorError> {
        let power = ffi_try!(wb_radio_get_tx_power(self.0))?;
        Ok(power)
    }

    pub fn set_callback(
        &self,
        callback: Option<unsafe extern "C" fn(bindings::WbRadioEvent)>,
    ) -> Result<(), SimulatorError> {
        ffi_try!(wb_radio_set_callback(self.0, callback))?;
        Ok(())
    }

    pub fn send(
        &self,
        destination: &str,
        message: &[u8],
        delay: f64,
    ) -> Result<(), SimulatorError> {
        let destination_cstring = CString::new(destination)?;
        let msg = ffi_try!(wb_radio_message_new(
            message.len() as i32,
            message.as_ptr() as *const i8,
            destination_cstring.as_ptr()
        ))?;
        ffi_try!(wb_radio_send(self.0, msg, delay))?;
        ffi_try!(wb_radio_message_delete(msg))?;
        Ok(())
    }
}
