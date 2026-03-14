use crate::v2025a::SimulatorError;
use crate::v2025a::bindings::{
    WbDeviceTag, wb_radar_disable, wb_radar_enable, wb_radar_get_horizontal_fov,
    wb_radar_get_max_range, wb_radar_get_min_range, wb_radar_get_number_of_targets,
    wb_radar_get_sampling_period, wb_radar_get_targets, wb_radar_get_vertical_fov,
};

#[derive(Debug, Clone, Copy)]
pub struct RadarTarget {
    pub distance: f64,
    pub received_power: f64,
    pub speed: f64,
    pub azimuth: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct RadarInfo {
    pub min_range: f64,
    pub max_range: f64,
    pub horizontal_fov: f64,
    pub vertical_fov: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Radar {
    tag: WbDeviceTag,
}

impl Radar {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_radar_enable(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_radar_disable(self.tag))?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_radar_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn info(&self) -> Result<RadarInfo, SimulatorError> {
        let min_range = ffi_try!(wb_radar_get_min_range(self.tag))?;
        let max_range = ffi_try!(wb_radar_get_max_range(self.tag))?;
        let horizontal_fov = ffi_try!(wb_radar_get_horizontal_fov(self.tag))?;
        let vertical_fov = ffi_try!(wb_radar_get_vertical_fov(self.tag))?;
        Ok(RadarInfo {
            min_range,
            max_range,
            horizontal_fov,
            vertical_fov,
        })
    }

    pub fn targets(&self) -> Result<Vec<RadarTarget>, SimulatorError> {
        let count = ffi_try!(wb_radar_get_number_of_targets(self.tag))? as usize;
        let ptr = ffi_try!(wb_radar_get_targets(self.tag))?;
        let slice = ffi_try!(std::slice::from_raw_parts(ptr, count))?;
        let targets = slice
            .iter()
            .map(|t| RadarTarget {
                distance: t.distance,
                received_power: t.received_power,
                speed: t.speed,
                azimuth: t.azimuth,
            })
            .collect::<Vec<_>>();
        Ok(targets)
    }
}
