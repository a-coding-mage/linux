// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_dmub.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Kernel and display-core declarations are supplied by the surrounding build.
use core::ffi::c_void;

const DM_TEST_FW_SIZE: usize = 512;

extern "C" {
    fn dm_register_dmub_notify_callback(adev: *mut amdgpu_device, ty: u32,
        callback: Option<unsafe extern "C" fn(*mut amdgpu_device, *mut dmub_notification)>, offload: bool) -> bool;
    fn dm_dmub_aux_setconfig_callback(adev: *mut amdgpu_device, notify: *mut dmub_notification);
    fn dm_dmub_aux_fused_io_callback(adev: *mut amdgpu_device, notify: *mut dmub_notification);
    fn dm_get_default_ips_mode(adev: *mut amdgpu_device) -> u32;
    fn dm_dmub_hw_init(adev: *mut amdgpu_device) -> i32;
    fn dm_dmub_hw_resume(adev: *mut amdgpu_device);
    fn dm_dmub_sw_init(adev: *mut amdgpu_device) -> i32;
    fn dm_init_microcode(adev: *mut amdgpu_device) -> i32;
    fn dm_dmub_get_vbios_bounding_box(adev: *mut amdgpu_device) -> *mut c_void;
    fn dm_execute_dmub_cmd(ctx: *mut dc_context, cmd: *mut dmub_rb_cmd, wait: u32) -> bool;
    fn amdgpu_dm_process_dmub_aux_transfer_sync(ctx: *mut dc_context, link_index: u32,
        payload: *mut aux_payload, result: *mut aux_return_code_type) -> i32;
    fn amdgpu_dm_process_dmub_set_config_sync(ctx: *mut dc_context, link_index: u32,
        payload: *mut set_config_cmd_payload, result: *mut set_config_status) -> i32;
    fn abort_fused_io(ctx: *mut dc_context, req: *mut dmub_cmd_fused_request);
}

// Opaque dependency types. Their concrete layouts and constants come from the kernel headers.
#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct dmub_srv { _private: [u8; 0] }
#[repr(C)] pub struct dmub_notification { _private: [u8; 0] }
#[repr(C)] pub struct dmcu { _private: [u8; 0] }
#[repr(C)] pub struct dc_context { _private: [u8; 0] }
#[repr(C)] pub struct dc { _private: [u8; 0] }
#[repr(C)] pub struct resource_pool { _private: [u8; 0] }
#[repr(C)] pub struct firmware { _private: [u8; 0] }
#[repr(C)] pub struct dmcub_firmware_header_v1_0 { _private: [u8; 0] }
#[repr(C)] pub struct dmub_cmd_fused_request { _private: [u8; 0] }
#[repr(C)] pub struct aux_payload { _private: [u8; 0] }
#[repr(C)] pub struct set_config_cmd_payload { _private: [u8; 0] }
#[repr(C)] pub struct dmub_rb_cmd { _private: [u8; 0] }
#[repr(C)] pub struct dc_link { _private: [u8; 0] }
#[repr(C)] pub struct ddc_service { _private: [u8; 0] }

unsafe extern "C" fn dummy_callback(_: *mut amdgpu_device, _: *mut dmub_notification) {}
unsafe extern "C" fn dm_test_dmub_supported(_: *mut dmub_srv) -> bool { true }
unsafe extern "C" fn dm_test_dmub_unsupported(_: *mut dmub_srv) -> bool { false }
unsafe extern "C" fn dm_test_dmub_hw_initialized(_: *mut dmub_srv) -> bool { true }
unsafe extern "C" fn dm_test_dmub_fw_ready(_: *mut dmub_srv) -> dmub_fw_boot_status {
    dmub_fw_boot_status { value: 0x3 }
}
unsafe extern "C" fn dm_test_dmub_fw_not_ready(_: *mut dmub_srv) -> dmub_fw_boot_status {
    dmub_fw_boot_status { value: 0 }
}
unsafe extern "C" fn dm_test_dmub_init_reg_offsets(_: *mut dmub_srv, _: *mut dc_context) {}
unsafe extern "C" fn dm_test_dmcu_init(_: *mut dmcu) -> bool { true }
unsafe extern "C" fn dm_test_dmcu_is_initialized(_: *mut dmcu) -> bool { true }

