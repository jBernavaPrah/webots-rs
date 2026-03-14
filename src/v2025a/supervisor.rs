#![allow(clippy::macro_metavars_in_unsafe)]

// Supervisor support for the v2025a Webots API surface.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::slice;

use crate::v2025a::{bindings, SimulatorError};

fn bool_from_c_char(value: c_char) -> bool {
    value != 0
}

fn bool_to_c_char(value: bool) -> c_char {
    if value {
        1
    } else {
        0
    }
}

fn string_from_ptr(ptr: *const c_char) -> Result<String, SimulatorError> {
    if ptr.is_null() {
        return Err(SimulatorError::UnsafeOperation);
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };
    Ok(c_str.to_str()?.to_owned())
}

fn array_from_ptr<const N: usize>(ptr: *const f64) -> Result<[f64; N], SimulatorError> {
    if ptr.is_null() {
        return Err(SimulatorError::UnsafeOperation);
    }

    let slice = unsafe { slice::from_raw_parts(ptr, N) };
    let mut array = [0.0; N];
    array.copy_from_slice(slice);
    Ok(array)
}

fn contact_points_from_raw(
    ptr: *mut bindings::WbContactPoint,
    len: usize,
) -> Result<Vec<ContactPoint>, SimulatorError> {
    if len == 0 {
        return Ok(Vec::new());
    }

    if ptr.is_null() {
        return Err(SimulatorError::UnsafeOperation);
    }

    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    Ok(slice
        .iter()
        .map(|point| ContactPoint {
            point: point.point,
            node_id: point.node_id,
        })
        .collect())
}

#[derive(Debug, Clone)]
pub struct ContactPoint {
    pub point: [f64; 3],
    pub node_id: i32,
}

#[derive(Default)]
pub struct Supervisor;

impl Supervisor {
    pub fn new() -> Self {
        Self
    }

    pub fn world_load(&self, filename: &str) -> Result<(), SimulatorError> {
        let c_filename = CString::new(filename)?;
        ffi_try!(bindings::wb_supervisor_world_load(c_filename.as_ptr()))?;
        Ok(())
    }

    pub fn world_save(&self, filename: &str) -> Result<bool, SimulatorError> {
        let c_filename = CString::new(filename)?;
        let result = ffi_try!(bindings::wb_supervisor_world_save(c_filename.as_ptr()))?;
        Ok(bool_from_c_char(result))
    }

