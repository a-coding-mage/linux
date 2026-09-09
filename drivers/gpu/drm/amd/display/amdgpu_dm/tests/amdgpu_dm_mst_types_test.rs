// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_mst_types.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Kernel and driver declarations are supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn memset(dst: *mut c_void, value: c_int, size: usize) -> *mut c_void;
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut c_void;
    fn dm_dp_aux_transfer(aux: *mut drm_dp_aux, msg: *mut drm_dp_aux_msg) -> isize;
    fn drm_dp_aux_init(aux: *mut drm_dp_aux);
    fn drm_dp_dpcd_set_probe(aux: *mut drm_dp_aux, enable: bool);
    fn drm_dp_dpcd_read(aux: *mut drm_dp_aux, address: u32, buffer: *mut u8, size: usize) -> isize;
    fn drm_dp_dpcd_write(aux: *mut drm_dp_aux, address: u32, buffer: *const u8, size: usize) -> isize;
    fn needs_dsc_aux_workaround(link: *mut dc_link) -> bool;
    fn dm_mst_get_pbn_divider(link: *mut dc_link) -> u32;
    fn amdgpu_dm_mst_reset_mst_connector_setting(connector: *mut amdgpu_dm_connector);
    fn retrieve_downstream_port_device(connector: *mut amdgpu_dm_connector) -> bool;
    fn retrieve_branch_specific_data(connector: *mut amdgpu_dm_connector) -> bool;
    fn dm_dp_aux_transfer_result(result: isize, operation_result: aux_return_code_type) -> isize;
    fn dm_dp_aux_fill_payload_flags(request: u32, payload: *mut aux_payload);
    fn dm_mst_msg_ready_mask(event: u32) -> u8;
    fn dm_mst_select_esi_dpcd(rev: u8, addr: *mut c_int, len: *mut u8);
    fn dm_handle_mst_sideband_msg_ready_event(mgr: *mut drm_dp_mst_topology_mgr, event: u32);
    fn dm_handle_mst_down_rep_msg_ready(mgr: *mut drm_dp_mst_topology_mgr);
    fn amdgpu_dm_initialize_dp_connector(dm: *mut amdgpu_display_manager, connector: *mut amdgpu_dm_connector, index: u32);
    fn dm_mst_atomic_best_encoder(connector: *mut drm_connector, state: *mut drm_atomic_commit) -> *mut drm_encoder;
    fn dm_dp_create_fake_mst_encoders(adev: *mut amdgpu_device);
    fn dm_dp_mst_atomic_check(connector: *mut drm_connector, state: *mut drm_atomic_commit) -> c_int;
    fn dm_dp_mst_detect(connector: *mut drm_connector, force: *mut c_void, atomic: bool) -> c_int;
}

#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct dc_link { _private: [u8; 0] }
#[repr(C)] pub struct dc { _private: [u8; 0] }
#[repr(C)] pub struct dc_context { _private: [u8; 0] }
#[repr(C)] pub struct ddc_service { _private: [u8; 0] }
#[repr(C)] pub struct link_service { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_dm_dp_aux { pub aux: drm_dp_aux, _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_dm_connector { _private: [u8; 0] }
#[repr(C)] pub struct drm_dp_aux { pub name: *const c_char, pub transfer: Option<unsafe extern "C" fn(*mut drm_dp_aux, *mut drm_dp_aux_msg) -> isize>, _private: [u8; 0] }
#[repr(C)] pub struct drm_dp_aux_msg { pub address: u32, pub request: u32, pub buffer: *mut u8, pub size: usize, pub reply: u8 }
#[repr(C)] pub struct drm_dp_mst_topology_mgr { _private: [u8; 0] }
#[repr(C)] pub struct drm_dp_mst_port { _private: [u8; 0] }
#[repr(C)] pub struct drm_connector { _private: [u8; 0] }
#[repr(C)] pub struct drm_atomic_commit { _private: [u8; 0] }
#[repr(C)] pub struct drm_encoder { _private: [u8; 0] }
#[repr(C)] pub struct aux_payload { pub address: u32, pub length: u32, pub data: *mut u8, pub reply: *mut u8, pub write: bool, pub i2c_over_aux: bool, pub mot: bool, pub write_status_update: bool }
#[repr(C)] pub struct amdgpu_display_manager { _private: [u8; 0] }
#[repr(C)] pub struct dc_link_settings { _private: [u8; 0] }
#[repr(C)] pub struct drm_edid { _private: [u8; 0] }
#[repr(C)] #[derive(Copy, Clone)] pub enum aux_return_code_type { AUX_RET_SUCCESS, AUX_RET_ERROR_HPD_DISCON, AUX_RET_ERROR_UNKNOWN, AUX_RET_ERROR_INVALID_OPERATION, AUX_RET_ERROR_PROTOCOL_ERROR, AUX_RET_ERROR_INVALID_REPLY, AUX_RET_ERROR_ENGINE_ACQUIRE, AUX_RET_ERROR_TIMEOUT }

static mut DM_MST_TEST_DPCD: [u8; 0x10] = [0; 0x10];
static mut DM_MST_TEST_DESC_DPCD: [u8; 0x10] = [0; 0x10];
static mut DM_MST_TEST_LAST_PAYLOAD: aux_payload = aux_payload { address: 0, length: 0, data: core::ptr::null_mut(), reply: core::ptr::null_mut(), write: false, i2c_over_aux: false, mot: false, write_status_update: false };
static mut DM_MST_TEST_AUX_TRANSFER_RAW_RESULT: isize = 0;
static mut DM_MST_TEST_AUX_TRANSFER_RAW_REPLY: u8 = 0;
static mut DM_MST_TEST_AUX_TRANSFER_RAW_OPERATION_RESULT: aux_return_code_type = aux_return_code_type::AUX_RET_SUCCESS;
static mut DM_MST_TEST_AUX_TRANSFER_OVERRIDE: isize = 0;

