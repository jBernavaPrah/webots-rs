use crate::v2025a::bindings::{
    wb_camera_disable, wb_camera_enable, wb_camera_get_exposure, wb_camera_get_focal_distance,
    wb_camera_get_focal_length, wb_camera_get_fov, wb_camera_get_height, wb_camera_get_image,
    wb_camera_get_max_focal_distance, wb_camera_get_max_fov, wb_camera_get_min_focal_distance,
    wb_camera_get_min_fov, wb_camera_get_near, wb_camera_get_sampling_period, wb_camera_get_width,
    wb_camera_has_recognition, wb_camera_recognition_disable,
    wb_camera_recognition_disable_segmentation, wb_camera_recognition_enable,
    wb_camera_recognition_enable_segmentation, wb_camera_recognition_get_number_of_objects,
    wb_camera_recognition_get_objects, wb_camera_recognition_get_sampling_period,
    wb_camera_recognition_get_segmentation_image, wb_camera_recognition_has_segmentation,
    wb_camera_recognition_is_segmentation_enabled, wb_camera_recognition_save_segmentation_image,
    wb_camera_save_image, wb_camera_set_exposure, wb_camera_set_focal_distance, wb_camera_set_fov,
    WbCameraRecognitionObject, WbDeviceTag,
};
use crate::v2025a::SimulatorError;
use std::ffi::CString;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    tag: WbDeviceTag,
}

#[derive(Debug, Clone)]
pub struct RecognitionObject {
    pub id: i32,
    pub position: [f64; 3],
    pub orientation: [f64; 4],
    pub size: [f64; 2],
    pub position_on_image: [i32; 2],
    pub size_on_image: [i32; 2],
    pub number_of_colors: i32,
    pub colors: Vec<f64>,
    pub model: String,
}

impl Camera {
    pub fn new(tag: WbDeviceTag) -> Self {
        Self { tag }
    }