    pub fn world_reload(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_world_reload())?;
        Ok(())
    }

    pub fn load_world(&self, filename: &str) -> Result<(), SimulatorError> {
        let c_filename = CString::new(filename)?;
        ffi_try!(bindings::wb_supervisor_load_world(c_filename.as_ptr()))?;
        Ok(())
    }

    pub fn save_world(&self, filename: &str) -> Result<bool, SimulatorError> {
        let c_filename = CString::new(filename)?;
        let result = ffi_try!(bindings::wb_supervisor_save_world(c_filename.as_ptr()))?;
        Ok(bool_from_c_char(result))
    }

    pub fn simulation_quit(&self, status: c_int) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_simulation_quit(status))?;
        Ok(())
    }

    pub fn simulation_reset(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_simulation_reset())?;
        Ok(())
    }

    pub fn simulation_reset_physics(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_simulation_reset_physics())?;
        Ok(())
    }

    pub fn simulation_physics_reset(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_simulation_physics_reset())?;
        Ok(())
    }

    pub fn simulation_revert(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_simulation_revert())?;
        Ok(())
    }

    pub fn simulation_get_mode(&self) -> Result<bindings::WbSimulationMode, SimulatorError> {
        let mode = ffi_try!(bindings::wb_supervisor_simulation_get_mode())?;
        Ok(mode)
    }

    pub fn simulation_set_mode(
        &self,
        mode: bindings::WbSimulationMode,
    ) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_simulation_set_mode(mode))?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_label(
        &self,
        id: c_int,
        text: &str,
        x: f64,
        y: f64,
        size: f64,
        color: c_int,
        transparency: f64,
        font: &str,
    ) -> Result<(), SimulatorError> {
        let c_text = CString::new(text)?;
        let c_font = CString::new(font)?;
        ffi_try!(bindings::wb_supervisor_set_label(
            id,
            c_text.as_ptr(),
            x,
            y,
            size,
            color,
            transparency,
            c_font.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn export_image(&self, filename: &str, quality: c_int) -> Result<(), SimulatorError> {
        let c_filename = CString::new(filename)?;
        ffi_try!(bindings::wb_supervisor_export_image(
            c_filename.as_ptr(),
            quality
        ))?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn movie_start_recording(
        &self,
        filename: &str,
        width: c_int,
        height: c_int,
        codec: c_int,
        quality: c_int,
        acceleration: c_int,
        caption: bool,
    ) -> Result<(), SimulatorError> {
        let c_filename = CString::new(filename)?;
        ffi_try!(bindings::wb_supervisor_movie_start_recording(
            c_filename.as_ptr(),
            width,
            height,
            codec,
            quality,
            acceleration,
            bool_to_c_char(caption),
        ))?;
        Ok(())
    }

    pub fn movie_stop_recording(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_movie_stop_recording())?;
        Ok(())
    }

    pub fn movie_is_ready(&self) -> Result<bool, SimulatorError> {
        let ready = ffi_try!(bindings::wb_supervisor_movie_is_ready())?;
        Ok(bool_from_c_char(ready))
    }

    pub fn movie_failed(&self) -> Result<bool, SimulatorError> {
        let failed = ffi_try!(bindings::wb_supervisor_movie_failed())?;
        Ok(bool_from_c_char(failed))
    }

    pub fn movie_get_status(&self) -> Result<c_int, SimulatorError> {
        let status = ffi_try!(bindings::wb_supervisor_movie_get_status())?;
        Ok(status)
    }

    pub fn animation_start_recording(&self, filename: &str) -> Result<bool, SimulatorError> {
        let c_filename = CString::new(filename)?;
        let started = ffi_try!(bindings::wb_supervisor_animation_start_recording(
            c_filename.as_ptr(),
        ))?;
        Ok(bool_from_c_char(started))
    }

    pub fn animation_stop_recording(&self) -> Result<bool, SimulatorError> {
        let stopped = ffi_try!(bindings::wb_supervisor_animation_stop_recording())?;
        Ok(bool_from_c_char(stopped))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn start_movie(
        &self,
        filename: &str,
        width: c_int,
        height: c_int,
        codec: c_int,
        quality: c_int,
        acceleration: c_int,
        caption: bool,
    ) -> Result<(), SimulatorError> {
        let c_filename = CString::new(filename)?;
        ffi_try!(bindings::wb_supervisor_start_movie(
            c_filename.as_ptr(),
            width,
            height,
            codec,
            quality,
            acceleration,
            bool_to_c_char(caption),
        ))?;
        Ok(())
    }

    pub fn stop_movie(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_stop_movie())?;
        Ok(())
    }

    pub fn get_root(&self) -> Result<Node, SimulatorError> {
        Node::new(ffi_try!(bindings::wb_supervisor_node_get_root())?)
    }

    pub fn get_self(&self) -> Result<Node, SimulatorError> {
        Node::new(ffi_try!(bindings::wb_supervisor_node_get_self())?)
    }

    pub fn get_selected(&self) -> Result<Node, SimulatorError> {
        Node::new(ffi_try!(bindings::wb_supervisor_node_get_selected())?)
    }

    pub fn virtual_reality_headset_is_used(&self) -> Result<bool, SimulatorError> {
        let used = ffi_try!(bindings::wb_supervisor_virtual_reality_headset_is_used())?;
        Ok(bool_from_c_char(used))
    }

    pub fn virtual_reality_headset_position(&self) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_virtual_reality_headset_get_position())?;
        array_from_ptr(ptr)
    }

    pub fn virtual_reality_headset_orientation(&self) -> Result<[f64; 4], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_virtual_reality_headset_get_orientation())?;
        array_from_ptr(ptr)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Node(bindings::WbNodeRef);

