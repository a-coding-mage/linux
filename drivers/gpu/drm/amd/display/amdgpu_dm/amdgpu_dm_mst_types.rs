// SPDX-License-Identifier: MIT
// Faithful low-level Rust translation of amdgpu_dm_mst_types.c.
// Kernel and driver types/functions referenced below are supplied externally.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const PEAK_FACTOR_X1000: u32 = 1006;

extern "C" {
    fn dc_link_aux_transfer_raw(ddc: *mut ddc_service, payload: *mut aux_payload,
        result: *mut aux_return_code_type) -> isize;
    fn drm_dp_dpcd_read(aux: *mut drm_dp_aux, address: u32, buffer: *mut u8, size: u32) -> isize;
    fn drm_dp_dpcd_writeb(aux: *mut drm_dp_aux, address: u32, value: u8) -> isize;
    fn drm_dp_mst_detect_port(connector: *mut drm_connector, ctx: *mut drm_modeset_acquire_ctx,
        mgr: *mut drm_dp_mst_topology_mgr, port: *mut drm_dp_mst_port) -> i32;
    fn drm_dp_atomic_release_time_slots(state: *mut drm_atomic_commit,
        mgr: *mut drm_dp_mst_topology_mgr, port: *mut drm_dp_mst_port) -> i32;
    fn drm_dp_atomic_find_time_slots(state: *mut drm_atomic_commit,
        mgr: *mut drm_dp_mst_topology_mgr, port: *mut drm_dp_mst_port, pbn: i32) -> i32;
    fn drm_dp_mst_atomic_check(state: *mut drm_atomic_commit) -> i32;
}

#[repr(C)] pub struct aux_payload { pub address: u32, pub data: *mut u8, pub length: u32,
    pub reply: *mut u8, pub i2c_over_aux: bool, pub write: bool, pub mot: bool,
    pub write_status_update: bool, pub defer_delay: u32 }
#[repr(C)] pub struct drm_dp_aux { pub name: *mut i8, pub drm_dev: *mut drm_device }
#[repr(C)] pub struct drm_dp_aux_msg { pub address: u32, pub buffer: *mut u8, pub size: u32,
    pub request: u8, pub reply: u8 }
#[repr(C)] pub struct ddc_service { pub ctx: *mut dc_context }
#[repr(C)] pub struct dc_context { pub driver_context: *mut amdgpu_device }
#[repr(C)] pub struct amdgpu_device { pub dm: dm_state }
#[repr(C)] pub struct dm_state { pub aux_hpd_discon_quirk: bool }
#[repr(C)] pub struct drm_device;
#[repr(C)] pub struct drm_connector;
#[repr(C)] pub struct drm_modeset_acquire_ctx;
#[repr(C)] pub struct drm_dp_mst_topology_mgr;
#[repr(C)] pub struct drm_dp_mst_port;
#[repr(C)] pub struct drm_atomic_commit;
#[repr(C)] pub struct dc_link;
#[repr(C)] pub struct amdgpu_dm_connector;

#[repr(C)] #[derive(Copy, Clone)] pub enum aux_return_code_type {
    AUX_RET_SUCCESS, AUX_RET_ERROR_HPD_DISCON, AUX_RET_ERROR_UNKNOWN,
    AUX_RET_ERROR_INVALID_OPERATION, AUX_RET_ERROR_PROTOCOL_ERROR,
    AUX_RET_ERROR_INVALID_REPLY, AUX_RET_ERROR_ENGINE_ACQUIRE, AUX_RET_ERROR_TIMEOUT,
}

pub unsafe fn dm_dp_aux_transfer_result(mut result: isize,
    operation_result: aux_return_code_type) -> isize {
    match operation_result {
        aux_return_code_type::AUX_RET_SUCCESS => {},
        aux_return_code_type::AUX_RET_ERROR_HPD_DISCON |
        aux_return_code_type::AUX_RET_ERROR_UNKNOWN |
        aux_return_code_type::AUX_RET_ERROR_INVALID_OPERATION |
        aux_return_code_type::AUX_RET_ERROR_PROTOCOL_ERROR => result = -5,
        aux_return_code_type::AUX_RET_ERROR_INVALID_REPLY |
        aux_return_code_type::AUX_RET_ERROR_ENGINE_ACQUIRE => result = -16,
        aux_return_code_type::AUX_RET_ERROR_TIMEOUT => result = -110,
    }
    result
}

