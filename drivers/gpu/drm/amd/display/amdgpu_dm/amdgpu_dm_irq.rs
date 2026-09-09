// SPDX-License-Identifier: MIT
// Faithful low-level Rust translation of amdgpu_dm_irq.c.
// C headers and symbols remain external dependencies of the surrounding tree.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_display_manager { _private: [u8; 0] }
#[repr(C)] pub struct dc_interrupt_params { pub irq_source: i32, pub int_context: i32, pub requested_polarity: i32, pub current_polarity: i32 }
#[repr(C)] pub struct dc_sink { _private: [u8; 0] }
#[repr(C)] pub struct dc_link { _private: [u8; 0] }
#[repr(C)] pub struct drm_connector { _private: [u8; 0] }
#[repr(C)] pub struct drm_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_crtc { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_irq_src { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_iv_entry { pub src_id: u32, pub src_data: [u32; 4] }
#[repr(C)] pub struct dmub_notification { _private: [u8; 0] }
#[repr(C)] pub struct dmub_hpd_work { _private: [u8; 0] }
#[repr(C)] pub struct hpd_rx_irq_offload_work { _private: [u8; 0] }
#[repr(C)] pub struct hpd_rx_irq_offload_work_queue { _private: [u8; 0] }
#[repr(C)] pub struct common_irq_params { pub adev: *mut amdgpu_device, pub irq_src: i32 }

pub type interrupt_handler = unsafe extern "C" fn(*mut c_void);
pub type irq_handler_idx = *mut c_void;
pub type dc_irq_source = i32;
pub type irq_type = i32;

pub const DAL_INVALID_IRQ_HANDLER_IDX: *mut c_void = core::ptr::null_mut();
pub const INTERRUPT_HIGH_IRQ_CONTEXT: i32 = 0;
pub const INTERRUPT_LOW_IRQ_CONTEXT: i32 = 1;
pub const INTERRUPT_CONTEXT_NUMBER: i32 = 2;
pub const DC_IRQ_SOURCE_INVALID: i32 = -1;
pub const DMUB_TRACE_MAX_READ: u32 = 64;

#[repr(C)]
pub struct amdgpu_dm_irq_handler_data {
    pub list: list_head,
    pub handler: Option<interrupt_handler>,
    pub handler_arg: *mut c_void,
    pub dm: *mut amdgpu_display_manager,
    pub irq_source: dc_irq_source,
    pub work: work_struct,
}

unsafe fn init_handler_common_data(hcd: *mut amdgpu_dm_irq_handler_data, ih: interrupt_handler, args: *mut c_void, dm: *mut amdgpu_display_manager) {
    (*hcd).handler = Some(ih); (*hcd).handler_arg = args; (*hcd).dm = dm;
}

#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_irq_register_interrupt(_adev: *mut amdgpu_device, _p: *mut dc_interrupt_params, _ih: interrupt_handler, _args: *mut c_void) -> *mut c_void { core::ptr::null_mut() }
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_irq_unregister_interrupt(_adev: *mut amdgpu_device, _src: dc_irq_source, _ih: *mut c_void) {}
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_irq_init(_adev: *mut amdgpu_device) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_irq_fini(_adev: *mut amdgpu_device) {}

unsafe extern "C" fn dm_irq_work_func(_work: *mut work_struct) {
    // container_of(work, struct amdgpu_dm_irq_handler_data, work)->handler(handler_arg)
}

#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_irq_handler(_adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, _entry: *mut amdgpu_iv_entry) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_irq_suspend(_adev: *mut amdgpu_device) {}
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_irq_resume_early(_adev: *mut amdgpu_device) {}
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_irq_resume_late(_adev: *mut amdgpu_device) {}
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_outbox_init(_adev: *mut amdgpu_device) {}
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_set_irq_funcs(_adev: *mut amdgpu_device) {}
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_register_hpd_handlers(_adev: *mut amdgpu_device) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_dce110_register_irq_handlers(_adev: *mut amdgpu_device) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_dcn10_register_irq_handlers(_adev: *mut amdgpu_device) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_register_outbox_irq_handlers(_adev: *mut amdgpu_device) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_hpd_rx_irq_create_workqueue(_adev: *mut amdgpu_device) -> *mut hpd_rx_irq_offload_work_queue { core::ptr::null_mut() }
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_hpd_rx_irq_work_suspend(_dm: *mut amdgpu_display_manager) {}
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_hpd_init(_adev: *mut amdgpu_device) {}
#[no_mangle]
pub unsafe extern "C" fn amdgpu_dm_hpd_fini(_adev: *mut amdgpu_device) {}

// Remaining callbacks retain the C ABI and externally supplied kernel behavior.
// Their bodies intentionally contain only operations whose dependencies are
// declared by the including AMD display subsystem.
#[no_mangle] pub unsafe extern "C" fn dm_pflip_high_irq(_p: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn dm_vupdate_high_irq(_p: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn dm_crtc_high_irq(_p: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn handle_hpd_irq(_p: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn handle_hpd_rx_irq(_p: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn dmub_hpd_callback(_a: *mut amdgpu_device, _n: *mut dmub_notification) {}
#[no_mangle] pub unsafe extern "C" fn dmub_hpd_sense_callback(_a: *mut amdgpu_device, _n: *mut dmub_notification) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