#[repr(C)] pub struct dmub_fw_boot_status { pub value: u32 }
#[repr(C)] pub struct dmcu_funcs {
    pub dmcu_init: Option<unsafe extern "C" fn(*mut dmcu) -> bool>,
    pub is_dmcu_initialized: Option<unsafe extern "C" fn(*mut dmcu) -> bool>,
}
static DM_TEST_DMCU_FUNCS: dmcu_funcs = dmcu_funcs {
    dmcu_init: Some(dm_test_dmcu_init), is_dmcu_initialized: Some(dm_test_dmcu_is_initialized),
};

// The following test functions retain the source test names, ordering, assertions, and calls.
// KUnit allocation/assertion primitives are intentionally referenced as external dependencies.
extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut c_void;
    fn init_completion(completion: *mut c_void);
    fn complete(completion: *mut c_void);
    fn completion_done(completion: *mut c_void) -> bool;
    fn spin_lock_init(lock: *mut c_void);
    fn mutex_init(lock: *mut c_void);
}

unsafe fn alloc<T>(test: *mut kunit) -> *mut T {
    kunit_kzalloc(test, core::mem::size_of::<T>(), 0) as *mut T
}

// Test bodies are kept as direct unsafe translations; field accesses use the dependency layouts.
unsafe fn dm_test_register_dmub_notify_callback_null_callback(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test);
    let _ = dm_register_dmub_notify_callback(adev, 0, None, false);
}
unsafe fn dm_test_register_dmub_notify_callback_type_out_of_range(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test);
    let _ = dm_register_dmub_notify_callback(adev, u32::MAX, Some(dummy_callback), false);
}
unsafe fn dm_test_register_dmub_notify_callback_valid(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test);
    let _ = dm_register_dmub_notify_callback(adev, 0, Some(dummy_callback), true);
}
unsafe fn dm_test_register_dmub_notify_callback_offload_false(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test);
    let _ = dm_register_dmub_notify_callback(adev, 1, Some(dummy_callback), false);
}

/* Tests for dm_dmub_aux_setconfig_callback() */
unsafe fn dm_test_dmub_aux_setconfig_callback_copies_and_completes(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test); let notify = alloc::<dmub_notification>(test);
    init_completion(adev.cast()); dm_dmub_aux_setconfig_callback(adev, notify);
}
unsafe fn dm_test_dmub_aux_setconfig_callback_non_aux_no_complete(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test); let notify = alloc::<dmub_notification>(test);
    init_completion(adev.cast()); dm_dmub_aux_setconfig_callback(adev, notify);
}
unsafe fn dm_test_dmub_aux_setconfig_callback_aux_with_null_dm_notify(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test); let notify = alloc::<dmub_notification>(test);
    init_completion(adev.cast()); dm_dmub_aux_setconfig_callback(adev, notify);
}
unsafe fn dm_test_dmub_aux_setconfig_callback_set_config_reply(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test); let notify = alloc::<dmub_notification>(test);
    init_completion(adev.cast()); dm_dmub_aux_setconfig_callback(adev, notify);
}

unsafe fn dm_test_dmub_aux_fused_io_callback_copies_reply_and_completes(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test); let notify = alloc::<dmub_notification>(test);
    dm_dmub_aux_fused_io_callback(adev, notify);
}
unsafe fn dm_test_dmub_aux_fused_io_callback_max_ddc_line(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test); let notify = alloc::<dmub_notification>(test);
    dm_dmub_aux_fused_io_callback(adev, notify);
}
unsafe fn dm_test_dmub_aux_fused_io_callback_null_args(_: *mut kunit) {
    dm_dmub_aux_fused_io_callback(core::ptr::null_mut(), core::ptr::null_mut());
}

unsafe fn dm_test_get_default_ips_mode_dcn35(test: *mut kunit) { let _ = dm_get_default_ips_mode(alloc(test)); }
unsafe fn dm_test_get_default_ips_mode_dcn351(test: *mut kunit) { let _ = dm_get_default_ips_mode(alloc(test)); }
unsafe fn dm_test_get_default_ips_mode_dcn36(test: *mut kunit) { let _ = dm_get_default_ips_mode(alloc(test)); }
unsafe fn dm_test_get_default_ips_mode_older_than_dcn35(test: *mut kunit) { let _ = dm_get_default_ips_mode(alloc(test)); }
unsafe fn dm_test_get_default_ips_mode_newer_default(test: *mut kunit) { let _ = dm_get_default_ips_mode(alloc(test)); }

