use crate::v2025a::bindings::{
    wb_gps_convert_to_degrees_minutes_seconds, wb_gps_disable, wb_gps_enable,
    wb_gps_get_coordinate_system, wb_gps_get_sampling_period, wb_gps_get_speed,
    wb_gps_get_speed_vector, wb_gps_get_values, WbDeviceTag,
};
use crate::v2025a::bindings::{
    WbGpsCoordinateSystem_WB_GPS_LOCAL_COORDINATE, WbGpsCoordinateSystem_WB_GPS_WGS84_COORDINATE,
};
use crate::v2025a::SimulatorError;
use std::ffi::CStr;

#[derive(Debug, Clone, Copy)]
pub struct GpsData {
    pub position: [f64; 3],
    pub speed: f64,
    pub speed_vector: [f64; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpsCoordinateSystem {
    Local,
    Wgs84,
}

#[derive(Debug, Clone, Copy)]
pub struct Gps {
    tag: WbDeviceTag,
}

impl Gps {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_gps_enable(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_gps_disable(self.tag))?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_gps_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn reading(&self) -> Result<GpsData, SimulatorError> {
        let ptr = ffi_try!(wb_gps_get_values(self.tag))?;
        let vals = unsafe { std::slice::from_raw_parts(ptr, 3) };
        let speed = ffi_try!(wb_gps_get_speed(self.tag))?;
        let sv_ptr = ffi_try!(wb_gps_get_speed_vector(self.tag))?;
        let sv = unsafe { std::slice::from_raw_parts(sv_ptr, 3) };
        Ok(GpsData {
            position: [vals[0], vals[1], vals[2]],
            speed,
            speed_vector: [sv[0], sv[1], sv[2]],
        })
    }

    pub fn get_coordinate_system(&self) -> Result<GpsCoordinateSystem, SimulatorError> {
        let system = ffi_try!(wb_gps_get_coordinate_system(self.tag))?;
        #[allow(non_upper_case_globals)]
        let system = match system {
            WbGpsCoordinateSystem_WB_GPS_LOCAL_COORDINATE => GpsCoordinateSystem::Local,
            WbGpsCoordinateSystem_WB_GPS_WGS84_COORDINATE => GpsCoordinateSystem::Wgs84,
            _ => unreachable!(),
        };
        Ok(system)
    }

    pub fn convert_to_degrees_minutes_seconds(
        decimal_degrees: f64,
    ) -> Result<String, SimulatorError> {
        let ptr = ffi_try!(wb_gps_convert_to_degrees_minutes_seconds(decimal_degrees))?;
        let c_str = unsafe { CStr::from_ptr(ptr) };
        Ok(c_str.to_string_lossy().into_owned())
    }
}
