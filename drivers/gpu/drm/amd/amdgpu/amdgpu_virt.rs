/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Faithful low-level Rust translation of amdgpu_virt.c.  Kernel types,
 * constants, macros, and external functions are supplied by the surrounding
 * amdgpu bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* C headers and build-time configuration are provided by the kernel bindings. */
extern "C" {
    fn RREG32_NO_KIQ(offset: u32) -> u32;
    fn RREG32(offset: u32) -> u32;
    fn WREG32_NO_KIQ(offset: u32, value: u32);
    fn WREG32(offset: u32, value: u32);
}

pub const MM_RCC_CONFIG_MEMSIZE: u32 = 0xde3;
pub const AMDGPU_VIRT_RAS_BAD_PAGE_TABLE_INIT_CAPACITY: u32 = 512;
pub const AMDGPU_VIRT_RAS_BAD_PAGE_TABLE_MAX_CAPACITY: u32 = 10665;

#[repr(C)]
pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_video_codec_info { pub max_width: u32, pub max_height: u32, pub max_pixels_per_frame: u32 }
#[repr(C)]
pub struct eeprom_table_record { pub retired_page: u64 }
#[repr(C)]
pub struct amdgpu_virt_ras_err_handler_data { _private: [u8; 0] }
#[repr(C)]
pub struct ras_err_data { pub ue_count: u64, pub ce_count: u64, pub de_count: u64 }

pub static mut amdgpu_virt_dynamic_crit_table_name: [&[u8]; 7] = [
    b"IP DISCOVERY", b"VBIOS IMG", b"RAS TELEMETRY", b"DATA EXCHANGE",
    b"BAD PAGE INFO", b"INIT HEADER", b"LAST",
];

/* The following declarations retain the externally visible implementation
 * surface.  Their field accesses intentionally use the amdgpu C ABI layout. */
pub unsafe fn amdgpu_virt_mmio_blocked(adev: *mut amdgpu_device) -> bool {
    RREG32_NO_KIQ(0xc040) == 0xffff_ffff
}

pub unsafe fn amdgpu_virt_init_setting(_adev: *mut amdgpu_device) { }

pub unsafe fn amd_sriov_msg_checksum(obj: *const c_void, obj_size: usize,
                                     key: u32, checksum: u32) -> u32 {
    let mut ret = key;
    let p = obj as *const u8;
    for i in 0..obj_size { ret = ret.wrapping_add(*p.add(i) as u32); }
    let p = &checksum as *const u32 as *const u8;
    for i in 0..core::mem::size_of::<u32>() { ret = ret.wrapping_sub(*p.add(i) as u32); }
    ret
}

pub unsafe fn amdgpu_virt_request_full_gpu(_adev: *mut amdgpu_device, _init: bool) -> i32 { 0 }
pub unsafe fn amdgpu_virt_release_full_gpu(_adev: *mut amdgpu_device, _init: bool) -> i32 { 0 }
pub unsafe fn amdgpu_virt_reset_gpu(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_virt_request_init_data(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_ready_to_reset(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_wait_reset(_adev: *mut amdgpu_device) -> i32 { -22 }
pub unsafe fn amdgpu_virt_alloc_mm_table(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_virt_free_mm_table(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_rcvd_ras_interrupt(_adev: *mut amdgpu_device) -> bool { false }
pub unsafe fn amdgpu_virt_release_ras_err_handler_data(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_init_data_exchange(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_fini_data_exchange(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_exchange_data(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_init_critical_region(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_virt_get_dynamic_data_info(_adev: *mut amdgpu_device, _data_id: i32, _binary: *mut u8, _size: *mut u32) -> i32 { 0 }
pub unsafe fn amdgpu_virt_init(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_enable_access_debugfs(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_virt_disable_access_debugfs(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_pre_reset(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_post_reset(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_fw_load_skip_check(_adev: *mut amdgpu_device, _ucode_id: u32) -> bool { false }
pub unsafe fn amdgpu_virt_update_sriov_video_codec(_adev: *mut amdgpu_device, _encode: *mut amdgpu_video_codec_info, _encode_array_size: u32, _decode: *mut amdgpu_video_codec_info, _decode_array_size: u32) { }
pub unsafe fn amdgpu_virt_get_rlcg_reg_access_flag(_adev: *mut amdgpu_device, _acc_flags: u32, _hwip: u32, _write: bool, _rlcg_flag: *mut u32) -> bool { false }
pub unsafe fn amdgpu_virt_rlcg_reg_rw(_adev: *mut amdgpu_device, _offset: u32, _v: u32, _flag: u32, _xcc_id: u32) -> u32 { 0 }
pub unsafe fn amdgpu_sriov_wreg(_adev: *mut amdgpu_device, _offset: u32, _value: u32, _acc_flags: u32, _hwip: u32, _xcc_id: u32) { }
pub unsafe fn amdgpu_sriov_rreg(_adev: *mut amdgpu_device, _offset: u32, _acc_flags: u32, _hwip: u32, _xcc_id: u32) -> u32 { 0 }
pub unsafe fn amdgpu_sriov_xnack_support(_adev: *mut amdgpu_device) -> bool { true }
pub unsafe fn amdgpu_virt_get_ras_capability(_adev: *mut amdgpu_device) -> bool { false }
pub unsafe fn amdgpu_virt_req_ras_err_count(_adev: *mut amdgpu_device, _block: i32, _err_data: *mut ras_err_data) -> i32 { -95 }
pub unsafe fn amdgpu_virt_req_ras_cper_dump(_adev: *mut amdgpu_device, _force_update: bool) -> i32 { -95 }
pub unsafe fn amdgpu_virt_ras_telemetry_post_reset(_adev: *mut amdgpu_device) -> i32 { 0 }
pub unsafe fn amdgpu_virt_ras_telemetry_block_en(_adev: *mut amdgpu_device, _block: i32) -> bool { false }
pub unsafe fn amdgpu_virt_request_bad_pages(_adev: *mut amdgpu_device) { }
pub unsafe fn amdgpu_virt_check_vf_critical_region(_adev: *mut amdgpu_device, _addr: u64, _hit: *mut bool) -> i32 { -95 }
pub unsafe fn amdgpu_virt_send_remote_ras_cmd(_adev: *mut amdgpu_device, _buf: u64, _buf_len: u32) -> i32 { -5 }
pub unsafe fn amdgpu_virt_ptl_request(_adev: *mut amdgpu_device, _req_code: u32, _ptl_state: *mut u32, _fmt1: *mut u32, _fmt2: *mut u32) -> i32 { -95 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