impl Node {
    fn new(node: bindings::WbNodeRef) -> Result<Self, SimulatorError> {
        if node.is_null() {
            return Err(SimulatorError::UnsafeOperation);
        }
        Ok(Self(node))
    }

    pub fn from_id(id: c_int) -> Result<Self, SimulatorError> {
        Node::new(ffi_try!(bindings::wb_supervisor_node_get_from_id(id))?)
    }

    pub fn from_device(tag: bindings::WbDeviceTag) -> Result<Self, SimulatorError> {
        Node::new(ffi_try!(bindings::wb_supervisor_node_get_from_device(tag))?)
    }

    pub fn from_def(def: &str) -> Result<Self, SimulatorError> {
        let c_def = CString::new(def)?;
        Node::new(ffi_try!(bindings::wb_supervisor_node_get_from_def(
            c_def.as_ptr()
        ))?)
    }

    pub fn from_proto_def(&self, def: &str) -> Result<Self, SimulatorError> {
        let c_def = CString::new(def)?;
        Node::new(ffi_try!(bindings::wb_supervisor_node_get_from_proto_def(
            self.0,
            c_def.as_ptr(),
        ))?)
    }

    pub fn id(&self) -> Result<c_int, SimulatorError> {
        let id = ffi_try!(bindings::wb_supervisor_node_get_id(self.0))?;
        Ok(id)
    }

    pub fn node_type(&self) -> Result<bindings::WbNodeType, SimulatorError> {
        let node_type = ffi_try!(bindings::wb_supervisor_node_get_type(self.0))?;
        Ok(node_type)
    }

    pub fn parent(&self) -> Result<Self, SimulatorError> {
        Node::new(ffi_try!(bindings::wb_supervisor_node_get_parent_node(
            self.0
        ))?)
    }

    pub fn field(&self, name: &str) -> Result<Field, SimulatorError> {
        let c_name = CString::new(name)?;
        Field::new(ffi_try!(bindings::wb_supervisor_node_get_field(
            self.0,
            c_name.as_ptr(),
        ))?)
    }

    pub fn field_by_index(&self, index: c_int) -> Result<Field, SimulatorError> {
        Field::new(ffi_try!(bindings::wb_supervisor_node_get_field_by_index(
            self.0, index,
        ))?)
    }

    pub fn number_of_fields(&self) -> Result<c_int, SimulatorError> {
        let count = ffi_try!(bindings::wb_supervisor_node_get_number_of_fields(self.0))?;
        Ok(count)
    }

    pub fn base_node_field(&self, name: &str) -> Result<Field, SimulatorError> {
        let c_name = CString::new(name)?;
        Field::new(ffi_try!(bindings::wb_supervisor_node_get_base_node_field(
            self.0,
            c_name.as_ptr(),
        ))?)
    }

    pub fn base_node_field_by_index(&self, index: c_int) -> Result<Field, SimulatorError> {
        Field::new(ffi_try!(
            bindings::wb_supervisor_node_get_base_node_field_by_index(self.0, index,)
        )?)
    }

    pub fn number_of_base_node_fields(&self) -> Result<c_int, SimulatorError> {
        let count = ffi_try!(bindings::wb_supervisor_node_get_number_of_base_node_fields(
            self.0,
        ))?;
        Ok(count)
    }

