use crate::v2025a::bindings::{
    self, wb_speaker_get_engine, wb_speaker_get_language, wb_speaker_is_sound_playing,
    wb_speaker_is_speaking, wb_speaker_play_sound, wb_speaker_set_engine, wb_speaker_set_language,
    wb_speaker_speak, wb_speaker_stop,
};
use crate::v2025a::SimulatorError;
use std::ffi::{CStr, CString};

#[derive(Debug, Clone, Copy)]
pub struct Speaker(bindings::WbDeviceTag);

impl Speaker {
    pub fn new(tag: bindings::WbDeviceTag) -> Self {
        Self(tag)
    }

    pub fn play_sound(
        left: &Speaker,
        right: &Speaker,
        sound: &str,
        volume: f64,
        pitch: f64,
        balance: f64,
        loop_: bool,
    ) -> Result<(), SimulatorError> {
        let sound_cstring = CString::new(sound)?;
        ffi_try!(wb_speaker_play_sound(
            left.0,
            right.0,
            sound_cstring.as_ptr(),
            volume,
            pitch,
            balance,
            if loop_ { 1 } else { 0 },
        ))?;
        Ok(())
    }

    pub fn stop(&self, sound: &str) -> Result<(), SimulatorError> {
        let sound_cstring = CString::new(sound)?;
        ffi_try!(wb_speaker_stop(self.0, sound_cstring.as_ptr()))?;
        Ok(())
    }

    pub fn is_sound_playing(&self, sound: &str) -> Result<bool, SimulatorError> {
        let sound_cstring = CString::new(sound)?;
        let result = ffi_try!(wb_speaker_is_sound_playing(self.0, sound_cstring.as_ptr()))?;
        Ok(result != 0)
    }

    pub fn set_engine(&self, engine: &str) -> Result<bool, SimulatorError> {
        let engine_cstring = CString::new(engine)?;
        let result = ffi_try!(wb_speaker_set_engine(self.0, engine_cstring.as_ptr()))?;
        Ok(result != 0)
    }

    pub fn set_language(&self, language: &str) -> Result<bool, SimulatorError> {
        let language_cstring = CString::new(language)?;
        let result = ffi_try!(wb_speaker_set_language(self.0, language_cstring.as_ptr()))?;
        Ok(result != 0)
    }

    pub fn get_engine(&self) -> Result<String, SimulatorError> {
        let engine_ptr = ffi_try!(wb_speaker_get_engine(self.0))?;
        let engine = unsafe { CStr::from_ptr(engine_ptr).to_string_lossy().into_owned() };
        Ok(engine)
    }

    pub fn get_language(&self) -> Result<String, SimulatorError> {
        let language_ptr = ffi_try!(wb_speaker_get_language(self.0))?;
        let language = unsafe { CStr::from_ptr(language_ptr).to_string_lossy().into_owned() };
        Ok(language)
    }

    pub fn speak(&self, text: &str, volume: f64) -> Result<(), SimulatorError> {
        let text_cstring = CString::new(text)?;
        ffi_try!(wb_speaker_speak(self.0, text_cstring.as_ptr(), volume))?;
        Ok(())
    }

    pub fn is_speaking(&self) -> Result<bool, SimulatorError> {
        let result = ffi_try!(wb_speaker_is_speaking(self.0))?;
        Ok(result != 0)
    }
}