    pub fn enable(&self, step_ms: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_camera_enable(self.tag, step_ms))?;
        Ok(())
    }

    pub fn disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_camera_disable(self.tag))?;
        Ok(())
    }

    pub fn get_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_camera_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn get_image(&self) -> Result<Vec<u8>, SimulatorError> {
        let width = ffi_try!(wb_camera_get_width(self.tag))? as usize;
        let height = ffi_try!(wb_camera_get_height(self.tag))? as usize;
        let ptr = ffi_try!(wb_camera_get_image(self.tag))?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, width * height * 4) };
        Ok(slice.to_vec())
    }

    pub fn get_width(&self) -> Result<i32, SimulatorError> {
        let width = ffi_try!(wb_camera_get_width(self.tag))?;
        Ok(width)
    }

    pub fn get_height(&self) -> Result<i32, SimulatorError> {
        let height = ffi_try!(wb_camera_get_height(self.tag))?;
        Ok(height)
    }

    pub fn get_fov(&self) -> Result<f64, SimulatorError> {
        let fov = ffi_try!(wb_camera_get_fov(self.tag))?;
        Ok(fov)
    }

    pub fn get_max_fov(&self) -> Result<f64, SimulatorError> {
        let fov = ffi_try!(wb_camera_get_max_fov(self.tag))?;
        Ok(fov)
    }

    pub fn get_min_fov(&self) -> Result<f64, SimulatorError> {
        let fov = ffi_try!(wb_camera_get_min_fov(self.tag))?;
        Ok(fov)
    }

    pub fn set_fov(&self, fov: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_camera_set_fov(self.tag, fov))?;
        Ok(())
    }

    pub fn get_exposure(&self) -> Result<f64, SimulatorError> {
        let exposure = ffi_try!(wb_camera_get_exposure(self.tag))?;
        Ok(exposure)
    }

    pub fn set_exposure(&self, exposure: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_camera_set_exposure(self.tag, exposure))?;
        Ok(())
    }

    pub fn get_focal_length(&self) -> Result<f64, SimulatorError> {
        let focal_length = ffi_try!(wb_camera_get_focal_length(self.tag))?;
        Ok(focal_length)
    }

    pub fn get_focal_distance(&self) -> Result<f64, SimulatorError> {
        let focal_distance = ffi_try!(wb_camera_get_focal_distance(self.tag))?;
        Ok(focal_distance)
    }

    pub fn get_max_focal_distance(&self) -> Result<f64, SimulatorError> {
        let focal_distance = ffi_try!(wb_camera_get_max_focal_distance(self.tag))?;
        Ok(focal_distance)
    }

    pub fn get_min_focal_distance(&self) -> Result<f64, SimulatorError> {
        let focal_distance = ffi_try!(wb_camera_get_min_focal_distance(self.tag))?;
        Ok(focal_distance)
    }

    pub fn set_focal_distance(&self, focal_distance: f64) -> Result<(), SimulatorError> {
        ffi_try!(wb_camera_set_focal_distance(self.tag, focal_distance))?;
        Ok(())
    }

    pub fn get_near(&self) -> Result<f64, SimulatorError> {
        let near = ffi_try!(wb_camera_get_near(self.tag))?;
        Ok(near)
    }

    pub fn save_image(&self, filename: &str, quality: i32) -> Result<(), SimulatorError> {
        let c_filename = CString::new(filename)?;
        ffi_try!(wb_camera_save_image(self.tag, c_filename.as_ptr(), quality))?;
        Ok(())
    }

    pub fn has_recognition(&self) -> Result<bool, SimulatorError> {
        let result = ffi_try!(wb_camera_has_recognition(self.tag))?;
        Ok(result != 0)
    }

    pub fn recognition_enable(&self, sampling_period: i32) -> Result<(), SimulatorError> {
        ffi_try!(wb_camera_recognition_enable(self.tag, sampling_period))?;
        Ok(())
    }

    pub fn recognition_disable(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_camera_recognition_disable(self.tag))?;
        Ok(())
    }

    pub fn get_recognition_sampling_period(&self) -> Result<i32, SimulatorError> {
        let period = ffi_try!(wb_camera_recognition_get_sampling_period(self.tag))?;
        Ok(period)
    }

    pub fn get_recognition_number_of_objects(&self) -> Result<i32, SimulatorError> {
        let num = ffi_try!(wb_camera_recognition_get_number_of_objects(self.tag))?;
        Ok(num)
    }

    pub fn get_recognition_objects(&self) -> Result<Vec<RecognitionObject>, SimulatorError> {
        let num = self.get_recognition_number_of_objects()? as usize;
        let ptr = ffi_try!(wb_camera_recognition_get_objects(self.tag))?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, num) };
        let objects = slice.iter().map(from_native).collect();
        Ok(objects)
    }

    pub fn recognition_has_segmentation(&self) -> Result<bool, SimulatorError> {
        let result = ffi_try!(wb_camera_recognition_has_segmentation(self.tag))?;
        Ok(result != 0)
    }

    pub fn recognition_enable_segmentation(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_camera_recognition_enable_segmentation(self.tag))?;
        Ok(())
    }

    pub fn recognition_disable_segmentation(&self) -> Result<(), SimulatorError> {
        ffi_try!(wb_camera_recognition_disable_segmentation(self.tag))?;
        Ok(())
    }

    pub fn recognition_is_segmentation_enabled(&self) -> Result<bool, SimulatorError> {
        let result = ffi_try!(wb_camera_recognition_is_segmentation_enabled(self.tag))?;
        Ok(result != 0)
    }

    pub fn get_recognition_segmentation_image(&self) -> Result<Vec<u8>, SimulatorError> {
        let width = ffi_try!(wb_camera_get_width(self.tag))? as usize;
        let height = ffi_try!(wb_camera_get_height(self.tag))? as usize;
        let ptr = ffi_try!(wb_camera_recognition_get_segmentation_image(self.tag))?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, width * height * 4) };
        Ok(slice.to_vec())
    }

    pub fn save_recognition_segmentation_image(
        &self,
        filename: &str,
        quality: i32,
    ) -> Result<(), SimulatorError> {
        let c_filename = CString::new(filename)?;
        ffi_try!(wb_camera_recognition_save_segmentation_image(
            self.tag,
            c_filename.as_ptr(),
            quality
        ))?;
        Ok(())
    }
}

fn from_native(native: &WbCameraRecognitionObject) -> RecognitionObject {
    let colors_slice =
        unsafe { std::slice::from_raw_parts(native.colors, native.number_of_colors as usize * 3) };
    let model_str = unsafe {
        std::ffi::CStr::from_ptr(native.model)
            .to_string_lossy()
            .into_owned()
    };
    RecognitionObject {
        id: native.id,
        position: native.position,
        orientation: native.orientation,
        size: native.size,
        position_on_image: native.position_on_image,
        size_on_image: native.size_on_image,
        number_of_colors: native.number_of_colors,
        colors: colors_slice.to_vec(),
        model: model_str,
    }
}