    pub fn remove(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_remove(self.0))?;
        Ok(())
    }

    pub fn save_state(&self, state_name: &str) -> Result<(), SimulatorError> {
        let c_state = CString::new(state_name)?;
        ffi_try!(bindings::wb_supervisor_node_save_state(
            self.0,
            c_state.as_ptr()
        ))?;
        Ok(())
    }

    pub fn load_state(&self, state_name: &str) -> Result<(), SimulatorError> {
        let c_state = CString::new(state_name)?;
        ffi_try!(bindings::wb_supervisor_node_load_state(
            self.0,
            c_state.as_ptr()
        ))?;
        Ok(())
    }

    pub fn set_joint_position(&self, position: f64, index: c_int) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_set_joint_position(
            self.0, position, index,
        ))?;
        Ok(())
    }

    pub fn proto(&self) -> Result<Proto, SimulatorError> {
        Proto::new(ffi_try!(bindings::wb_supervisor_node_get_proto(self.0))?)
    }

    pub fn def(&self) -> Result<String, SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_node_get_def(self.0))?;
        string_from_ptr(ptr)
    }

    pub fn type_name(&self) -> Result<String, SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_node_get_type_name(self.0))?;
        string_from_ptr(ptr)
    }

    pub fn base_type_name(&self) -> Result<String, SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_node_get_base_type_name(self.0))?;
        string_from_ptr(ptr)
    }

    pub fn is_proto(&self) -> Result<bool, SimulatorError> {
        let value = ffi_try!(bindings::wb_supervisor_node_is_proto(self.0))?;
        Ok(bool_from_c_char(value))
    }

    pub fn center_of_mass(&self) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_node_get_center_of_mass(self.0))?;
        array_from_ptr(ptr)
    }

    pub fn contact_point(&self, index: c_int) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_node_get_contact_point(
            self.0, index,
        ))?;
        array_from_ptr(ptr)
    }

    pub fn contact_point_node(&self, index: c_int) -> Result<Self, SimulatorError> {
        Node::new(ffi_try!(
            bindings::wb_supervisor_node_get_contact_point_node(self.0, index,)
        )?)
    }

    pub fn number_of_contact_points(
        &self,
        include_descendants: bool,
    ) -> Result<c_int, SimulatorError> {
        let count = ffi_try!(bindings::wb_supervisor_node_get_number_of_contact_points(
            self.0,
            bool_to_c_char(include_descendants),
        ))?;
        Ok(count)
    }

    pub fn contact_points(
        &self,
        include_descendants: bool,
    ) -> Result<Vec<ContactPoint>, SimulatorError> {
        let mut size: c_int = 0;
        let pointer = ffi_try!(bindings::wb_supervisor_node_get_contact_points(
            self.0,
            bool_to_c_char(include_descendants),
            &mut size,
        ))?;
        contact_points_from_raw(pointer, size as usize)
    }

    pub fn orientation(&self) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_node_get_orientation(self.0))?;
        array_from_ptr(ptr)
    }

    pub fn position(&self) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_node_get_position(self.0))?;
        array_from_ptr(ptr)
    }

    pub fn pose(&self, from: &Node) -> Result<[f64; 7], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_node_get_pose(self.0, from.0))?;
        array_from_ptr(ptr)
    }

    pub fn static_balance(&self) -> Result<bool, SimulatorError> {
        let value = ffi_try!(bindings::wb_supervisor_node_get_static_balance(self.0))?;
        Ok(bool_from_c_char(value))
    }

    pub fn velocity(&self) -> Result<[f64; 6], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_node_get_velocity(self.0))?;
        array_from_ptr(ptr)
    }

    pub fn set_velocity(&self, velocity: [f64; 6]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_set_velocity(
            self.0,
            velocity.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn reset_physics(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_reset_physics(self.0))?;
        Ok(())
    }

    pub fn restart_controller(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_restart_controller(self.0))?;
        Ok(())
    }

    pub fn export_string(&self) -> Result<String, SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_node_export_string(self.0))?;
        string_from_ptr(ptr)
    }

    pub fn move_viewpoint(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_move_viewpoint(self.0))?;
        Ok(())
    }

    pub fn set_visibility(&self, from: &Node, visible: bool) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_set_visibility(
            self.0,
            from.0,
            bool_to_c_char(visible),
        ))?;
        Ok(())
    }

    pub fn add_force(&self, force: [f64; 3], relative: bool) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_add_force(
            self.0,
            force.as_ptr(),
            bool_to_c_char(relative),
        ))?;
        Ok(())
    }

    pub fn add_force_with_offset(
        &self,
        force: [f64; 3],
        offset: [f64; 3],
        relative: bool,
    ) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_add_force_with_offset(
            self.0,
            force.as_ptr(),
            offset.as_ptr(),
            bool_to_c_char(relative),
        ))?;
        Ok(())
    }

    pub fn add_torque(&self, torque: [f64; 3], relative: bool) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_add_torque(
            self.0,
            torque.as_ptr(),
            bool_to_c_char(relative),
        ))?;
        Ok(())
    }

    pub fn enable_pose_tracking(
        &self,
        sampling_period: c_int,
        from: &Node,
    ) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_enable_pose_tracking(
            self.0,
            sampling_period,
            from.0,
        ))?;
        Ok(())
    }

    pub fn disable_pose_tracking(&self, from: &Node) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_disable_pose_tracking(
            self.0, from.0,
        ))?;
        Ok(())
    }

    pub fn enable_contact_points_tracking(
        &self,
        sampling_period: c_int,
        include_descendants: bool,
    ) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_enable_contact_points_tracking(
            self.0,
            sampling_period,
            bool_to_c_char(include_descendants),
        ))?;
        Ok(())
    }

    pub fn disable_contact_points_tracking(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_disable_contact_points_tracking(self.0))?;
        Ok(())
    }

    pub fn enable_contact_point_tracking(
        &self,
        sampling_period: c_int,
        include_descendants: bool,
    ) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_enable_contact_point_tracking(
            self.0,
            sampling_period,
            bool_to_c_char(include_descendants),
        ))?;
        Ok(())
    }

    pub fn disable_contact_point_tracking(
        &self,
        include_descendants: bool,
    ) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_node_disable_contact_point_tracking(
            self.0,
            bool_to_c_char(include_descendants),
        ))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Field(bindings::WbFieldRef);

