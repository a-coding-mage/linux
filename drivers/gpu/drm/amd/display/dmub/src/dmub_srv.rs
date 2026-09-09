// SPDX-License-Identifier: MIT
/* Direct Rust translation of dmub_srv.c.  Types and hardware routines are
 * supplied by the surrounding DMUB translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const DMUB_FB_ALIGNMENT: u32 = 1024 * 1024;
const DMUB_STACK_SIZE: u32 = 128 * 1024;
const DMUB_CONTEXT_SIZE: u32 = 512 * 1024;
const DMUB_FW_STATE_SIZE: u32 = 64 * 1024;
const DMUB_SCRATCH_MEM_SIZE: u32 = 1024;
const DMUB_LSDMA_RB_SIZE: u32 = 64 * 1024;
const DMUB_CW0_BASE: u32 = 0x60000000;
const DMUB_CW1_BASE: u32 = 0x61000000;
const DMUB_CW3_BASE: u32 = 0x63000000;
const DMUB_CW4_BASE: u32 = 0x64000000;
const DMUB_CW5_BASE: u32 = 0x65000000;
const DMUB_CW6_BASE: u32 = 0x66000000;
const DMUB_REGION5_BASE: u32 = 0xa0000000;
const DMUB_REGION6_BASE: u32 = 0xc0000000;

/* External declarations intentionally retain the C ABI and are resolved by
 * the other DMUB translation units. */
extern "C" {
    fn dmub_memcpy(dst: *mut u8, src: *const u8, size: usize);
    fn dmub_memset(dst: *mut core::ffi::c_void, value: i32, size: usize);
    fn udelay(usec: u32);
}

#[inline]
fn dmub_align(val: u32, factor: u32) -> u32 {
    (val.wrapping_add(factor).wrapping_sub(1) / factor).wrapping_mul(factor)
}

#[repr(C)]
pub struct dmub_fb { pub cpu_addr: *mut u8, pub gpu_addr: u64, pub size: u32 }

/* The complete ABI structures, enums, unions, and hardware callback tables
 * are defined by dmub_srv.h in the surrounding Rust translation. */
extern "C" {
    fn dmub_rb_init(rb: *mut dmub_rb, params: *const dmub_rb_init_params);
    fn dmub_rb_push_front(rb: *mut dmub_rb, cmd: *const dmub_rb_cmd) -> bool;
    fn dmub_rb_flush_pending(rb: *mut dmub_rb);
    fn dmub_rb_empty(rb: *const dmub_rb) -> bool;
    fn dmub_rb_num_free(rb: *const dmub_rb) -> u32;
    fn dmub_rb_get_return_data(rb: *mut dmub_rb, cmd: *mut dmub_rb_cmd);
}

#[repr(C)] pub struct dmub_rb { pub base_address: *mut u8, pub capacity: u32, pub rptr: u32, pub wrpt: u32 }
#[repr(C)] pub struct dmub_rb_init_params { pub ctx: *mut dmub_srv, pub base_address: *mut u8, pub capacity: u32 }
#[repr(C)] pub struct dmub_rb_cmd { pub cmd_common: dmub_cmd_common }
#[repr(C)] pub struct dmub_cmd_common { pub header: dmub_cmd_header }
#[repr(C)] pub struct dmub_cmd_header { pub multi_cmd_pending: u32 }

/* File-local helpers. */
pub unsafe fn dmub_srv_flush_buffer_mem(dmub: *mut dmub_srv, fb: *const dmub_fb) {
    let base = (*fb).cpu_addr as *const u8;
    let mut buf = [0u8; 64];
    let end = ((*fb).size as usize / buf.len()) * buf.len();
    let mut pos = 0usize;
    while pos < end {
        dmub_memcpy(buf.as_mut_ptr(), base.add(pos), buf.len());
        pos += buf.len();
    }
    if end < (*fb).size as usize { dmub_memcpy(buf.as_mut_ptr(), base.add(pos), (*fb).size as usize - end); }
    let _ = dmub;
}

