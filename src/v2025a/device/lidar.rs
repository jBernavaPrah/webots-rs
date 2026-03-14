use crate::v2025a::bindings::{
    wb_lidar_disable, wb_lidar_disable_point_cloud, wb_lidar_enable, wb_lidar_enable_point_cloud,
    wb_lidar_get_fov, wb_lidar_get_frequency, wb_lidar_get_horizontal_resolution,
    wb_lidar_get_layer_point_cloud, wb_lidar_get_layer_range_image, wb_lidar_get_max_frequency,
    wb_lidar_get_max_range, wb_lidar_get_min_frequency, wb_lidar_get_min_range,
    wb_lidar_get_number_of_layers, wb_lidar_get_number_of_points, wb_lidar_get_point_cloud,
    wb_lidar_get_range_image, wb_lidar_get_sampling_period, wb_lidar_get_vertical_fov,
    wb_lidar_is_point_cloud_enabled, wb_lidar_set_frequency, WbDeviceTag, WbLidarPoint,
};
use crate::v2025a::SimulatorError;
use derive_new::new;
use derive_setters::Setters;

/// Data returned by a lidar device
#[derive(Debug, Clone)]
pub enum LidarReading {
    /// Range image data
    RangeImage(Vec<f32>),
    /// Point cloud data
    PointCloud(Vec<WbLidarPoint>),
}

/// Optional configuration for each lidar
#[derive(Debug, Clone, Copy, new, Setters)]
#[setters(prefix = "with_", into, strip_option)]
pub struct LidarConfig {
    #[new(default)]
    pub point_cloud: bool,
    #[new(default)]
    pub frequency: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct Lidar {
    tag: WbDeviceTag,
    pub config: LidarConfig,
}

impl Lidar {
    pub fn new(tag: WbDeviceTag, config: LidarConfig) -> Self {
        Self { tag, config }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_lidar_enable(self.tag, step_ms))?;
        if self.config.point_cloud {
            ffi_try!(wb_lidar_enable_point_cloud(self.tag))?;
        }
        if let Some(freq) = self.config.frequency {
            ffi_try!(wb_lidar_set_frequency(self.tag, freq))?;
        }

        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        if self.config.point_cloud {
            ffi_try!(wb_lidar_disable_point_cloud(self.tag))?;
        }
        ffi_try!(wb_lidar_disable(self.tag))?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_lidar_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn is_point_cloud_enabled(&self) -> Result<bool, SimulatorError> {
        let enabled = ffi_try!(wb_lidar_is_point_cloud_enabled(self.tag))?;
        Ok(enabled != 0)
    }

    pub fn get_layer_range_image(&self, layer: i32) -> Result<Vec<f32>, SimulatorError> {
        let n = ffi_try!(wb_lidar_get_horizontal_resolution(self.tag))? as usize;
        let ptr = ffi_try!(wb_lidar_get_layer_range_image(self.tag, layer))?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, n) };
        Ok(slice.to_vec())
    }

    pub fn get_layer_point_cloud(&self, layer: i32) -> Result<Vec<WbLidarPoint>, SimulatorError> {
        let n = ffi_try!(wb_lidar_get_horizontal_resolution(self.tag))? as usize;
        let ptr = ffi_try!(wb_lidar_get_layer_point_cloud(self.tag, layer))?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, n) };
        Ok(slice.to_vec())
    }

    pub fn get_horizontal_resolution(&self) -> Result<i32, SimulatorError> {
        let resolution = ffi_try!(wb_lidar_get_horizontal_resolution(self.tag))?;
        Ok(resolution)
    }

    pub fn get_number_of_layers(&self) -> Result<i32, SimulatorError> {
        let layers = ffi_try!(wb_lidar_get_number_of_layers(self.tag))?;
        Ok(layers)
    }

    pub fn get_min_frequency(&self) -> Result<f64, SimulatorError> {
        let freq = ffi_try!(wb_lidar_get_min_frequency(self.tag))?;
        Ok(freq)
    }

    pub fn get_max_frequency(&self) -> Result<f64, SimulatorError> {
        let freq = ffi_try!(wb_lidar_get_max_frequency(self.tag))?;
        Ok(freq)
    }

    pub fn get_frequency(&self) -> Result<f64, SimulatorError> {
        let freq = ffi_try!(wb_lidar_get_frequency(self.tag))?;
        Ok(freq)
    }

    pub fn get_fov(&self) -> Result<f64, SimulatorError> {
        let fov = ffi_try!(wb_lidar_get_fov(self.tag))?;
        Ok(fov)
    }

    pub fn get_vertical_fov(&self) -> Result<f64, SimulatorError> {
        let fov = ffi_try!(wb_lidar_get_vertical_fov(self.tag))?;
        Ok(fov)
    }

    pub fn get_min_range(&self) -> Result<f64, SimulatorError> {
        let range = ffi_try!(wb_lidar_get_min_range(self.tag))?;
        Ok(range)
    }

    pub fn get_max_range(&self) -> Result<f64, SimulatorError> {
        let range = ffi_try!(wb_lidar_get_max_range(self.tag))?;
        Ok(range)
    }

    pub fn reading(&self) -> Result<LidarReading, SimulatorError> {
        let n = ffi_try!(wb_lidar_get_number_of_points(self.tag))? as usize;
        if self.config.point_cloud {
            let ptr = ffi_try!(wb_lidar_get_point_cloud(self.tag))?;
            let slice = unsafe { std::slice::from_raw_parts(ptr, n) };
            Ok(LidarReading::PointCloud(slice.to_vec()))
        } else {
            let ptr = ffi_try!(wb_lidar_get_range_image(self.tag))?;
            let slice = unsafe { std::slice::from_raw_parts(ptr, n) };
            Ok(LidarReading::RangeImage(slice.to_vec()))
        }
    }
}