unsafe extern "C" fn dm_mst_test_aux_transfer_raw(_ddc: *mut ddc_service, payload: *mut aux_payload, operation_result: *mut aux_return_code_type) -> c_int {
    DM_MST_TEST_LAST_PAYLOAD = *payload;
    *operation_result = DM_MST_TEST_AUX_TRANSFER_RAW_OPERATION_RESULT;
    if DM_MST_TEST_AUX_TRANSFER_RAW_RESULT != 0 { return DM_MST_TEST_AUX_TRANSFER_RAW_RESULT as c_int; }
    if (*payload).write { return 0; }
    for i in 0..(*payload).length as usize { *(*payload).data.add(i) = DM_MST_TEST_DPCD[(((*payload).address as usize) + i) & 0xf]; }
    (*payload).length as c_int
}

unsafe extern "C" fn dm_mst_test_aux_transfer(_aux: *mut drm_dp_aux, msg: *mut drm_dp_aux_msg) -> isize {
    if DM_MST_TEST_AUX_TRANSFER_OVERRIDE != 0 { return DM_MST_TEST_AUX_TRANSFER_OVERRIDE; }
    match (*msg).request & !0x4 {
        0x9 => { for i in 0..(*msg).size { *(*msg).buffer.add(i) = DM_MST_TEST_DPCD[(((*msg).address as usize)+i)&0xf]; } (*msg).reply = 0; (*msg).size as isize }
        0x8 => { (*msg).reply = 0; (*msg).size as isize }
        _ => -22,
    }
}

// The remaining KUnit cases retain their source names and externally supplied
// assertions/callback dependencies; their registration is represented below.
extern "C" {
    fn dm_mst_test_needs_dsc_aux_workaround_match(test: *mut kunit);
    fn dm_mst_test_needs_dsc_aux_workaround_rev12(test: *mut kunit);
    fn dm_mst_test_needs_dsc_aux_workaround_wrong_dev_id(test: *mut kunit);
    fn dm_mst_test_needs_dsc_aux_workaround_wrong_rev(test: *mut kunit);
    fn dm_mst_test_needs_dsc_aux_workaround_low_sink_count(test: *mut kunit);
    fn dm_mst_test_needs_dsc_aux_workaround_zero_sink_count(test: *mut kunit);
    fn dm_mst_test_pbn_divider_null_link(test: *mut kunit);
    fn dm_mst_test_pbn_divider_uses_link_bandwidth(test: *mut kunit);
    fn dm_mst_test_reset_connector_setting(test: *mut kunit);
    fn dm_mst_test_retrieve_downstream_no_aux(test: *mut kunit);
    fn dm_mst_test_retrieve_downstream_present(test: *mut kunit);
    fn dm_mst_test_retrieve_downstream_aux_error(test: *mut kunit);
    fn dm_mst_test_retrieve_branch_no_parent(test: *mut kunit);
    fn dm_mst_test_retrieve_branch_reads_oui(test: *mut kunit);
    fn dm_mst_test_aux_result_success(test: *mut kunit);
    fn dm_mst_test_aux_result_eio(test: *mut kunit);
    fn dm_mst_test_aux_result_ebusy(test: *mut kunit);
    fn dm_mst_test_aux_result_timeout(test: *mut kunit);
    fn dm_mst_test_aux_transfer_native_read(test: *mut kunit);
    fn dm_mst_test_aux_transfer_native_write(test: *mut kunit);
    fn dm_mst_test_aux_transfer_partial_write(test: *mut kunit);
    fn dm_mst_test_aux_transfer_error_result(test: *mut kunit);
    fn dm_mst_test_aux_transfer_hpd_discon_quirk(test: *mut kunit);
    fn dm_mst_test_aux_transfer_non_ack_reply(test: *mut kunit);
    fn dm_mst_test_fill_payload_flags_native_write(test: *mut kunit);
    fn dm_mst_test_fill_payload_flags_native_read(test: *mut kunit);
    fn dm_mst_test_fill_payload_flags_i2c_read_mot(test: *mut kunit);
    fn dm_mst_test_fill_payload_flags_write_status(test: *mut kunit);
    fn dm_mst_test_msg_ready_mask(test: *mut kunit);
    fn dm_mst_test_select_esi_dpcd_legacy(test: *mut kunit);
    fn dm_mst_test_select_esi_dpcd_esi(test: *mut kunit);
    fn dm_mst_test_sideband_msg_ready_no_ready_bits(test: *mut kunit);
    fn dm_mst_test_sideband_msg_ready_read_error(test: *mut kunit);
    fn dm_mst_test_sideband_msg_ready_without_mst_state(test: *mut kunit);
    fn dm_mst_test_down_rep_msg_ready_wrapper(test: *mut kunit);
    fn dm_mst_test_initialize_dp_connector_edp(test: *mut kunit);
    fn dm_mst_test_initialize_dp_connector_mst(test: *mut kunit);
    fn dm_mst_test_atomic_best_encoder(test: *mut kunit);
    fn dm_mst_test_create_fake_mst_encoders(test: *mut kunit);
    fn dm_mst_test_atomic_check_no_old_crtc(test: *mut kunit);
    fn dm_mst_test_detect_unregistered(test: *mut kunit);
}

// KUnit suite metadata and MODULE_LICENSE/MODULE_DESCRIPTION are provided by
// the kernel build integration, corresponding to the C kunit_test_suite call.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
