/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2015 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency intent: DAL IRQ definitions are supplied by irq_types.h.

#[repr(C)] pub struct amdgpu_device { _unused: [u8; 0] }
#[repr(C)] pub struct amdgpu_crtc { _unused: [u8; 0] }
#[repr(C)] pub struct amdgpu_display_manager { _unused: [u8; 0] }
#[repr(C)] pub struct dc_sink { _unused: [u8; 0] }
#[repr(C)] pub struct hpd_rx_irq_offload_work_queue { _unused: [u8; 0] }
#[repr(C)] pub struct work_struct { _unused: [u8; 0] }
#[repr(C)] pub struct amdgpu_irq_src { _unused: [u8; 0] }
#[repr(C)] pub struct amdgpu_iv_entry { _unused: [u8; 0] }
#[repr(C)] pub struct amdgpu_dm_connector { _unused: [u8; 0] }
#[repr(C)] pub struct dmub_notification { _unused: [u8; 0] }
#[repr(C)] pub union hpd_irq_data { _unused: [u8; 0] }

// External enum/type declarations supplied by dependent headers.
pub type dmub_notification_type = core::ffi::c_int;
pub type dc_irq_source = core::ffi::c_int;
pub type dc_detect_reason = core::ffi::c_int;
pub type amdgpu_interrupt_state = core::ffi::c_int;
#[repr(C)] pub struct dc_interrupt_params { _unused: [u8; 0] }

/* Display Manager IRQ-related interfaces (for use by DAL). */

/// amdgpu_dm_irq_init - Initialize internal structures of 'amdgpu_dm_irq'.
///
/// This function should be called exactly once - during DM initialization.
///
/// Returns:
///     0 - success
///     non-zero - error
pub extern "C" fn amdgpu_dm_irq_init(adev: *mut amdgpu_device) -> core::ffi::c_int;

/// amdgpu_dm_irq_fini - deallocate internal structures of 'amdgpu_dm_irq'.
pub extern "C" fn amdgpu_dm_irq_fini(adev: *mut amdgpu_device);

pub extern "C" fn amdgpu_dm_irq_register_interrupt(
    adev: *mut amdgpu_device,
    int_params: *mut dc_interrupt_params,
    ih: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    handler_args: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void;

pub extern "C" fn amdgpu_dm_irq_unregister_interrupt(
    adev: *mut amdgpu_device,
    irq_source: dc_irq_source,
    ih_index: *mut core::ffi::c_void,
);

pub extern "C" fn amdgpu_dm_set_irq_funcs(adev: *mut amdgpu_device);
pub extern "C" fn amdgpu_dm_outbox_init(adev: *mut amdgpu_device);
pub extern "C" fn amdgpu_dm_hpd_init(adev: *mut amdgpu_device);
pub extern "C" fn amdgpu_dm_hpd_fini(adev: *mut amdgpu_device);
pub extern "C" fn amdgpu_dm_irq_suspend(adev: *mut amdgpu_device);
pub extern "C" fn amdgpu_dm_irq_resume_early(adev: *mut amdgpu_device);
pub extern "C" fn amdgpu_dm_irq_resume_late(adev: *mut amdgpu_device);

pub extern "C" fn amdgpu_dm_hpd_rx_irq_create_workqueue(adev: *mut amdgpu_device) -> *mut hpd_rx_irq_offload_work_queue;
pub extern "C" fn amdgpu_dm_hpd_rx_irq_work_suspend(dm: *mut amdgpu_display_manager);
pub extern "C" fn amdgpu_dm_register_hpd_handlers(adev: *mut amdgpu_device) -> core::ffi::c_int;
pub extern "C" fn amdgpu_dm_hdmi_hpd_debounce_work(work: *mut work_struct);

pub extern "C" fn amdgpu_dm_get_crtc_by_otg_inst(adev: *mut amdgpu_device, otg_inst: core::ffi::c_int) -> *mut amdgpu_crtc;
pub extern "C" fn amdgpu_dm_dce110_register_irq_handlers(adev: *mut amdgpu_device) -> core::ffi::c_int;
pub extern "C" fn amdgpu_dm_dcn10_register_irq_handlers(adev: *mut amdgpu_device) -> core::ffi::c_int;
pub extern "C" fn amdgpu_dm_register_outbox_irq_handlers(adev: *mut amdgpu_device) -> core::ffi::c_int;

// Build-time condition preserved from CONFIG_DRM_AMD_DC_KUNIT_TEST.
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn amdgpu_dm_hpd_to_dal_irq_source(type_: u32) -> dc_irq_source;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn are_sinks_equal(sink1: *const dc_sink, sink2: *const dc_sink) -> bool;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn dmub_notification_type_str(e: dmub_notification_type) -> *const core::ffi::c_char;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn amdgpu_dm_set_hpd_irq_state(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, type_: u32, state: amdgpu_interrupt_state) -> core::ffi::c_int;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn amdgpu_dm_set_dmub_outbox_irq_state(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, crtc_id: u32, state: amdgpu_interrupt_state) -> core::ffi::c_int;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn amdgpu_dm_set_dmub_trace_irq_state(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, type_: u32, state: amdgpu_interrupt_state) -> core::ffi::c_int;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn amdgpu_dm_set_pflip_irq_state(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, crtc_id: u32, state: amdgpu_interrupt_state) -> core::ffi::c_int;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn amdgpu_dm_set_crtc_irq_state(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, crtc_id: u32, state: amdgpu_interrupt_state) -> core::ffi::c_int;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn amdgpu_dm_set_vline0_irq_state(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, crtc_id: u32, state: amdgpu_interrupt_state) -> core::ffi::c_int;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn amdgpu_dm_set_vupdate_irq_state(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, crtc_id: u32, state: amdgpu_interrupt_state) -> core::ffi::c_int;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn amdgpu_dm_irq_schedule_work(adev: *mut amdgpu_device, irq_source: dc_irq_source);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn amdgpu_dm_irq_immediate_work(adev: *mut amdgpu_device, irq_source: dc_irq_source);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn dm_handle_hpd_rx_offload_work(work: *mut work_struct);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn handle_hpd_irq_helper(aconnector: *mut amdgpu_dm_connector, reason: dc_detect_reason);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn handle_hpd_irq(param: *mut core::ffi::c_void);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn schedule_hpd_rx_offload_work(adev: *mut amdgpu_device, offload_wq: *mut hpd_rx_irq_offload_work_queue, hpd_irq_data: hpd_irq_data);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn handle_hpd_rx_irq(param: *mut core::ffi::c_void);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn dmub_hpd_callback(adev: *mut amdgpu_device, notify: *mut dmub_notification);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn dmub_hpd_sense_callback(adev: *mut amdgpu_device, notify: *mut dmub_notification);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn dm_pflip_high_irq(interrupt_params: *mut core::ffi::c_void);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn dm_vupdate_high_irq(interrupt_params: *mut core::ffi::c_void);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn dm_crtc_high_irq(interrupt_params: *mut core::ffi::c_void);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn dm_handle_hpd_work(work: *mut work_struct);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn dm_dmub_outbox1_low_irq(interrupt_params: *mut core::ffi::c_void);
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn amdgpu_dm_irq_handler(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> core::ffi::c_int;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub extern "C" fn dm_handle_vmin_vmax_update(offload_work: *mut work_struct);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