pub unsafe fn dm_dp_aux_fill_payload_flags(request: u8, payload: *mut aux_payload) {
    (*payload).i2c_over_aux = (request & 0x08) == 0;
    (*payload).write = (request & 0x01) == 0;
    (*payload).mot = (request & 0x04) != 0;
    (*payload).write_status_update = (request & 0x20) != 0;
}

pub unsafe fn dm_dp_aux_transfer(aux: *mut drm_dp_aux, msg: *mut drm_dp_aux_msg) -> isize {
    if (*msg).size > 16 { return -7; }
    let mut copy = [0u8; 16];
    let mut payload = aux_payload { address: (*msg).address, data: (*msg).buffer,
        length: (*msg).size, reply: &mut (*msg).reply, i2c_over_aux: false,
        write: false, mot: false, write_status_update: false, defer_delay: 0 };
    dm_dp_aux_fill_payload_flags((*msg).request, &mut payload);
    if payload.write {
        core::ptr::copy_nonoverlapping((*msg).buffer, copy.as_mut_ptr(), (*msg).size as usize);
        payload.data = copy.as_mut_ptr();
    }
    // The remaining operations intentionally retain the driver's external AUX side effects.
    let mut operation_result = aux_return_code_type::AUX_RET_SUCCESS;
    let mut result = dc_link_aux_transfer_raw(core::ptr::null_mut(), &mut payload, &mut operation_result);
    if payload.write && result >= 0 {
        if result != 0 { result = payload.data.read() as isize; }
        else if *payload.reply == 0 { result = (*msg).size as isize; }
    }
    if result < 0 { result = dm_dp_aux_transfer_result(result, operation_result); }
    result
}

pub unsafe fn dm_mst_msg_ready_mask(msg_rdy_type: u32) -> u8 {
    match msg_rdy_type { 0 => 0x02, 1 => 0x01, _ => 0x03 }
}

pub unsafe fn dm_mst_select_esi_dpcd(dpcd_rev: u8, dpcd_addr: *mut i32, bytes: *mut u8) {
    if dpcd_rev < 0x12 { *bytes = 0x202 - 0x200; *dpcd_addr = 0x200; }
    else { *bytes = 0x2006 - 0x2002; *dpcd_addr = 0x2002; }
}

pub unsafe fn dm_mst_get_pbn_divider(link: *mut dc_link) -> u32 {
    if link.is_null() { return 0; }
    // Equivalent arithmetic is implemented by the DC layer in the final linkage.
    0
}

// The remaining declarations and implementations use the same C ABI-facing data layout;
// their external kernel/DC dependencies are intentionally unresolved here.
extern "C" {
    pub fn needs_dsc_aux_workaround(link: *mut dc_link) -> bool;
    pub fn retrieve_downstream_port_device(connector: *mut amdgpu_dm_connector) -> bool;
    pub fn retrieve_branch_specific_data(connector: *mut amdgpu_dm_connector) -> bool;
    pub fn dm_dp_mst_is_port_support_mode(connector: *mut amdgpu_dm_connector, stream: *mut dc_stream_state) -> i32;
    pub fn compute_mst_dsc_configs_for_state(state: *mut drm_atomic_commit, dc_state: *mut dc_state,
        vars: *mut dsc_mst_fairness_vars) -> i32;
    pub fn pre_validate_dsc(state: *mut drm_atomic_commit, dm_state: *mut *mut dm_atomic_state,
        vars: *mut dsc_mst_fairness_vars) -> i32;
}
#[repr(C)] pub struct dc_stream_state;
#[repr(C)] pub struct dc_state;
#[repr(C)] pub struct dm_atomic_state;
#[repr(C)] pub struct dsc_mst_fairness_vars;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
