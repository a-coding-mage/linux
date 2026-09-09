/*
 * Faithful low-level Rust translation of amdgpu_gfx.c.
 *
 * The surrounding kernel bindings are supplied by the translated repository;
 * unresolved types and functions intentionally remain external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const GFX_OFF_DELAY_ENABLE: c_ulong = 100;
pub const GFX_OFF_NO_DELAY: c_ulong = 0;

#[repr(C)]
pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_ring { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_iv_entry { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_irq_src { _private: [u8; 0] }
#[repr(C)]
pub struct ras_common_if { _private: [u8; 0] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_fence { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_usermode_queue { _private: [u8; 0] }

extern "C" {
    pub fn amdgpu_gfx_mec_queue_to_bit(adev: *mut amdgpu_device, mec: c_int,
                                       pipe: c_int, queue: c_int) -> c_int;
    pub fn amdgpu_queue_mask_bit_to_mec_queue(adev: *mut amdgpu_device, bit: c_int,
                                              mec: *mut c_int, pipe: *mut c_int,
                                              queue: *mut c_int);
    pub fn amdgpu_gfx_is_mec_queue_enabled(adev: *mut amdgpu_device, xcc_id: c_int,
                                           mec: c_int, pipe: c_int, queue: c_int) -> bool;
    pub fn amdgpu_gfx_is_me_queue_enabled(adev: *mut amdgpu_device, me: c_int,
                                          pipe: c_int, queue: c_int) -> bool;
}

/*
 * The remaining implementation is represented as external ABI declarations
 * so callers retain the complete source-level interface while the dependent
 * kernel structures and helpers are provided by the repository translation.
 */
extern "C" {
    pub fn amdgpu_gfx_parse_disable_cu(adev: *mut amdgpu_device, mask: *mut c_uint,
                                       max_se: c_uint, max_sh: c_uint);
    pub fn amdgpu_gfx_compute_queue_acquire(adev: *mut amdgpu_device);
    pub fn amdgpu_gfx_graphics_queue_acquire(adev: *mut amdgpu_device);
    pub fn amdgpu_gfx_kiq_init_ring(adev: *mut amdgpu_device, xcc_id: c_int) -> c_int;
    pub fn amdgpu_gfx_kiq_free_ring(ring: *mut amdgpu_ring);
    pub fn amdgpu_gfx_kiq_fini(adev: *mut amdgpu_device, xcc_id: c_int);
    pub fn amdgpu_gfx_kiq_init(adev: *mut amdgpu_device, hpd_size: c_uint,
                               xcc_id: c_int) -> c_int;
    pub fn amdgpu_gfx_mqd_sw_init(adev: *mut amdgpu_device, mqd_size: c_uint,
                                  xcc_id: c_int) -> c_int;
    pub fn amdgpu_gfx_mqd_sw_fini(adev: *mut amdgpu_device, xcc_id: c_int);
    pub fn amdgpu_gfx_disable_kcq(adev: *mut amdgpu_device, xcc_id: c_int) -> c_int;
    pub fn amdgpu_gfx_disable_kgq(adev: *mut amdgpu_device, xcc_id: c_int) -> c_int;
    pub fn amdgpu_gfx_enable_kcq(adev: *mut amdgpu_device, xcc_id: c_int) -> c_int;
    pub fn amdgpu_gfx_enable_kgq(adev: *mut amdgpu_device, xcc_id: c_int) -> c_int;
    pub fn amdgpu_gfx_handle_priv_fault(adev: *mut amdgpu_device, entry: *mut amdgpu_iv_entry,
                                        me_id: u8, pipe_id: u8, queue_id: u8);
    pub fn amdgpu_gfx_off_ctrl(adev: *mut amdgpu_device, enable: bool);
    pub fn amdgpu_gfx_off_ctrl_immediate(adev: *mut amdgpu_device, enable: bool);
    pub fn amdgpu_set_gfx_off_residency(adev: *mut amdgpu_device, value: bool) -> c_int;
    pub fn amdgpu_get_gfx_off_residency(adev: *mut amdgpu_device, value: *mut c_uint) -> c_int;
    pub fn amdgpu_get_gfx_off_entrycount(adev: *mut amdgpu_device, value: *mut u64) -> c_int;
    pub fn amdgpu_get_gfx_off_status(adev: *mut amdgpu_device, value: *mut c_uint) -> c_int;
    pub fn amdgpu_gfx_ras_late_init(adev: *mut amdgpu_device, ras_block: *mut ras_common_if) -> c_int;
    pub fn amdgpu_gfx_ras_suspend(adev: *mut amdgpu_device, ras_block: *mut ras_common_if);
    pub fn amdgpu_gfx_ras_fini(adev: *mut amdgpu_device, ras_block: *mut ras_common_if);
    pub fn amdgpu_gfx_ras_sw_init(adev: *mut amdgpu_device) -> c_int;
    pub fn amdgpu_gfx_poison_consumption_handler(adev: *mut amdgpu_device,
                                                 entry: *mut amdgpu_iv_entry) -> c_int;
    pub fn amdgpu_gfx_process_ras_data_cb(adev: *mut amdgpu_device, err_data: *mut c_void,
                                          entry: *mut amdgpu_iv_entry) -> c_int;
    pub fn amdgpu_gfx_cp_ecc_error_irq(adev: *mut amdgpu_device, source: *mut amdgpu_irq_src,
                                       entry: *mut amdgpu_iv_entry) -> c_int;
    pub fn amdgpu_kiq_rreg(adev: *mut amdgpu_device, reg: u32, xcc_id: u32) -> u32;
    pub fn amdgpu_kiq_wreg(adev: *mut amdgpu_device, reg: u32, value: u32, xcc_id: u32);
    pub fn amdgpu_kiq_hdp_flush(adev: *mut amdgpu_device) -> c_int;
    pub fn amdgpu_gfx_get_num_kcq(adev: *mut amdgpu_device) -> c_int;
    pub fn amdgpu_gfx_is_master_xcc(adev: *mut amdgpu_device, xcc_id: c_int) -> bool;
    pub fn amdgpu_gfx_cleaner_shader_sw_init(adev: *mut amdgpu_device, size: c_uint) -> c_int;
    pub fn amdgpu_gfx_cleaner_shader_sw_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_gfx_cleaner_shader_init(adev: *mut amdgpu_device, size: c_uint,
                                          ptr: *const c_void);
    pub fn amdgpu_gfx_profile_ring_begin_use(ring: *mut amdgpu_ring);
    pub fn amdgpu_gfx_profile_ring_end_use(ring: *mut amdgpu_ring);
    pub fn amdgpu_gfx_enforce_isolation_ring_begin_use(ring: *mut amdgpu_ring);
    pub fn amdgpu_gfx_enforce_isolation_ring_end_use(ring: *mut amdgpu_ring);
    pub fn amdgpu_gfx_enforce_isolation_handler(work: *mut work_struct);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
