use crate::v2025a::bindings::{
    WbDeviceTag, wb_robot_cleanup, wb_robot_get_basic_time_step, wb_robot_get_device,
    wb_robot_get_time, wb_robot_step,
};
use crate::v2025a::device::lidar::{Lidar, LidarConfig};
use crate::v2025a::{SimulatorError, device, supervisor};
use std::ffi::CString;
use std::os::raw::c_int;

#[cfg(windows)]
unsafe extern "C" {
    fn wb_robot_init_msvc() -> c_int;
}

#[cfg(windows)]
unsafe fn wb_robot_init_wrapper() -> c_int {
    unsafe { wb_robot_init_msvc() }
}

#[cfg(not(windows))]
unsafe fn wb_robot_init_wrapper() -> c_int {
    unsafe { crate::v2025a::bindings::wb_robot_init() }
}

#[derive(Debug)]
pub struct Webots {
    cleanup_on_drop: bool,
}

pub type Simulator = Webots;

impl Webots {
    pub fn new() -> Result<Self, SimulatorError> {
        ffi_try!(wb_robot_init_wrapper())?;
        unsafe { libc::signal(libc::SIGINT, libc::SIG_DFL) };
        Ok(Self {
            cleanup_on_drop: true,
        })
    }

    pub fn step(&self, step_ms: i32) -> Result<bool, SimulatorError> {
        let step_result = ffi_try!(wb_robot_step(step_ms))?;
        Ok(step_result != -1)
    }

    fn get_tag(&self, name: String) -> Result<WbDeviceTag, SimulatorError> {
        let c_name = CString::new(name.clone())?;
        let tag = ffi_try!(wb_robot_get_device(c_name.as_ptr()))?;

        if (tag as i32) == 0 {
            return Err(SimulatorError::UnknownDevice(name));
        }

        Ok(tag)
    }

    pub fn motor(&self, name: impl Into<String>) -> Result<device::motor::Motor, SimulatorError> {
        Ok(device::motor::Motor::new(self.get_tag(name.into())?))
    }

    pub fn position_sensor(
        &self,
        name: impl Into<String>,
    ) -> Result<device::position_sensor::PositionSensor, SimulatorError> {
        Ok(device::position_sensor::PositionSensor::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn accelerometer(
        &self,
        name: impl Into<String>,
    ) -> Result<device::accelerometer::Accelerometer, SimulatorError> {
        Ok(device::accelerometer::Accelerometer::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn altimeter(
        &self,
        name: impl Into<String>,
    ) -> Result<device::altimeter::Altimeter, SimulatorError> {
        Ok(device::altimeter::Altimeter::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn distance_sensor(
        &self,
        name: impl Into<String>,
    ) -> Result<device::distance_sensor::DistanceSensor, SimulatorError> {
        Ok(device::distance_sensor::DistanceSensor::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn camera(
        &self,
        name: impl Into<String>,
    ) -> Result<device::camera::Camera, SimulatorError> {
        Ok(device::camera::Camera::new(self.get_tag(name.into())?))
    }

    pub fn gps(&self, name: impl Into<String>) -> Result<device::gps::Gps, SimulatorError> {
        Ok(device::gps::Gps::new(self.get_tag(name.into())?))
    }

    pub fn gyro(&self, name: impl Into<String>) -> Result<device::gyro::Gyro, SimulatorError> {
        Ok(device::gyro::Gyro::new(self.get_tag(name.into())?))
    }

    pub fn inertial_unit(
        &self,
        name: impl Into<String>,
    ) -> Result<device::inertial_unit::InertialUnit, SimulatorError> {
        Ok(device::inertial_unit::InertialUnit::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn compass(
        &self,
        name: impl Into<String>,
    ) -> Result<device::compass::Compass, SimulatorError> {
        Ok(device::compass::Compass::new(self.get_tag(name.into())?))
    }

    pub fn light_sensor(
        &self,
        name: impl Into<String>,
    ) -> Result<device::light_sensor::LightSensor, SimulatorError> {
        Ok(device::light_sensor::LightSensor::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn lidar(
        &self,
        name: impl Into<String>,
        config: LidarConfig,
    ) -> Result<Lidar, SimulatorError> {
        Ok(Lidar::new(self.get_tag(name.into())?, config))
    }

    pub fn radar(&self, name: impl Into<String>) -> Result<device::radar::Radar, SimulatorError> {
        Ok(device::radar::Radar::new(self.get_tag(name.into())?))
    }

    pub fn receiver(
        &self,
        name: impl Into<String>,
    ) -> Result<device::receiver::Receiver, SimulatorError> {
        Ok(device::receiver::Receiver::new(self.get_tag(name.into())?))
    }

    pub fn emitter(
        &self,
        name: impl Into<String>,
    ) -> Result<device::emitter::Emitter, SimulatorError> {
        Ok(device::emitter::Emitter::new(self.get_tag(name.into())?))
    }

    pub fn led(&self, name: impl Into<String>) -> Result<device::led::Led, SimulatorError> {
        Ok(device::led::Led::new(self.get_tag(name.into())?))
    }

    pub fn touch_sensor(
        &self,
        name: impl Into<String>,
    ) -> Result<device::touch_sensor::TouchSensor, SimulatorError> {
        Ok(device::touch_sensor::TouchSensor::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn touch_sensor3d(
        &self,
        name: impl Into<String>,
    ) -> Result<device::touch_sensor3d::TouchSensor3D, SimulatorError> {
        Ok(device::touch_sensor3d::TouchSensor3D::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn connector(
        &self,
        name: impl Into<String>,
    ) -> Result<device::connector::Connector, SimulatorError> {
        Ok(device::connector::Connector::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn brake(&self, name: impl Into<String>) -> Result<device::brake::Brake, SimulatorError> {
        Ok(device::brake::Brake::new(self.get_tag(name.into())?))
    }

    pub fn vacuum_gripper(
        &self,
        name: impl Into<String>,
    ) -> Result<device::vacuum_gripper::VacuumGripper, SimulatorError> {
        Ok(device::vacuum_gripper::VacuumGripper::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn microphone(
        &self,
        name: impl Into<String>,
    ) -> Result<device::microphone::Microphone, SimulatorError> {
        Ok(device::microphone::Microphone::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn speaker(
        &self,
        name: impl Into<String>,
    ) -> Result<device::speaker::Speaker, SimulatorError> {
        Ok(device::speaker::Speaker::new(self.get_tag(name.into())?))
    }

    pub fn radio(&self, name: impl Into<String>) -> Result<device::radio::Radio, SimulatorError> {
        Ok(device::radio::Radio::new(self.get_tag(name.into())?))
    }

    pub fn range_finder(
        &self,
        name: impl Into<String>,
    ) -> Result<device::range_finder::RangeFinder, SimulatorError> {
        Ok(device::range_finder::RangeFinder::new(
            self.get_tag(name.into())?,
        ))
    }

    pub fn get_supervisor(&self) -> supervisor::Supervisor {
        supervisor::Supervisor::new()
    }

    pub fn get_basic_time_step(&self) -> Result<f64, SimulatorError> {
        let timestep = ffi_try!(wb_robot_get_basic_time_step())?;
        Ok(timestep)
    }

    pub fn get_time(&self) -> Result<f64, SimulatorError> {
        let time = ffi_try!(wb_robot_get_time())?;
        Ok(time)
    }
}

impl Drop for Webots {
    fn drop(&mut self) {
        if self.cleanup_on_drop {
            let _ = ffi_try!(wb_robot_cleanup());
        }
    }
}