impl Field {
    fn new(field: bindings::WbFieldRef) -> Result<Self, SimulatorError> {
        if field.is_null() {
            return Err(SimulatorError::UnsafeOperation);
        }
        Ok(Self(field))
    }

    pub fn name(&self) -> Result<String, SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_name(self.0))?;
        string_from_ptr(ptr)
    }

    pub fn field_type(&self) -> Result<bindings::WbFieldType, SimulatorError> {
        let field_type = ffi_try!(bindings::wb_supervisor_field_get_type(self.0))?;
        Ok(field_type)
    }

    pub fn type_name(&self) -> Result<String, SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_type_name(self.0))?;
        string_from_ptr(ptr)
    }

    pub fn count(&self) -> Result<c_int, SimulatorError> {
        let count = ffi_try!(bindings::wb_supervisor_field_get_count(self.0))?;
        Ok(count)
    }

    pub fn actual_field(&self) -> Result<Self, SimulatorError> {
        Field::new(ffi_try!(bindings::wb_supervisor_field_get_actual_field(
            self.0
        ))?)
    }

    pub fn enable_sf_tracking(&self, sampling_period: c_int) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_enable_sf_tracking(
            self.0,
            sampling_period,
        ))?;
        Ok(())
    }

    pub fn disable_sf_tracking(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_disable_sf_tracking(self.0))?;
        Ok(())
    }

    pub fn sf_bool(&self) -> Result<bool, SimulatorError> {
        let value = ffi_try!(bindings::wb_supervisor_field_get_sf_bool(self.0))?;
        Ok(bool_from_c_char(value))
    }

    pub fn sf_int32(&self) -> Result<c_int, SimulatorError> {
        let value = ffi_try!(bindings::wb_supervisor_field_get_sf_int32(self.0))?;
        Ok(value)
    }

    pub fn sf_float(&self) -> Result<f64, SimulatorError> {
        let value = ffi_try!(bindings::wb_supervisor_field_get_sf_float(self.0))?;
        Ok(value)
    }

    pub fn sf_vec2f(&self) -> Result<[f64; 2], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_sf_vec2f(self.0))?;
        array_from_ptr(ptr)
    }

    pub fn sf_vec3f(&self) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_sf_vec3f(self.0))?;
        array_from_ptr(ptr)
    }

    pub fn sf_rotation(&self) -> Result<[f64; 4], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_sf_rotation(self.0))?;
        array_from_ptr(ptr)
    }

    pub fn sf_color(&self) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_sf_color(self.0))?;
        array_from_ptr(ptr)
    }

    pub fn sf_string(&self) -> Result<String, SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_sf_string(self.0))?;
        string_from_ptr(ptr)
    }

    pub fn sf_node(&self) -> Result<Node, SimulatorError> {
        Node::new(ffi_try!(bindings::wb_supervisor_field_get_sf_node(self.0))?)
    }

    pub fn mf_bool(&self, index: c_int) -> Result<bool, SimulatorError> {
        let value = ffi_try!(bindings::wb_supervisor_field_get_mf_bool(self.0, index))?;
        Ok(bool_from_c_char(value))
    }

    pub fn mf_int32(&self, index: c_int) -> Result<c_int, SimulatorError> {
        let value = ffi_try!(bindings::wb_supervisor_field_get_mf_int32(self.0, index))?;
        Ok(value)
    }

    pub fn mf_float(&self, index: c_int) -> Result<f64, SimulatorError> {
        let value = ffi_try!(bindings::wb_supervisor_field_get_mf_float(self.0, index))?;
        Ok(value)
    }

    pub fn mf_vec2f(&self, index: c_int) -> Result<[f64; 2], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_mf_vec2f(self.0, index))?;
        array_from_ptr(ptr)
    }

    pub fn mf_vec3f(&self, index: c_int) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_mf_vec3f(self.0, index))?;
        array_from_ptr(ptr)
    }

    pub fn mf_color(&self, index: c_int) -> Result<[f64; 3], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_mf_color(self.0, index))?;
        array_from_ptr(ptr)
    }

    pub fn mf_rotation(&self, index: c_int) -> Result<[f64; 4], SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_mf_rotation(self.0, index))?;
        array_from_ptr(ptr)
    }

    pub fn mf_string(&self, index: c_int) -> Result<String, SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_field_get_mf_string(self.0, index))?;
        string_from_ptr(ptr)
    }

    pub fn mf_node(&self, index: c_int) -> Result<Node, SimulatorError> {
        Node::new(ffi_try!(bindings::wb_supervisor_field_get_mf_node(
            self.0, index
        ))?)
    }

    pub fn set_sf_bool(&self, value: bool) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_sf_bool(
            self.0,
            bool_to_c_char(value),
        ))?;
        Ok(())
    }

    pub fn set_sf_int32(&self, value: c_int) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_sf_int32(self.0, value))?;
        Ok(())
    }

    pub fn set_sf_float(&self, value: f64) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_sf_float(self.0, value))?;
        Ok(())
    }

    pub fn set_sf_vec2f(&self, values: [f64; 2]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_sf_vec2f(
            self.0,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn set_sf_vec3f(&self, values: [f64; 3]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_sf_vec3f(
            self.0,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn set_sf_rotation(&self, values: [f64; 4]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_sf_rotation(
            self.0,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn set_sf_color(&self, values: [f64; 3]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_sf_color(
            self.0,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn set_sf_string(&self, value: &str) -> Result<(), SimulatorError> {
        let c_value = CString::new(value)?;
        ffi_try!(bindings::wb_supervisor_field_set_sf_string(
            self.0,
            c_value.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn set_mf_bool(&self, index: c_int, value: bool) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_mf_bool(
            self.0,
            index,
            bool_to_c_char(value),
        ))?;
        Ok(())
    }

    pub fn set_mf_int32(&self, index: c_int, value: c_int) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_mf_int32(
            self.0, index, value,
        ))?;
        Ok(())
    }

    pub fn set_mf_float(&self, index: c_int, value: f64) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_mf_float(
            self.0, index, value,
        ))?;
        Ok(())
    }

    pub fn set_mf_vec2f(&self, index: c_int, values: [f64; 2]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_mf_vec2f(
            self.0,
            index,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn set_mf_vec3f(&self, index: c_int, values: [f64; 3]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_mf_vec3f(
            self.0,
            index,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn set_mf_rotation(&self, index: c_int, values: [f64; 4]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_mf_rotation(
            self.0,
            index,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn set_mf_color(&self, index: c_int, values: [f64; 3]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_set_mf_color(
            self.0,
            index,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn set_mf_string(&self, index: c_int, value: &str) -> Result<(), SimulatorError> {
        let c_value = CString::new(value)?;
        ffi_try!(bindings::wb_supervisor_field_set_mf_string(
            self.0,
            index,
            c_value.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn insert_mf_bool(&self, index: c_int, value: bool) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_insert_mf_bool(
            self.0,
            index,
            bool_to_c_char(value),
        ))?;
        Ok(())
    }

    pub fn insert_mf_int32(&self, index: c_int, value: c_int) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_insert_mf_int32(
            self.0, index, value,
        ))?;
        Ok(())
    }

    pub fn insert_mf_float(&self, index: c_int, value: f64) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_insert_mf_float(
            self.0, index, value,
        ))?;
        Ok(())
    }

    pub fn insert_mf_vec2f(&self, index: c_int, values: [f64; 2]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_insert_mf_vec2f(
            self.0,
            index,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn insert_mf_vec3f(&self, index: c_int, values: [f64; 3]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_insert_mf_vec3f(
            self.0,
            index,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn insert_mf_rotation(&self, index: c_int, values: [f64; 4]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_insert_mf_rotation(
            self.0,
            index,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn insert_mf_color(&self, index: c_int, values: [f64; 3]) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_insert_mf_color(
            self.0,
            index,
            values.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn insert_mf_string(&self, index: c_int, value: &str) -> Result<(), SimulatorError> {
        let c_value = CString::new(value)?;
        ffi_try!(bindings::wb_supervisor_field_insert_mf_string(
            self.0,
            index,
            c_value.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn remove_mf(&self, index: c_int) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_remove_mf(self.0, index))?;
        Ok(())
    }

    pub fn remove_mf_node(&self, index: c_int) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_remove_mf_node(self.0, index))?;
        Ok(())
    }

    pub fn remove_sf(&self) -> Result<(), SimulatorError> {
        ffi_try!(bindings::wb_supervisor_field_remove_sf(self.0))?;
        Ok(())
    }

    pub fn import_mf_node_from_string(
        &self,
        position: c_int,
        proto: &str,
    ) -> Result<(), SimulatorError> {
        let c_proto = CString::new(proto)?;
        ffi_try!(bindings::wb_supervisor_field_import_mf_node_from_string(
            self.0,
            position,
            c_proto.as_ptr(),
        ))?;
        Ok(())
    }

    pub fn import_sf_node_from_string(&self, node: &str) -> Result<(), SimulatorError> {
        let c_node = CString::new(node)?;
        ffi_try!(bindings::wb_supervisor_field_import_sf_node_from_string(
            self.0,
            c_node.as_ptr(),
        ))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Proto(bindings::WbProtoRef);

impl Proto {
    fn new(proto: bindings::WbProtoRef) -> Result<Self, SimulatorError> {
        if proto.is_null() {
            return Err(SimulatorError::UnsafeOperation);
        }
        Ok(Self(proto))
    }

    pub fn type_name(&self) -> Result<String, SimulatorError> {
        let ptr = ffi_try!(bindings::wb_supervisor_proto_get_type_name(self.0))?;
        string_from_ptr(ptr)
    }

    pub fn is_derived(&self) -> Result<bool, SimulatorError> {
        let value = ffi_try!(bindings::wb_supervisor_proto_is_derived(self.0))?;
        Ok(bool_from_c_char(value))
    }

    pub fn parent(&self) -> Result<Self, SimulatorError> {
        Proto::new(ffi_try!(bindings::wb_supervisor_proto_get_parent(self.0))?)
    }

    pub fn field(&self, name: &str) -> Result<Field, SimulatorError> {
        let c_name = CString::new(name)?;
        Field::new(ffi_try!(bindings::wb_supervisor_proto_get_field(
            self.0,
            c_name.as_ptr(),
        ))?)
    }

    pub fn field_by_index(&self, index: c_int) -> Result<Field, SimulatorError> {
        Field::new(ffi_try!(bindings::wb_supervisor_proto_get_field_by_index(
            self.0, index,
        ))?)
    }

    pub fn number_of_fields(&self) -> Result<c_int, SimulatorError> {
        let count = ffi_try!(bindings::wb_supervisor_proto_get_number_of_fields(self.0))?;
        Ok(count)
    }
}