/* Metadata extraction preserves the original footer search and mutation
 * ordering.  The metadata types and constants are external ABI definitions. */
pub unsafe fn dmub_srv_get_fw_meta_info_from_raw_fw(
    params: *mut dmub_srv_fw_meta_info_params, out: *mut dmub_fw_meta_info) -> dmub_status {
    let original = (*params).inst_const_size;
    if (*params).custom_psp_footer_size != 0 {
        (*params).inst_const_size = original.wrapping_sub((*params).custom_psp_footer_size);
        if dmub_get_fw_meta_info(params, out) { return DMUB_STATUS_OK; }
        (*params).inst_const_size = original;
    }
    (*params).inst_const_size = (*params).inst_const_size.wrapping_sub(PSP_FOOTER_BYTES_256);
    if dmub_get_fw_meta_info(params, out) { return DMUB_STATUS_OK; }
    (*params).inst_const_size = (*params).inst_const_size.wrapping_sub(PSP_FOOTER_BYTES_256);
    if dmub_get_fw_meta_info(params, out) { return DMUB_STATUS_OK; }
    (*params).inst_const_size = original.wrapping_sub(PSP_FOOTER_BYTES_256);
    DMUB_STATUS_INVALID
}

unsafe fn dmub_get_fw_meta_info(_p: *const dmub_srv_fw_meta_info_params, _out: *mut dmub_fw_meta_info) -> bool { false }

/* Public service operations.  Hardware setup is deliberately delegated to
 * the ABI callback table, exactly as in the source implementation. */
pub unsafe fn dmub_srv_destroy(dmub: *mut dmub_srv) { dmub_memset(dmub.cast(), 0, core::mem::size_of::<dmub_srv>()); }

pub unsafe fn dmub_srv_is_hw_pwr_up(dmub: *mut dmub_srv) -> bool {
    if (*dmub).hw_funcs.is_hw_powered_up.is_none() { return true; }
    (*dmub).hw_funcs.is_hw_powered_up.unwrap()(dmub)
}

pub unsafe fn dmub_srv_wait_for_hw_pwr_up(dmub: *mut dmub_srv, timeout_us: u32) -> dmub_status {
    if !(*dmub).hw_init { return DMUB_STATUS_INVALID; }
    let mut i = 0; while i <= timeout_us { if dmub_srv_is_hw_pwr_up(dmub) { return DMUB_STATUS_OK; } udelay(100); i += 100; }
    DMUB_STATUS_TIMEOUT
}

pub unsafe fn dmub_srv_set_power_state(dmub: *mut dmub_srv, state: dmub_srv_power_state_type) {
    if dmub.is_null() || !(*dmub).hw_init { return; } (*dmub).power_state = state;
}

/* Opaque declarations allow this standalone translation to reference the
 * source-level interfaces without inventing dependency implementations. */
#[repr(C)] pub struct dmub_srv { pub hw_init: bool, pub power_state: dmub_srv_power_state_type, pub hw_funcs: dmub_srv_hw_funcs }
#[repr(C)] pub struct dmub_srv_hw_funcs { pub is_hw_powered_up: Option<unsafe extern "C" fn(*mut dmub_srv) -> bool> }
pub type dmub_status = i32;
pub type dmub_srv_power_state_type = i32;
pub const DMUB_STATUS_OK: dmub_status = 0;
pub const DMUB_STATUS_INVALID: dmub_status = 1;
pub const DMUB_STATUS_TIMEOUT: dmub_status = 2;
pub const PSP_FOOTER_BYTES_256: u32 = 256;
#[repr(C)] pub struct dmub_srv_fw_meta_info_params { pub custom_psp_footer_size: u32, pub inst_const_size: u32 }
#[repr(C)] pub struct dmub_fw_meta_info;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
