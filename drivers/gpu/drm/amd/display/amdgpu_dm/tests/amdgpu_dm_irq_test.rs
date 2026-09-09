// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_irq.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 *
 * This is a source-level Rust translation of the isolated C implementation.
 * Kernel and DRM definitions are supplied by the surrounding build.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// External kernel/DRM types and functions are intentionally unresolved here;
// they are supplied by the amdgpu/DRM translation units.
#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct dc { _private: [u8; 0] }
#[repr(C)] pub struct dc_link { _private: [u8; 0] }
#[repr(C)] pub struct dc_sink { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct irq_service { _private: [u8; 0] }
#[repr(C)] pub struct dmub_srv { _private: [u8; 0] }
#[repr(C)] pub struct dmub_notification { _private: [u8; 0] }
#[repr(C)] pub struct irq_source_info { _private: [u8; 0] }
#[repr(C)] pub struct resource_pool { _private: [u8; 0] }
#[repr(C)] pub struct drm_connector { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_crtc { _private: [u8; 0] }
#[repr(C)] pub struct hpd_rx_irq_offload_work_queue { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }

pub type bool_t = bool;
pub type irq_handler_t = unsafe extern "C" fn(*mut c_void);
pub type dc_connection_type = u32;
pub type dc_detect_reason = u32;
pub type dc_status = u32;
pub type dc_irq_source = u32;

extern "C" {
    fn amdgpu_dm_hpd_to_dal_irq_source(hpd: u32) -> dc_irq_source;
    fn are_sinks_equal(a: *mut dc_sink, b: *mut dc_sink) -> bool;
    fn dmub_notification_type_str(kind: u32) -> *const u8;
    fn amdgpu_dm_irq_init(adev: *mut amdgpu_device) -> i32;
    fn amdgpu_dm_irq_register_interrupt(adev: *mut amdgpu_device, params: *mut c_void,
                                        handler: Option<irq_handler_t>, data: *mut c_void) -> *mut c_void;
    fn amdgpu_dm_irq_unregister_interrupt(adev: *mut amdgpu_device, source: dc_irq_source,
                                          handler: Option<irq_handler_t>);
}

unsafe extern "C" fn dm_test_irq_handler(_arg: *mut c_void) {}
unsafe extern "C" fn dm_test_irq_handler_alt(_arg: *mut c_void) {}

unsafe extern "C" fn dm_test_irq_handler_count(arg: *mut c_void) {
    if !arg.is_null() {
        *(arg as *mut i32) += 1;
    }
}

unsafe extern "C" fn dm_test_detect_connection_none(
    _link: *mut dc_link, ty: *mut dc_connection_type,
) -> bool {
    *ty = 0; // dc_connection_none
    true
}

unsafe extern "C" fn dm_test_detect_link_false(
    _link: *mut dc_link, _reason: dc_detect_reason,
) -> bool { false }

unsafe extern "C" fn dm_test_detect_connection_single(
    _link: *mut dc_link, ty: *mut dc_connection_type,
) -> bool {
    *ty = 1; // dc_connection_single
    true
}

/* Recording stubs for the dm_handle_hpd_rx_offload_work() DP-IRQ branches. */
static mut dm_test_automated_test_count: i32 = 0;
static mut dm_test_handle_link_loss_count: i32 = 0;

unsafe extern "C" fn dm_test_dp_handle_automated_test(_link: *mut dc_link) {
    dm_test_automated_test_count += 1;
}
unsafe extern "C" fn dm_test_dp_handle_link_loss(_link: *mut dc_link) {
    dm_test_handle_link_loss_count += 1;
}
unsafe extern "C" fn dm_test_dp_parse_link_loss_true(_link: *mut dc_link, _data: *mut c_void) -> bool { true }
unsafe extern "C" fn dm_test_dp_should_allow_hpd_rx_irq_true(_link: *const dc_link) -> bool { true }
unsafe extern "C" fn dm_test_dp_read_hpd_rx_irq_data_ok(_link: *mut dc_link, _data: *mut c_void) -> dc_status { 0 }

unsafe extern "C" fn dm_test_allow_hpd_rx_irq_true(_link: *const dc_link) -> bool { true }

unsafe extern "C" fn dm_test_dmub_get_outbox0_wptr(_dmub: *mut dmub_srv) -> u32 { 0 }
unsafe extern "C" fn dm_test_dmub_get_outbox1_wptr(_dmub: *mut dmub_srv) -> u32 { 0 }
static mut dm_test_dmub_notify_count: i32 = 0;
unsafe extern "C" fn dm_test_dmub_notify_callback(_adev: *mut amdgpu_device, _notify: *mut dmub_notification) {
    dm_test_dmub_notify_count += 1;
}

/* The remaining KUnit entry points are declared as external Rust symbols so
 * the surrounding translated kernel test harness can provide their bodies. */
extern "C" {
    fn dm_test_hpd_to_dal_irq_source_hpd1(test: *mut kunit);
    fn dm_test_hpd_to_dal_irq_source_hpd2(test: *mut kunit);
    fn dm_test_hpd_to_dal_irq_source_hpd3(test: *mut kunit);
    fn dm_test_hpd_to_dal_irq_source_hpd4(test: *mut kunit);
    fn dm_test_hpd_to_dal_irq_source_hpd5(test: *mut kunit);
    fn dm_test_hpd_to_dal_irq_source_hpd6(test: *mut kunit);
    fn dm_test_hpd_to_dal_irq_source_invalid(test: *mut kunit);
    fn dm_test_hpd_to_dal_irq_source_out_of_range(test: *mut kunit);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