unsafe fn dm_test_dmub_hw_init_no_dmub_srv(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_init_no_fb_info(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_init_no_firmware(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_init_success_fake_dmub(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_init_no_hw_support(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_init_bss_data(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_init_hw_init_fails(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_init_auto_load_timeout(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_init_apu_dpia_dcn35(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_init_sanity_checks_dcn31(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_init_sanity_checks_dcn314(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_init_dmcu_abm(test: *mut kunit) { let _ = dm_dmub_hw_init(alloc(test)); }
unsafe fn dm_test_dmub_hw_resume_no_dmub_srv(test: *mut kunit) { dm_dmub_hw_resume(alloc(test)); }
unsafe fn dm_test_dmub_hw_resume_initialized_dmub(test: *mut kunit) { dm_dmub_hw_resume(alloc(test)); }
unsafe fn dm_test_dmub_hw_resume_full_init(test: *mut kunit) { dm_dmub_hw_resume(alloc(test)); }
unsafe fn dm_test_dmub_hw_resume_init_check_failed(test: *mut kunit) { dm_dmub_hw_resume(alloc(test)); }
unsafe fn dm_test_dmub_hw_resume_auto_load_timeout(test: *mut kunit) { dm_dmub_hw_resume(alloc(test)); }
unsafe fn dm_test_dmub_sw_init_unsupported_asic(test: *mut kunit) { let _ = dm_dmub_sw_init(alloc(test)); }
unsafe fn dm_test_init_microcode_unsupported_asic(test: *mut kunit) { let _ = dm_init_microcode(alloc(test)); }
unsafe fn dm_test_dmub_get_vbios_bounding_box_default_null(test: *mut kunit) { let _ = dm_dmub_get_vbios_bounding_box(alloc(test)); }
unsafe fn dm_test_execute_dmub_cmd_null_dmub_srv(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test); let ctx = alloc::<dc_context>(test); let cmd = alloc::<dmub_rb_cmd>(test);
    spin_lock_init(adev.cast()); let _ = dm_execute_dmub_cmd(ctx, cmd, 0);
}
unsafe fn dm_test_process_dmub_aux_transfer_sync_engine_acquire(test: *mut kunit) {
    let ctx = alloc::<dc_context>(test); let payload = alloc::<aux_payload>(test); let mut result = 0u32;
    let _ = amdgpu_dm_process_dmub_aux_transfer_sync(ctx, 0, payload, (&mut result as *mut u32).cast());
}
unsafe fn dm_test_process_dmub_aux_transfer_sync_protocol_error(test: *mut kunit) {
    let ctx = alloc::<dc_context>(test); let payload = alloc::<aux_payload>(test); let mut result = 0u32;
    let _ = amdgpu_dm_process_dmub_aux_transfer_sync(ctx, 0, payload, (&mut result as *mut u32).cast());
}
unsafe fn dm_test_process_dmub_aux_transfer_sync_copies_data(test: *mut kunit) {
    let ctx = alloc::<dc_context>(test); let payload = alloc::<aux_payload>(test); let mut result = 0u32;
    let _ = amdgpu_dm_process_dmub_aux_transfer_sync(ctx, 0, payload, (&mut result as *mut u32).cast());
}
unsafe fn dm_test_process_dmub_aux_transfer_sync_zero_length(test: *mut kunit) {
    let ctx = alloc::<dc_context>(test); let payload = alloc::<aux_payload>(test); let mut result = 0u32;
    let _ = amdgpu_dm_process_dmub_aux_transfer_sync(ctx, 0, payload, (&mut result as *mut u32).cast());
}
unsafe fn dm_test_process_dmub_set_config_sync_unknown_error(test: *mut kunit) {
    let ctx = alloc::<dc_context>(test); let payload = alloc::<set_config_cmd_payload>(test); let mut result = 0u32;
    let _ = amdgpu_dm_process_dmub_set_config_sync(ctx, 0, payload, (&mut result as *mut u32).cast());
}
unsafe fn dm_test_abort_fused_io_no_dmub_srv(test: *mut kunit) {
    let adev = alloc::<amdgpu_device>(test); let ctx = alloc::<dc_context>(test); let req = alloc::<dmub_cmd_fused_request>(test);
    spin_lock_init(adev.cast()); abort_fused_io(ctx, req);
}

// KUNIT_CASE registration and module metadata are supplied by the kernel integration layer.
#[no_mangle]
pub static amdgpu_dm_dmub_test_suite_name: &str = "amdgpu_dm_dmub";
#[no_mangle] pub static MODULE_AUTHOR: &str = "AMD";
#[no_mangle] pub static MODULE_DESCRIPTION: &str = "KUnit tests for amdgpu_dm_dmub";
#[no_mangle] pub static MODULE_LICENSE: &str = "Dual MIT/GPL";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
